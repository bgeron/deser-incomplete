
# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run it on this file.
Output "target/live-travel-modes.gif"
Set Padding 10
Set Theme "AtomOneLight"
Set Shell bash
Set Height 520
# Sleep 1s
Hide
Type@0ms "reset && echo '$ cargo run --example live -- --schema=travel-modes'"
Type@0ms " && cargo run -q --example live -- --schema=travel-modes"
Type@0ms " --tui-height=18"
Enter
Wait+Screen /Output/
Show
Sleep 1s
Type@50ms `[{"mode": 'foot`
#Sleep 250ms
Backspace@25ms 5
Type@50ms `"foot", "benefit"::"he`
#Sleep 250ms
Backspace@25ms 4
Type@50ms ` "healthy"}, `
Enter
Type@50ms ` {"mode" "ae`
#Sleep 250ms
Backspace@25ms 4
Type@50ms `: "aeroplane", "benefitr": "f`
#Sleep 250ms
Backspace@25ms 6
Type@50ms `": "fasst" damage is tolerated`
Sleep 400ms
Left@25ms 22
Backspace@25ms 1
Sleep 1s
Hide
Escape
