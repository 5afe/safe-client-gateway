{{/*
Expand the name of the chart.
*/}}
{{- define "safe-client-gateway.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "safe-client-gateway.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "safe-client-gateway.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Default labels
*/}}
{{- define "safe-client-gateway.labels" -}}
helm.sh/chart: {{ include "safe-client-gateway.chart" . }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
app.kubernetes.io/name: {{ .Release.Name }}
app.kubernetes.io/instance: {{ include "safe-client-gateway.name" . }}

{{- end }}

{{/*
Redis Secret
*/}}
{{- define "safe-client-gateway.redis-secret" -}}
{{- if .Values.config.redis.secretReferenceKey -}}
{{- .Values.config.redis.secretReferenceKey }}
{{- else -}}
{{ include "safe-client-gateway.name" . }}-redis
{{- end -}}
{{- end -}}

{{/*
Redis Mainnet Secret
*/}}
{{- define "safe-client-gateway.redis-mainnet-secret" -}}
{{- if .Values.config.redisMainnet.secretReferenceKey -}}
{{- .Values.config.redisMainnet.secretReferenceKey }}
{{- else -}}
{{ include "safe-client-gateway.name" . }}-redis-mainnet
{{- end -}}
{{- end -}}