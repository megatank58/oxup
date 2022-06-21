use crate::shell_command;

pub fn uninstall_w() {
    shell_command("rm", vec![r"C:\bin\oxido"]);
}