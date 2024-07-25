use plotters::prelude::*;
use std::collections::HashMap;

#[warn(dead_code)]
fn hex_to_rgb(hex: &str) -> RGBColor {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    RGBColor(r, g, b)
}

#[warn(dead_code)]
pub fn generate_pie_chart(tax_data: &Vec<(&str, f64)>, output_file: &str) -> Result<String, String> {
    
    let color_lookup: HashMap<&str, &str> = [
        ("MC", "#F58282"),
        ("SE", "#F5B082"),
        ("ED", "#FDFA7E"),
        ("LI", "#DEF582"),
        ("LH", "#B0F582"),
        ("CC", "#82F582"),
        ("LD", "#82F5B0"),
        ("FA", "#82F5DE"),
        ("SD", "#82DEF5"),
        ("FD", "#82B0F5"),
        ("FL", "#8282F5"),
        ("RI", "#B082F5"),
        ("TV", "#DE82F5"),
        ("MR", "#F582DE"),
        ("WV", "#F582B0"),
    ].iter().cloned().collect();



    let root_area = BitMapBackend::new(output_file, (300, 250)).into_drawing_area();
    //root_area.fill(&WHITE).unwrap();

    // RGB with alpha channel
    let transparent_white = RGBAColor(255, 255, 255, 1.0);
    root_area.fill(&transparent_white).map_err(|e| e.to_string())?;
    //root_area.fill(&WHITE).map_err(|e| e.to_string())?;

    // match root_area.fill(&WHITE){
    //     Ok(_) => (),
    //     Err(e) => return Err(e.to_string())
    // }

    let title_style = TextStyle::from(("sans-serif", 30).into_font()).color(&(BLACK));
    root_area.titled("", title_style).unwrap();

    let dims = root_area.dim_in_pixel();
    let center = (dims.0 as i32 / 2, dims.1 as i32 / 2);
    let radius = 100.0;  // Adjusted radius to make the pie chart smaller



    let mut sorted_tax_map = tax_data.clone();
    sorted_tax_map.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mid = sorted_tax_map.len() / 2;
    let (large, small) = sorted_tax_map.split_at(mid);
    
    let mut interleaved_tax_map = Vec::new();
    for i in 0..mid {
        interleaved_tax_map.push(large[i].clone());
        if i < small.len() {
            interleaved_tax_map.push(small[i].clone());
        }
    }

    let sizes: Vec<f64> = interleaved_tax_map.iter().map(|&(_, v)| v).collect();
    let labels: Vec<&str> = interleaved_tax_map.iter().map(|&(k, _)| k).collect();
    let colors: Vec<RGBColor> = interleaved_tax_map.iter().map(|&(k, _)| hex_to_rgb(color_lookup.get(k).unwrap())).collect();
    
    let mut pie = Pie::new(&center, &radius, &sizes, &colors, &labels);
    pie.start_angle(66.0);
    pie.label_style((("sans-serif", 15).into_font()).color(&(BLACK)));
    pie.label_offset(15.0);
    root_area.draw(&pie).map_err(|e| e.to_string())?;

    Ok(format!("Pie chart generated successfully and saved to {}", output_file))
}
