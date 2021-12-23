pub fn help() {
    let bold = "\x1b[1m";
    let reset = "\x1b[0m";

    println!(
        r"
{} 
    reator <command> <template> <path>

{}
    n | new | create         Creates a new file
    d | del | delete         Deletes existing file
    h | ? | help             Shows this screen

{} 
    rc | component           React Component
    ct | context             React Context
    np | next-page           Next.js Page
    nd | next-doc            Next.js _document file
    s  | style               CSS Module
    sc | styled              Styled Component

{} 
    Filename only, no extension needed. 
    Reator creates folders and files automatically.
    ",
        format!("{}Usage:{}", bold, reset),
        format!("{}Commands:{}", bold, reset),
        format!("{}Templates:{}", bold, reset),
        format!("{}Path:{}", bold, reset),
    )
}
