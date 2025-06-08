use lib_core::generate_snowflake_id;

fn main() {
    let id = generate_snowflake_id().unwrap();
    println!("{}", id);
}
