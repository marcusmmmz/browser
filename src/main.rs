mod markup;

use markup::markup_text_to_html;

fn main() {
    println!(
        "{}",
        markup_text_to_html(
            "div {
                div(class=\"flex justify-center\") {
                    p(text=\"banana\")
                }
            }"
        )
    );
}
