from AnkaiosSDK import Ankaios, Workload
import time

WAITING_TIME_IN_SEC = 5

if __name__ == "__main__":
    # Connect to control interface
    with Ankaios() as ankaios:
        # Create a new workload
        workload = Workload.builder() \
            .workload_name("dynamic_nginx") \
            .agent_name("agent_A") \
            .runtime("podman") \
            .restart_policy("NEVER") \
            .runtime_config("image: docker.io/library/nginx\ncommandOptions: [\"-p\", \"8080:80\"]") \
            .build()

        # Run the workload
        ankaios.run_workload(workload)

        while True:
            # Request complete state and print it
            complete_state = ankaios.get_state(
                timeout=5,
                field_mask=["workloadStates.agent_A.dynamic_nginx"])

            # Print complete state
            print(complete_state)

            time.sleep(WAITING_TIME_IN_SEC)
