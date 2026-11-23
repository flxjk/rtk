mod common;

#[test]
fn external_telemetry_cannot_be_enabled() {
    let enable = common::rtk_command()
        .env_remove("RTK_TELEMETRY_DISABLED")
        .args(["telemetry", "enable"])
        .output()
        .expect("run telemetry enable");
    assert!(!enable.status.success());
    assert!(String::from_utf8_lossy(&enable.stderr).contains("disabled in this build"));

    let status = common::rtk_command()
        .env_remove("RTK_TELEMETRY_DISABLED")
        .args(["telemetry", "status"])
        .output()
        .expect("run telemetry status");
    assert!(status.status.success());
    assert!(String::from_utf8_lossy(&status.stdout).contains("enabled:       no"));

    let gain = common::rtk_command()
        .env_remove("RTK_TELEMETRY_DISABLED")
        .arg("gain")
        .output()
        .expect("inspect local savings");
    assert!(gain.status.success());
}
