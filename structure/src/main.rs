struct Player {
    username: String,
    email: String,
    kill_count: u128,
    pos: Position,
    unique: Godlike,
}

struct Position(i32, i32, i32);

struct Godlike;

fn main() {
    let mut aspas = Player {
        username: String::from("aspas finalboss"),
        email: String::from("aspas@dev"),
        kill_count: 139_278,
        pos: Position(10, 0, -10),
        unique: Godlike,
    };

    aspas.email = String::from("aspas@pro");

    let aspas_smurf = Player {
        username: String::from("aspas semiboss"),
        ..aspas
    };

    aspas.username;
    aspas.kill_count;
    // aspas.email;
    // aspas.pos;
    // aspas.unique;
}
