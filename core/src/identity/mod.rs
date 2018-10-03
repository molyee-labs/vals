type Id = u64;

type UserId = Id;

struct User {
    id: UserId,
    name: String,
    email: String,
    password: String,
    phone: String,
    
}

type RoleId = Id;

struct Role {
    id: RoleId,
    name: String,
}
