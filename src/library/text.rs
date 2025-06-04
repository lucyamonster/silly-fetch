#[doc = "Just return the text within the quotes (\"this\") "]
pub fn just_quotes(cut_me: String) -> String {
    let sliced = cut_me.split("\"").collect::<Vec<&str>>();
    if sliced.len() != 3 {
        println!("sliced:");
        for i in sliced {
            println!("{}", i);
        }
        panic!("has a weird length")
    } else {
        return sliced[1].to_string();
    }
}

#[doc = "converts `to_say=hello` to `hello`"]
pub fn just_value(cut_me: String) -> String {
    let sliced = cut_me.split("=").collect::<Vec<&str>>();
     if sliced.len() != 2 {
        println!("sliced:");
        for i in sliced {
            println!("{}", i);
        }
        panic!("has a weird length")
    } else {
        return sliced[1].to_string();
    }
}