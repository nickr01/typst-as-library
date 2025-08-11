use std::fs;

use typst::layout::Abs;
use typst_as_library::TypstWrapperWorld;
use typst_pdf::PdfOptions;

//const BSNSW_FUSCHIA_01: &str = "#65004d";

fn main() {

    let content = r#"
    #set text(font: "Public Sans")
    #set page(
      paper: "a4",
      margin: (top: 3.5cm),
      header: context {
        if here().page() > 1 {
          [#text(stroke: rgb("da1884"))[This is a long title that will be truncated if it is too long]]
        }   
        [#h(1fr) #box([#image("bsnsw_logo_1.png", width: 30%)])
        #line(length: 100%, stroke: rgb("65004d"))]
}
  )
  text: #box(height: 14pt)[#line(length: 20%)]

      #table(
        columns: (auto, auto, auto),
        inset: 10pt,
        align: horizon,
        table.header(
          [TABLE1], [*Volume*], [*Parameters*],
        ),
        [], 
        $ pi h (D^2 - d^2) / 4 $,
        [
          $h$: height \
          $D$: outer radius \
          $d$: inner radius
        ],
        [],
        $ sqrt(2) / 12 a^3 $,
        [$a$: edge length]
      )

      #set table(
        stroke: none,
        gutter: 0.2em,
        fill: (x, y) =>
          if x == 0 or y == 0 { gray },
        inset: (right: 1.5em),
      )

      #show table.cell: it => {
        if it.x == 0 or it.y == 0 {
          set text(white)
          strong(it)
        } else if it.body == [] {
          // Replace empty cells with 'N/A'
          pad(..it.inset)[_N/A_]
        } else {
          it
        }
      }

      #let a = table.cell(
        fill: green.lighten(60%),
      )[A]
      #let b = table.cell(
        fill: aqua.lighten(60%),
      )[B]

      #table(
        rows: 100,
        columns: 4,
        [TABLE2], [Exam 1], [Exam 2], [Exam 3],

        [John], [Summink], a, [],
        [Mary], [], a, a,
        [Robert], b, a, b,
        [John], b, b, b,
      )

      #table(
        rows: 100,
        columns: 4,
        table.header([TABLE3], [Exam 1], [Exam 2], [Exam 3]),
        [John], [Summink], a, [],
        [Mary], [], a, a,
        [Robert], b, a, b,
        [John], b, b, b,
      )


    "#
//     r#"
// #import "@preview/polylux:0.4.0": *

// #set page(paper: "presentation-16-9")
// #show math.equation: set text(font: "Libertinus Math");


// #set text(
//   font: "Lato",
//   size: 23pt,
// )

// #slide[
//   #set page(footer: none)
//   #set align(horizon + center)

// = Hello, World!
// A document (+ `polylux` library) rendered with `Typst`!
// $ y = m x + n $
// ]"#
    .to_owned();

    // Create world with content.
    let world = TypstWrapperWorld::new("../".to_owned(), content);

    // Render document
    let document = typst::compile(&world)
        .output
        .expect("Error compiling typst");

    // Output to pdf and svg
    let pdf = typst_pdf::pdf(&document, &PdfOptions::default()).expect("Error exporting PDF");
    fs::write("./output.pdf", pdf).expect("Error writing PDF.");
    println!("Created pdf: `./output.pdf`");

    let svg = typst_svg::svg_merged(&document, Abs::pt(2.0));
    fs::write("./output.svg", svg).expect("Error writing SVG.");
    println!("Created svg: `./output.svg`");
}
