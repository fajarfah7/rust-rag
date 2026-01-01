pub enum AuthUsecaseError {
    WrongUsernameOrPassword,
    DatabaseError,
    InternalServerError,
    EmailAlreadyExist,
    UsernameAlreadyExist,
}
