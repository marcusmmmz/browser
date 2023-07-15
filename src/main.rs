mod markup;

use markup::markup_text_to_html;

fn main() {
    println!(
        "{}",
        markup_text_to_html(
            "div (class=\"flex justify-center\") {
                p \"Hello\"
                br
                strong {
                    p {
                        \"Nice\"
                        i \"markup\"
                        \"bro\"
                    }
                }
            }"
        )
    );
}
