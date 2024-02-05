use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
enum Lang {
    English,
    Spanish,
    Chinese,
    Texan,
    French,
}

#[derive(Debug)]
struct Greeting {
    message: String,
    lang: Lang,
}

fn main() {
    let mut v: Vec<Greeting> = Vec::new();

    let g: Greeting = Greeting {
        lang: Lang::English,
        message: String::from("Hello WasmEdge!"),
    };
    v.push(g);
    let g: Greeting = Greeting {
        lang: Lang::Spanish,
        message: String::from("Hola WasmEdge!"),
    };
    v.push(g);
    let g: Greeting = Greeting {
        lang: Lang::Texan,
        message: String::from("Howdy WasmEdge!"),
    };
    v.push(g);
    let g: Greeting = Greeting {
        lang: Lang::Chinese,
        message: String::from("WasmEdge 你好!"),
    };
    v.push(g);
    let g: Greeting = Greeting {
        lang: Lang::French,
        message: String::from("Salut WasmEdge!"),
    };
    v.push(g);

    println!("{:#?}", v);

    let query_lang = Lang::French;
    for e in v {
        if query_lang == e.lang {
            println!("{:?} {}", e.lang, e.message);
        }
    }
}
