use juniper::GraphQLEnum;

#[derive(GraphQLEnum, Debug, Clone, PartialEq)]
pub enum AuthenticationMethod {
    Password,
    Pubkey,
    OneTimePassword,
}

#[derive(GraphQLEnum, Debug, Clone, PartialEq)]
pub enum LogType {
    JobStart,
    JobLog,
    JobEnd,
    UserLog,
    AuditLog,
}
