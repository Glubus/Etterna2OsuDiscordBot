use anyhow::Result;
use plotters::prelude::*;
use minacalc_rs::Ssr;

const MAX_SKILL_VALUE: f64 = 40.0;

pub fn generate_radar_chart(
    title: &str,
    subtitle: &str,
    labels: &[&str],
    values: &[f64],   // valeurs normalisées [0.0, 1.0]
    size: (u32, u32),
) -> Result<Vec<u8>, anyhow::Error> {
    assert_eq!(labels.len(), values.len(), "Labels et valeurs doivent correspondre");

    const BG_COLOR: RGBColor = RGBColor(15, 15, 15);
    const GRID_COLOR: RGBColor = RGBColor(40, 40, 40);
    const AXIS_COLOR: RGBColor = RGBColor(60, 60, 60);
    const POINT_COLOR: RGBColor = RGBColor(255, 255, 255);

    let (width, height) = size;
    let center = (width as i32 / 2, height as i32 / 2);
    let radius = (width.min(height) as f64) * 0.35;
    let num_axes = labels.len();

    let temp_file = "temp_radar.png";

    {
        let root = BitMapBackend::new(temp_file, size).into_drawing_area();
        root.fill(&BG_COLOR)?;

        let chart = root.titled(title, ("sans-serif", 18).into_font().color(&WHITE))?;

        // Subtitle
        chart.draw_text(subtitle, &("sans-serif", 16).into_font().color(&WHITE), (20, 30))?;

        // Cercles concentriques (5 niveaux)
        for i in 1..=5 {
            let r = radius * (i as f64 / 5.0);
            chart.draw(&Circle::new(center, r as i32, &GRID_COLOR))?;
        }

        // Angles de chaque axe
        let angles: Vec<f64> = (0..num_axes)
            .map(|i| std::f64::consts::TAU * i as f64 / num_axes as f64)
            .collect();

        // Axes
        for &angle in &angles {
            let x = center.0 as f64 + radius * angle.cos();
            let y = center.1 as f64 + radius * angle.sin();
            chart.draw(&PathElement::new(vec![center, (x as i32, y as i32)], &AXIS_COLOR))?;
        }

        // Points du polygone radar
        let points: Vec<(i32, i32)> = values.iter()
            .zip(&angles)
            .map(|(&val, &angle)| {
                let x = center.0 as f64 + radius * val * angle.cos();
                let y = center.1 as f64 + radius * val * angle.sin();
                (x as i32, y as i32)
            })
            .collect();

        // Couleur dynamique en fonction de la moyenne
        let avg = values.iter().copied().sum::<f64>() / num_axes as f64;
        let fill_color = RGBColor((255.0 * avg) as u8, 150, (255.0 * (1.0 - avg)) as u8);

        // Polygone rempli
        chart.draw(&Polygon::new(points.clone(), fill_color.mix(0.6).filled()))?;

        // Points blancs sur les sommets
        for &pt in &points {
            chart.draw(&Circle::new(pt, 4, &POINT_COLOR))?;
        }

        // Labels
        for (i, (&label, &angle)) in labels.iter().zip(&angles).enumerate() {
            let label_radius = radius + 40.0;
            let x = center.0 as f64 + label_radius * angle.cos();
            let y = center.1 as f64 + label_radius * angle.sin();

            let x = x.clamp(10.0, (width - 10) as f64) as i32;
            let y = y.clamp(10.0, (height - 10) as f64) as i32;

            chart.draw_text(label, &("sans-serif", 18).into_font().color(&WHITE), (x, y + 5))?;
            chart.draw_text(
                &format!("{:.1}", values[i]*MAX_SKILL_VALUE),   
                &("sans-serif", 16).into_font().color(&WHITE),
                (x, y + 25),
            )?;
        }

        root.present()?;
    }

    let buffer = std::fs::read(temp_file)?;
    let _ = std::fs::remove_file(temp_file);

    Ok(buffer)
}

pub fn generate_msd_chart(ssr: &Ssr, beatmap_title: &str, version: &str) -> Result<Vec<u8>, anyhow::Error> {
    let skill_values = [
        ssr.stream,
        ssr.jumpstream,
        ssr.handstream,
        ssr.jackspeed,
        ssr.stamina,
        ssr.chordjack,
    ];

    let skill_names = [
        "Stream",
        "Jumpstream",
        "Handstream",
        "Jackspeed",
        "Stamina",
        "Chordjack",
    ];

    let normalized_values: Vec<f64> = skill_values
        .iter()
        .map(|&val| (val as f64 / MAX_SKILL_VALUE).clamp(0.0, 1.0))
        .collect();

    generate_radar_chart(beatmap_title, version, &skill_names, &normalized_values, (700, 700))
}