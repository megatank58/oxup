use crate::shell_command;

pub fn uninstall_l() {
    shell_command("rm", vec!["-rf", "$HOME/.oxido/oxido"]);
}
