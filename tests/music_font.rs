extern crate font;
extern crate svg;

use font::Glyph;
use svg::node::element::Group;

const TTF: &'static str = "tests/fixtures/MTF-Cadence-Fin.ttf";
const SVG: &'static str = "tests/output/g-clef.svg";

#[test]
fn test_jonas() {
    println!("Hello, Jonas");
    assert!(true);
}

#[test]
fn test_font() {
    use font::Font;
    use svg::node::element::Style;
    use svg::Document;

    let font = Font::open(TTF).unwrap();
    let glyph = font.draw('&').unwrap().unwrap();
    let (width, height) = (glyph.advance_width(), glyph.height() + 2.0 * 50.0);
    let transform = format!("translate(0, {}) scale(1, -1)", glyph.bounding_box.3 + 50.0);
    let glyph = draw(&glyph).set("transform", transform);
    let style = Style::new("path { fill: black; stroke: none; stroke-width: 3; }");
    let document = Document::new()
        .set("width", width)
        .set("height", height)
        .add(style)
        .add(glyph);
    svg::save(SVG, &document).unwrap();
}

fn draw(glyph: &Glyph) -> Group {
    use font::{Offset, Segment};
    use svg::node::element::path::Data;
    use svg::node::element::Path;
    use svg::node::Node;

    let mut group = Group::new();
    let mut a = Offset::default();

    let mut data = Data::new();

    for contour in glyph.iter() {
        a += contour.offset;

        data = data.move_to(vec![a.0, a.1]);

        for segment in contour.iter() {
            match segment {
                &Segment::Linear(b) => {
                    a += b;
                    data = data.line_by(vec![b.0, b.1]);
                }
                &Segment::Cubic(b, mut c, mut d) => {
                    c += b;
                    d += c;
                    a += d;
                    data = data.cubic_curve_by(vec![b.0, b.1, c.0, c.1, d.0, d.1]);
                }
                &Segment::Quadratic(b, mut c) => {
                    c += b;
                    a += c;
                    data = data.quadratic_curve_by(vec![b.0, b.1, c.0, c.1]);
                }
                _ => unreachable!(),
            }
        }

        data = data.close();

        // group.append(Path::new().set("d", data));
    }
    group.append(Path::new().set("d", data));

    group
}
