//! # Dev Examples - example `liquid` code
//! Note: These examples are here for dev use and functionality testing.
//!
//! Currently looking at `liquid` to replace current custom pattern replace.
//! (Use of existing piping system sounds particularly useful.)

fn main() -> anyhow::Result<()> {
        let globals = liquid::object!({
            "num": 78_f64
        });

        let template = liquid::ParserBuilder::with_stdlib().build()?
                                                           .parse("Liquid! {{num | minus: 12}}")
                                                           .unwrap();

        let output = template.render(&globals)
                             .unwrap();
        println!("rendered template: {}", output);

        Ok(())
}
