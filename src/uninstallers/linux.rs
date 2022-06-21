use crate::shell_command;

pub fn uninstall_l() {
    shell_command("sudo", vec!["rm -rf", "/usr/local/bin/oxido"]);
}