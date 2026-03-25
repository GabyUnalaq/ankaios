---
name: tutorial-checker
description: Check Ankaios tutorials by executing safe documented shell steps and validating expected outputs. Use this when asked to "check tutorials on main" or to verify local tutorial files.
---

# Tutorial Checker Skill

Use this skill to validate tutorials by directly following tutorial steps as the agent (not by delegating logic to a large checker script).

## Trigger behavior

- If the user asks "check tutorials on main" (or equivalent), use main docs as default without asking for source.
- If the user asks to check local tutorials, ask for the tutorial name (for example `events` for `tutorial-events.md`).

## Tutorial discovery

- Main tutorials:
    - Load `https://eclipse-ankaios.github.io/ankaios/main/usage/installation/`.
    - Discover all links containing `/usage/tutorial-` at runtime.
- Local tutorials:
    - Discover files matching `/workspaces/ankaios/doc/docs/usage/tutorial-*.md` at runtime.
    - If user requested local mode without a name, ask for the name.

## Temporary workspace (required)

- If you need to create tutorial-generated files in the repository workspace, use a dedicated folder under `target` (for example `target/tutorial-check-outputs`) and clean it up at the end.
- Create all required files and intermediate files inside the dedicated folder
- Run file-dependent commands from that temp directory (or use absolute temp paths) so no accidental repo files are produced.
- At the end, report the temp directory used and whether cleanup was performed.

## Execution process (agent-driven)

For each selected tutorial:

1. Parse markdown and extract shell code blocks.
2. Split multiline shell blocks into executable commands (respect line continuations using `\`).
3. Run reach step one by one.
4. For each command:
    - Show progress before execution (`Tutorial X/Y - Step A/B: <command>`).
    - Classify step as `execute`, `transform-and-execute`, `skip-manual`, or `skip-unsafe`.
    - Execute safe commands in terminal and capture stdout/stderr + exit code.
    - If a waiting time is required before the next command, don't combine it with the next command as this may affect allowed command detection and execution.
5. Validate expectations:
    - If surrounding text states "should print" and a following text block exists, compare output to expected snippets.
    - Mark step as failed if command fails or expected snippets are missing.
    - Include a warning in the final report as this may indicate a problem with the tutorial or environment.
6. Validate human understandability:
    - Review the tutorial text around each step and mark `understandable` or `unclear`.
    - Flag unclear items such as missing prerequisites, unexplained placeholders, ambiguous wording, missing expected result, or required manual context not stated.

## Command details

- The following commands are in the PATH:
    - `ank`, `ank-agent`, `ankaios-start`, `ankaios-clean`, `podman`, `systemctl`
- If the checking is running in a container, `systemctl` commands for starting Ankaios shall be supplemented with `ankaios-start` and `ank-agent`. In case `systemdctl` was not tested, the user must be informed in the final report.
- Disallow chained commands (for example `cmd1; cmd2`, `cmd1 && cmd2`, `cmd1 || cmd2`).
- Transform streaming commands to non-blocking equivalents and execute the transformed command:
    - `ank logs -f <workload>` or `ank logs --follow <workload>` -> `ank logs --tail 10 <workload>`
    - `mosquitto_sub ...` -> bounded receive mode, e.g. add `-C <count>` and `-W <seconds>` so command exits automatically.
    - Report both original and transformed command in step output.
- When server server IP substitution is required (for example containing `<SERVER_IP>`) use `http://localhost:25551` and assume all execution is local.

## Live run feedback

- While running, continuously provide concise status updates:
    - current tutorial
    - current step
    - pass/fail/skip decision
    - short reason for skips/failures

## Final report format

After all selected tutorials:

1. Per tutorial: `passed`, `failed`, `skipped` counts.
2. For each failure include:
    - exact command
    - output (or output tail if long)
    - expected snippets (if any)
    - likely cause / what may have gone wrong
3. Human-understandability summary per tutorial:
    - only list `unclear` steps with their surrounding text and reason for being unclear
    - give a brief overall assessment of the tutorial's clarity and usability based on the number and severity of unclear steps
4. End with: ask whether to try fixing detected problems.
