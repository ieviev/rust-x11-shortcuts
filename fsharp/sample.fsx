#load "common.fsx"
open Common
module K = KeySym
let bind_firefox =
    app "firefox-aurora" [
        // remap alt1-4 to ctrl 1-4
        bind (alt K.XK_1) (remap (ctrl K.XK_1))
        bind (alt K.XK_2) (remap (ctrl K.XK_2))
        bind (alt K.XK_3) (remap (ctrl K.XK_3))
        bind (alt K.XK_4) (remap (ctrl K.XK_4))
    ]

let bind_nemo =
    app "Nemo" [
        // map alt q to send a notification
        bind (alt K.XK_q) (cmd [ "notify-send"; "Nemo"; "alt-q-pressed" ])
        // map alt 1 to go to downloads
        bind (alt K.XK_1) (cmd [ "nemo"; "--existing-window"; "/home/ian/Downloads" ])
        // map alt ` to go to desktop
        bind (alt K.XK_grave) (cmd [ "nemo"; "--existing-window"; "/home/ian/Desktop" ])
    ]
let allbinds = [|
    bind_firefox
    bind_nemo
|]

let main() =
    let x11app = __SOURCE_DIRECTORY__ + "/../target/release/examples/applications"
    let info = System.Diagnostics.ProcessStartInfo(x11app, "", RedirectStandardInput = true)
    use proc = System.Diagnostics.Process.Start(info)
    let json_input = System.Text.Json.JsonSerializer.SerializeToUtf8Bytes(allbinds)
    let proc_input = proc.StandardInput.BaseStream
    proc_input.Write(json_input)
    proc.StandardInput.Close()
    proc.WaitForExit()
main ()
