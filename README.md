# rcon-command
Simple RCON client to send commands to Rust game server

# How to use
1. Rename `.env.example` into `.env` and fill it with your data
2. Build the app with `cargo build --release` and find the client in the `./target/release` directory
3. Run any command on the runned Rust server where you have RCON access: `./rcon-client -c "say 'Hello'" -e "/path/to/.env"`

Param `-e` is optional and in case it omitted app will try to find `.env` in same dir
