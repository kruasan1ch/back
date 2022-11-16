# back
sudo docker run --name postgresql -e POSTGRES_USER=usr -e POSTGRES_PASSWORD=pwd -p 5432:5432 -d postgres
cargo install diesel_cli --no-default-features --features postgres
diesel setup
cargo run
