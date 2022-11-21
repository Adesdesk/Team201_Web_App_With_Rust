// a call to external crate
use yew::prelude::*;
use whatlang::{detect, Lang, Script};

// define language detector function
fn detect_language(mut x: String) {
    let text = &x;

// handle exceptions using .unwrap()
    let info = detect(text).unwrap();

// manage output using the println! macro
    println!("____________",);
    println!("Language type {}", info.lang());

    println!("Language Script {}", info.script());

    println!("System reliability {}", info.is_reliable());
    println!("System confidence {}", info.confidence());

}

fn main() {
// input set of strings of speeches to detect language they're written in
    let speech_0 = String::from(
        "C'est une belle journée à Manchester, Royaume-Uni",
    );

    let speech_1 = String::from(
        "It's a beautiful day in Manchester, United Kingdom",
    );

    let speech_2 = String::from(
        "Прекрасный день в Манчестере, Великобритания",
    );

    let speech_3 = String::from(
        "إنه يوم جميل في مانشستر ، المملكة المتحدة",
    );

    let speech_4 = String::from(
        "Es ist ein schöner Tag in Manchester, Vereinigtes Königreich",
    );
// invoke function on each language speech
    detect_language(speech_0);
    detect_language(speech_1);
    detect_language(speech_2);
    detect_language(speech_3);
    detect_language(speech_4);
}