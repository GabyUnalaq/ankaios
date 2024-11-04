manifest_str = r"""apiVersion: v0.1
workloads:
  nginx:
    agent: "{{agent.agent_name}}"
    runtime: podman
    configs:
      port: web_server_config
      agent: agents
    runtimeConfig: |
      image: ghcr.io/eclipse-ankaios/tests/nginx:alpine-slim
      commandOptions: [ "-p", "{{port.access_port}}:80"]
  greeting_person:
    agent: agent_A
    runtime: podman
    configs:
      person: person
    runtimeConfig: |
      image: ghcr.io/eclipse-ankaios/tests/alpine:latest
      commandArgs: [ "echo", '{{#each person}}{{#if (eq this.age "40")}}Hello {{this.name}}(age: {{this.age}})\n{{/if}}{{/each}}' ]
configs:
  web_server_config:
    access_port: "8081"
  person:
    - name: John Doe
      age: "30"
    - name: Chris Smith
      age: "40"
  agents:
    agent_name: agent_A"""
