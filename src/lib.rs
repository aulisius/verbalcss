use std::collections::HashMap;

#[derive(Debug)]
struct Config {
    add_vendor_prefix: bool,
}

#[derive(Debug)]
enum Color {
    Hex(String),
    Rgb(i32, i32, i32),
    Rgba(i32, i32, i32, f32),
    Hsl(i32, i32, i32),
    Hsla(i32, i32, i32, f32),
}

#[derive(Debug)]
struct CSSElement {
    name: String,
    properties: HashMap<String, String>,
}

#[derive(Debug)]
struct VerbalCSS {
    config: Config,
    curr_elem: Option<CSSElement>,
}

impl VerbalCSS {
    fn new(c: Config) -> Self {
        VerbalCSS {
            config: c,
            curr_elem: None,
        }
    }

    fn new_elem(&mut self, elem: String) -> &mut Self {
        self.curr_elem = Some(CSSElement {
            name: elem,
            properties: HashMap::new(),
        });
        self
    }

    fn with_color(&mut self, color: Color) -> &mut Self {

        let color_str = match color {
            Color::Hex(s) => format!("#{}", s),
            Color::Rgb(r, g, b) => format!("rgb({},{},{})", r, g, b),
            Color::Rgba(r, g, b, a) => format!("rgba({},{},{},{})", r, g, b, a),
            Color::Hsl(h, s, l) => format!("hsl({},{},{})", h, s, l),
            Color::Hsla(h, s, l, a) => format!("hsla({},{},{},{})", h, s, l, a),
        };

        if let Some(ref mut elem) = self.curr_elem {
            elem.properties.insert("color".to_string(), color_str);
        } else {
            println!("No element to add color to! Make an element first!");
        }

        self
    }

    fn make(&mut self) -> String {
        if let Some(ref mut elem) = self.curr_elem {
            let mut tmp = String::new();

            for (k, v) in elem.properties.iter() {
                tmp.push_str(&format!("{}:{};\n", k, v));
            }

            format!("{} {{ \n {} }}", elem.name, tmp)
        } else {
            println!("No element to make! Make an element first!");
            String::new()
        }
    }
}

fn main() {
    let mut verbal_css = VerbalCSS::new(Config { add_vendor_prefix: false });

    println!("{:?}",
             verbal_css.new_elem("li".to_string())
                       .with_color(Color::Hex("fff".to_string()))
                       .make());


    println!("{:?}",
             verbal_css.new_elem("h1".to_string())
                       .with_color(Color::Hex("f00".to_string()))
                       .make());
}
