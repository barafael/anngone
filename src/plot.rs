use anyhow::Ok;
use plotters::prelude::*;

const OUT_FILE_NAME: &'static str = "plot.png";

pub fn plot(data: &[(f64, usize)]) -> anyhow::Result<()> {
    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .right_y_label_area_size(40)
        .margin(5)
        .caption("Dual Y-Axis Example", ("sans-serif", 50.0).into_font())
        .build_cartesian_2d(0f64..1f64, 0.1f64..10f64)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .y_desc("Log Scale")
        .y_label_formatter(&|x| format!("{:e}", x))
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            data.into_iter().map(|(x, y)| (*x, *y as f64)),
            &BLUE,
        ))?
        .label("y = 1.02^x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&RGBColor(128, 128, 128))
        .draw()?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}
