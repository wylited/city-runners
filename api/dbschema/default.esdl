using extension auth;

module default {
    type Player {
        required property username: str{
                 constraint exclusive
        }
        required property password: str;
        required property admin: bool;
    }
}
