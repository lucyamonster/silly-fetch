echo Starting builds
char-repeater = 50
echo

# More at https://github.com/cross-rs/cross
echo Linux for x86_64
char-repeater = 50
cross build --target x86_64-unknown-linux-gnu --release

echo
echo Widows for x86_64
char-repeater = 50
cross build --target x86_64-pc-windows-gnu --release

echo
echo Files can be found at target/release
cp target/x86_64-pc-windows-gnu/release/oxidized-dungeon.exe target/release/
cp target/x86_64-unknown-linux-gnu/release/oxidized-dungeon target/release/