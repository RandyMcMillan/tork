use tork_rs;

fn main() {
    println!("Tork Version: {}", tork_rs::version());
    println!("Git Commit: {}", tork_rs::git_commit());
}
