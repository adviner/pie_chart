mod pie_chart;
mod merge_to_pdf;

const OUT_CHART: &str = "C:\\Projects\\pie_chart\\pie-chart.png";
const TAX_STATEMENT: &str = "C:\\Projects\\pie_chart\\tax_statement.pdf";
const MERGED_TAX_STATEMENT: &str = "C:\\Projects\\pie_chart\\tax_statement_merged.pdf";

fn main() {
    
    let tax_map = vec![
        ("ED", 36.31),
        ("CC", 9.77),
        ("TV", 0.48),
        ("FA", 0.85),
        ("SD", 5.95),
        ("MC", 14.85),
        ("FD", 29.63),
        ("LI", 2.16),
    ];

    // let result = pie_chart::generate_pie_chart(&tax_map, &OUT_CHART);
    // match result {
    //     Ok(msg) => println!("{}", msg),
    //     Err(e) => println!("Error: {}", e),
    // }

   // let pdf_output = "C:\\Projects\\pie_chart\\pie-chart.pdf";
    merge_to_pdf::merge_image_to_pdf(&OUT_CHART, &TAX_STATEMENT, &MERGED_TAX_STATEMENT).unwrap();
}