import asyncio
import os


async def main():
    await asyncio.gather(
        run('cargo install drill'),
        run('cargo build')
    )


async def run(cmd):
    proc = await asyncio.create_subprocess_shell(
        cmd,
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.PIPE)

    stdout, stderr = await proc.communicate()

    # https://docs.python.org/3/library/asyncio-subprocess.html#asyncio-subprocess
    # for line in await proc.stdout.readline():
    #     print(line)

    print(f'[{cmd!r} exited with {proc.returncode}]')
    if stdout:
        print(f'[stdout]\n{stdout.decode()}')
    if stderr:
        print(f'[stderr]\n{stderr.decode()}')


asyncio.run(main())
