use std::process::ExitCode;

pub struct CustomExitCode(u8);

#[allow(dead_code)]
impl CustomExitCode {
    pub const SUCCESS: Self = CustomExitCode(0);
    pub const FAILED_TO_CHECK_FIREWALL_RULE: Self = Self(1);
    pub const FAILED_TO_ADD_FIREWALL_RULE: Self = Self(2);
    pub const FAILED_TO_REMOVE_FIREWALL_RULE: Self = Self(4);
    pub const USER_DENIED_TO_ELEVATE: Self = Self(8);

    pub fn new() -> Self {
        CustomExitCode::SUCCESS
    }
    pub fn is_success(&self) -> bool {
        self.0 == Self::SUCCESS.0
    }
    pub fn has_flag(&self, flag: Self) -> bool {
        (self.0 & flag.0) != 0
    }
    pub fn clear_flag(&mut self, flag: Self) -> &mut Self {
        self.0 &= !flag.0;
        self
    }
    pub fn set_flag(&mut self, flag: Self) -> &mut Self {
        self.0 |= flag.0;
        self
    }

    pub fn to_exit_code(&self) -> ExitCode {
        if self.is_success() {
            ExitCode::SUCCESS
        } else {
            ExitCode::from(self.0)
        }
    }
}
