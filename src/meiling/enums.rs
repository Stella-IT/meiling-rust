#[derive(Debug, Clone, PartialEq)]
pub enum AuthenticationMethod {
    Password,
    Pubkey,
    OneTimePassword,
    Fido2,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogType {
    JobStart,
    JobLog,
    JobEnd,
    UserLog,
    AuditLog,
}
