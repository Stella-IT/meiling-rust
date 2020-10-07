use diesel_derive_enum::DbEnum;

#[derive(DbEnum, Debug, Clone, PartialEq)]
pub enum AuthenticationMethod {
    Password,
    Pubkey,
    OneTimePassword,
    Fido2,
}

#[derive(DbEnum, Debug, Clone, PartialEq)]
pub enum LogType {
    JobStart,
    JobLog,
    JobEnd,
    UserLog,
    AuditLog,
    DebugLog,
    ClientLog,
}
