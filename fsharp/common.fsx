#load "keysym.fsx"
module K = KeySym
type KeyPress = { key: int; mask: int }
type Callback = { remap: KeyPress; command: string array }
type Binding = KeyPress * Callback
type AppBindings = { wm_class: string; bindings: obj array }
let inline kb k m = { key = k; mask = m }
let inline alt k = kb k K.M_Alt
let inline ctrl k = kb k K.M_Ctrl
let inline ctrlsh k = kb k (K.M_Ctrl|||K.M_Shift)
let inline altsh k = kb k (K.M_Alt|||K.M_Shift)
let inline cmd k = { remap = Unchecked.defaultof<_>; command = Seq.toArray k }
let inline remap tgt = { remap = tgt; command = null }
let inline bind (kp: KeyPress) (cb: Callback) : Binding = (kp, cb)
let inline app (name: string) (hooks: Binding seq) : AppBindings = {
    wm_class = name
    bindings = [|
        for hook in hooks do
            [| box (fst hook); box (snd hook) |]
    |]
}