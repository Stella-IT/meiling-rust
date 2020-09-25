use diesel_derive_enum::DbEnum;

#[derive(DbEnum, Debug)]
pub enum AuthMethod {
    Password,
    Pubkey,
    OneTimePassword,
}

#[derive(DbEnum, Debug)]
pub enum LogType {
    JobStart,
    JobLog,
    JobEnd,
    UserLog,
    AuditLog,
}
