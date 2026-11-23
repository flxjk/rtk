# Telemetry

External telemetry is disabled in this fork, including when an older configuration contains consent or a build environment contains collector settings. The release workflow builds both editions without telemetry endpoints or credentials.

`rtk gain` continues to show savings from the local tracking database.

| Command | Behavior |
| --- | --- |
| `rtk telemetry status` | Reports the effective disabled state. |
| `rtk telemetry enable` | Reports that this build has external telemetry disabled. |
| `rtk telemetry disable` | Records a local opt-out for compatibility with older builds. |
| `rtk telemetry forget` | Records the opt-out and removes the local tracking database and legacy telemetry files. |
