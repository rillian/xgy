// Text conversion for Ancient Egyptian.

fn main() {
    for arg in std::env::args().skip(1) {
        let mut mdc = arg;
        for (unicode, ascii) in [ ("ꜣ", "A"),
                                ("ꜥ", "a"),
                                ("š", "S"),
                                ("ḥ", "H"),
                                ("ḫ", "x"),
                                ("ẖ", "X"),
                                ("ṯ", "T"),
                                ("ḏ", "D"),
                                //("ḳ", "q"),
                              ].iter() {
            //mdc = mdc.replace(unicode, ascii);
            mdc = mdc.replace(ascii, unicode);
        }
        println!("{}", mdc);
    }
}
