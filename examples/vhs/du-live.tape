# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run it on this file.
Output "target/du-live.gif"
Set Padding 20
Set Theme "AtomOneLight"
Set Shell bash
Set Height 660
Hide
Type@0ms "reset && echo '$ cargo run --example du -- /dev /etc /snap /var /root /usr' && echo '  | cargo run --example live -- -o nushell-light --wrap'"
Type@0ms " && { sleep 2 ; cargo run -q --example du -- /dev /etc /snap /var /root /usr ; sleep 5 ; } | cargo run -q --example print-slowly -- -t 0.0025 | cargo run -q --example live -- -o nushell-light --wrap"
Type@0ms " --tui-height=22"
# Backspace
Enter
Wait+Screen /Output/
Show
Wait+Screen@30s /usr.*iB/
Sleep 3s
Hide