use juniper::GraphQLEnum;

#[derive(GraphQLEnum, Debug, Clone, PartialEq)]
pub enum AuthenticationMethod {
    Password,
    Pubkey,
    OneTimePassword,
    Fido2,
}

#[derive(GraphQLEnum, Debug, Clone, PartialEq)]
pub enum LogType {
    JobStart,
    JobLog,
    JobEnd,
    UserLog,
    AuditLog,
}
