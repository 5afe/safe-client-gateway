use crate::common::models::addresses::AddressEx;
use crate::common::models::data_decoded::{
    DataDecoded, InternalTransaction, Operation, ParamValue, Parameter, ValueDecodedType,
};
use crate::config::feature_flag_nested_decoding;
use crate::providers::ext::InfoProviderExt;
use crate::providers::info::InfoProvider;
use crate::routes::transactions::models::SettingsInfo;
use crate::utils::{
    ADD_OWNER_WITH_THRESHOLD, CHANGE_MASTER_COPY, CHANGE_THRESHOLD, DISABLE_MODULE, ENABLE_MODULE,
    MULTI_SEND, MULTI_SEND_TRANSACTIONS_PARAM, REMOVE_OWNER, SET_FALLBACK_HANDLER, SET_GUARD,
    SWAP_OWNER,
};
use std::collections::HashMap;

impl DataDecoded {
    pub(crate) async fn to_settings_info(
        &self,
        info_provider: &(impl InfoProvider + Sync),
    ) -> Option<SettingsInfo> {
        match self.method.as_str() {
            SET_FALLBACK_HANDLER => {
                let handler = self.get_parameter_single_value_at(0)?;
                Some(SettingsInfo::SetFallbackHandler {
                    handler: info_provider
                        .address_ex_from_contracts_or_default(&handler)
                        .await,
                })
            }
            ADD_OWNER_WITH_THRESHOLD => {
                let owner = self.get_parameter_single_value_at(0)?;
                Some(SettingsInfo::AddOwner {
                    owner: AddressEx::address_only(&owner),
                    threshold: self.get_parameter_single_value_at(1)?.parse().ok()?,
                })
            }
            REMOVE_OWNER => {
                let owner = self.get_parameter_single_value_at(1)?;
                Some(SettingsInfo::RemoveOwner {
                    owner: AddressEx::address_only(&owner),
                    threshold: self.get_parameter_single_value_at(2)?.parse().ok()?,
                })
            }
            SWAP_OWNER => {
                let old_owner = self.get_parameter_single_value_at(1)?;
                let new_owner = self.get_parameter_single_value_at(2)?;
                Some(SettingsInfo::SwapOwner {
                    old_owner: AddressEx::address_only(&old_owner),
                    new_owner: AddressEx::address_only(&new_owner),
                })
            }
            CHANGE_MASTER_COPY => {
                let implementation = self.get_parameter_single_value_at(0)?;
                Some(SettingsInfo::ChangeImplementation {
                    implementation: info_provider
                        .address_ex_from_contracts_or_default(&implementation)
                        .await,
                })
            }
            ENABLE_MODULE => {
                let module = self.get_parameter_single_value_at(0)?;
                Some(SettingsInfo::EnableModule {
                    module: info_provider
                        .address_ex_from_contracts_or_default(&module)
                        .await,
                })
            }
            DISABLE_MODULE => {
                let module = self.get_parameter_single_value_at(1)?;
                Some(SettingsInfo::DisableModule {
                    module: info_provider
                        .address_ex_from_contracts_or_default(&module)
                        .await,
                })
            }
            CHANGE_THRESHOLD => Some(SettingsInfo::ChangeThreshold {
                threshold: self.get_parameter_single_value_at(0)?.parse().ok()?,
            }),
            SET_GUARD => {
                let guard = self.get_parameter_single_value_at(0)?;
                let settings_info = if guard != "0x0000000000000000000000000000000000000000" {
                    let guard = info_provider
                        .address_ex_from_contracts_or_default(&guard)
                        .await;
                    SettingsInfo::SetGuard { guard }
                } else {
                    SettingsInfo::DeleteGuard
                };
                Some(settings_info)
                // gas_token == "0x0000000000000000000000000000000000000000"
            }
            _ => None,
        }
    }
}

impl DataDecoded {
    pub(crate) async fn build_address_info_index(
        &self,
        info_provider: &(impl InfoProvider + Sync),
    ) -> Option<HashMap<String, AddressEx>> {
        if !feature_flag_nested_decoding() {
            return None;
        }

        let mut index: HashMap<String, AddressEx> = HashMap::new();
        if self.method == MULTI_SEND {
            if let Some(value_decoded_type) =
                &self.get_parameter_value_decoded(MULTI_SEND_TRANSACTIONS_PARAM)
            {
                match value_decoded_type {
                    ValueDecodedType::InternalTransaction(transactions) => {
                        for transaction in transactions.iter() {
                            insert_value_into_index(&transaction.to, &mut index, info_provider)
                                .await;
                            put_parameter_into_index(
                                &transaction
                                    .data_decoded
                                    .as_ref()
                                    .map(|it| it.parameters.to_owned())
                                    .flatten(),
                                &mut index,
                                info_provider,
                            )
                            .await
                        }
                    }
                }
            }
        } else {
            put_parameter_into_index(&self.parameters, &mut index, info_provider).await;
        }
        if index.is_empty() {
            None
        } else {
            Some(index)
        }
    }

    pub fn has_nested_delegated(&self) -> bool {
        if let Some(parameters) = &self.parameters {
            parameters
                .iter()
                .map(|parameter| {
                    if let Some(value) = &parameter.value_decoded {
                        match value {
                            ValueDecodedType::InternalTransaction(transactions) => transactions
                                .iter()
                                .filter(|transaction| transaction.operation == Operation::DELEGATE)
                                .collect::<Vec<&InternalTransaction>>()
                                .is_empty(),
                        }
                    } else {
                        true // the "if" branch checks that there are NO entries with DELEGATE, therefore default true
                    }
                })
                .filter(|&it| it) // filter "true" meaning, we remove those entries with "no" DELEGATE
                .collect::<Vec<bool>>()
                .is_empty()
        } else {
            false
        }
    }
}

async fn put_parameter_into_index(
    parameters: &Option<Vec<Parameter>>,
    index: &mut HashMap<String, AddressEx>,
    info_provider: &impl InfoProvider,
) {
    if let Some(parameters) = parameters {
        for parameter in parameters {
            match &parameter.value {
                ParamValue::SingleValue(value) => {
                    insert_value_into_index(value, index, info_provider).await
                }
                ParamValue::ArrayValue(values) => {
                    for value in values {
                        if let ParamValue::SingleValue(value) = value {
                            insert_value_into_index(value, index, info_provider).await
                        }
                    }
                }
            }
        }
    }
}

async fn insert_value_into_index(
    value: &String,
    index: &mut HashMap<String, AddressEx>,
    info_provider: &impl InfoProvider,
) {
    if value.len() == 42
        && value.starts_with("0x")
        && value != "0x0000000000000000000000000000000000000000"
        && !index.contains_key(value)
    {
        if let Some(address_ex) = info_provider.address_ex_from_any_source(&value).await.ok() {
            index.insert(value.to_owned(), address_ex);
        };
    }
}
