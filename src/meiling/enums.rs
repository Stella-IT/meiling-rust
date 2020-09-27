#[derive(Debug, Clone, PartialEq)]
pub enum AuthenticationMethod {
    Password,
    Pubkey,
    OneTimePassword,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogType {
    JobStart,
    JobLog,
    JobEnd,
    UserLog,
    AuditLog,
}
