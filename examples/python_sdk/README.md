# Python SDK example

This explains how to work with the python sdk example.

In the src folder, there are 3 scripts:

* `original_example.py`: The original example that starts a workload and gets the state every 5 seconds
* `sleep.py`: Sleep forever, until the container gets forcefully closed.
* `vars.py`: In here the user can declare variables to prepare the run.

If the Dockerfile is configured using the `sleep.py` script, all the steps bellow can be followed to run custom sdk commands.

For full access, the startupState.yaml can be changed with:

```yaml
apiVersion: v0.1
workloads:
  control_interface_example:
    controlInterfaceAccess:
      allowRules:
        - type: StateRule
          operation: ReadWrite
          filterMask:
            - "*"
```

## Start the container

```sh
cargo build --release
cd examples
./run_example python_sdk
```

## Enter the container

```sh
podman ps
podman exec -ti <container_id> /bin/sh
```

## Go in the dir with the scripts and start python

```sh
cd /ankaios/src
python3
```

## Run the sdk

```py
from ankaios_sdk import *
from vars import *
ankaios = Ankaios()
...
```
