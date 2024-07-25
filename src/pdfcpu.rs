use tokio::process::Command;
use serde_json::Value;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::fs::OpenOptions;


pub fn merge_image(image_input: &str, pdf_input: &Str, pdf_output: &str) -> Result<(), Box<dyn std::error::Error>> {

    //let mut cmd = std::process::Command::new("pdftk");
    // pdfcpu.exe stamp add -mode image -pages 1 -- "pie-chart.png" "scalefactor:0.45 rel, rot:0, position:bl, offset:50 50" tax_statement.pdf out.pdf

    let mut pdfcpu_params = Vec::new();
    pdfcpu_params.push(pdf_input);
    pdfcpu_params.push("stamp");
    pdfcpu_params.push("add");
    pdfcpu_params.push("-mode");
    pdfcpu_params.push("-pages");
    pdfcpu_params.push("1");
    pdfcpu_params.push("--");

    let image_input = format!("\"{}\"", image_input);
    pdfcpu_params.push(image_input);

//    let pdf_input = format!("\"{}\"", pdf_input);
    

    let params = "scalefactor:0.45 rel, rot:0, position:bl, offset:50 50";
    pdfcpu_params.push(param);

    pdfcpu_params.push(pdf_input);
    pdfcpu_params.push(pdf_output);


    let pdfcpu_command = Command::new("pdfcpu")
        .args(&pdfcpu_params)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let output = pdftk_command.wait_with_output().await?;
    if output.status.success() {
        let _stdout = String::from_utf8_lossy(&output.stdout);
      //  println!("pdftk command succeeded");
    } else {
        let _stderr = String::from_utf8_lossy(&output.stderr);
      //  println!("pdftk command failed");
    }

    // cmd.arg("output");
    // cmd.arg(pdf_output);

    // let output = cmd.output().await?;
    // let output = String::from_utf8_lossy(&output.stdout);
    // println!("{}", output);

    Ok(())
}
