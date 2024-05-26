To build a release version use command
cargo build --release
The program can be then run with command
cargo run [insert input here]
or from the target/release directory with
./days [insert input here]

If you want to be able to run the program from any directory use command
sudo cp ./target/release/days /usr/local/bin/
to copy the application to the bin folder
Then the program can be run simply with the command
days [insert input here]