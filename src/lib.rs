use std::f64::consts::{TAU, PI, FRAC_PI_6};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;


fn draw(ctx: &web_sys::CanvasRenderingContext2d) -> Result<(), JsValue> {
    //
    ctx.translate(0.0, 0.0)?;
    {
        for i in 0..6 {
            for j in 0..6 {
                ctx.set_fill_style(&format!(
                    "rgb({},{},0)",
                    (255.0 - 42.5 * i as f64).floor() as i32,
                    (255.0 - 42.5 * j as f64).floor() as i32,
                ).into());
                ctx.fill_rect(25.0 * j as f64, 25.0 * i as f64, 25.0, 25.0);
            }
        }
    }
    ctx.translate(0.0, 160.0)?;
    {
        for i in 0..6 {
            for j in 0..6 {
                ctx.set_stroke_style(&format!(
                    "rgb(0,{},{})",
                    (255.0 - 42.5 * i as f64).floor() as i32,
                    (255.0 - 42.5 * j as f64).floor() as i32,
                ).into());
                ctx.begin_path();
                ctx.arc_with_anticlockwise(
                    13.5 + 25.0 * j as f64,
                    13.5 + 25.0 * i as f64,
                    10.0, 0.0,
                    TAU,
                    true,
                )?;
                ctx.stroke();
            }
        }
    }
    ctx.translate(0.0, 160.0)?;
    {
        ctx.set_fill_style(&"#FD0".into());
        ctx.fill_rect(0.0, 0.0, 75.0, 75.0);
        ctx.set_fill_style(&"#6C0".into());
        ctx.fill_rect(75.0, 0.0, 75.0, 75.0);
        ctx.set_fill_style(&"#09F".into());
        ctx.fill_rect(0.0, 75.0, 75.0, 75.0);
        ctx.set_fill_style(&"#F30".into());
        ctx.fill_rect(75.0, 75.0, 75.0, 75.0);
        //
        ctx.set_fill_style(&"#FFF".into());
        ctx.set_global_alpha(0.2);
        for i in 0..7 {
            ctx.begin_path();
            ctx.arc_with_anticlockwise(
                75.0, 75.0,
                10.0 + 10.0 * i as f64,
                0.0,
                TAU,
                true,
            )?;
            ctx.fill();
        }
    }
    ctx.translate(160.0, -320.0)?;
    {
        ctx.set_global_alpha(1.0);
        //
        ctx.set_fill_style(&"rgb(255,221,0)".into());
        ctx.fill_rect(0.0, 0.0, 152.0, 37.5);
        ctx.set_fill_style(&"rgb(102,204,0)".into());
        ctx.fill_rect(0.0, 37.5, 150.0, 37.5);
        ctx.set_fill_style(&"rgb(0,153,255)".into());
        ctx.fill_rect(0.0, 75.0, 150.0, 37.5);
        ctx.set_fill_style(&"rgb(255,51,0)".into());
        ctx.fill_rect(0.0, 112.0, 150.0, 37.5);
        //
        for i in 0..10 {
            ctx.set_fill_style(&format!(
                "rgba(255,255,255,{})",
                (i as f64 + 1.0) / 10.0
            ).into());
            for j in 0..4 {
                ctx.fill_rect(
                    5.0 + 14.0 * i as f64,
                    5.0 + 37.5 * j as f64,
                    14.0, 27.5,
                );
            }
        }
    }
    ctx.translate(0.0, 160.0)?;
    {
        ctx.set_stroke_style(&"#000".into());
        for i in 0..10 {
            ctx.set_line_width((i + i) as f64);
            ctx.begin_path();
            ctx.move_to(5.0 + 14.0 * i as f64, 5.0);
            ctx.line_to(5.0 + 14.0 * i as f64, 140.0);
            ctx.stroke();
        }
    }
    ctx.translate(0.0, 160.0)?;
    {
        let line_cap = vec!["butt", "round", "square"];
        //
        ctx.set_line_width(1.0);
        ctx.set_stroke_style(&"#09F".into());
        ctx.begin_path();
        ctx.move_to(10.0, 10.0);
        ctx.line_to(140.0, 10.0);
        ctx.move_to(10.0, 140.0);
        ctx.line_to(140.0, 140.0);
        ctx.stroke();
        //
        ctx.set_stroke_style(&"#000".into());
        ctx.set_line_width(15.0);
        for i in 0..line_cap.len() {
            ctx.set_line_cap(line_cap[i]);
            ctx.begin_path();
            ctx.move_to(25.0 + 50.0 * i as f64, 10.0);
            ctx.line_to(25.0 + 50.0 * i as f64, 140.0);
            ctx.stroke();
        }
    }
    ctx.translate(160.0, -320.0)?;
    {
        let line_join = vec!["round", "bevel", "miter"];
        ctx.set_line_width(10.0);
        for i in 0..line_join.len() {
            ctx.set_line_join(line_join[i]);
            ctx.begin_path();
            ctx.move_to(0.0, 10.0 + 40.0 * i as f64);
            ctx.line_to(35.0, 55.0 + 40.0 * i as f64);
            ctx.line_to(75.0, 10.0 + 40.0 * i as f64);
            ctx.line_to(115.0, 55.0 + 40.0 * i as f64);
            ctx.line_to(150.0, 10.0 + 40.0 * i as f64);
            ctx.stroke();
        }
    }
    ctx.translate(0.0, 160.0)?;
    {
        ctx.set_line_width(1.0);
        let line_grad = ctx.create_linear_gradient(0.0, 0.0, 0.0, 150.0);
        line_grad.add_color_stop(0.0, "#00ABEB")?;
        line_grad.add_color_stop(0.5, "#FFFFFF")?;
        line_grad.add_color_stop(0.5, "#26C000")?;
        line_grad.add_color_stop(1.0, "#FFFFFF")?;
        let line_grad2 = ctx.create_linear_gradient(0.0, 50.0, 0.0, 95.0);
        line_grad2.add_color_stop(0.5, "#000000")?;
        line_grad2.add_color_stop(1.0, "rgba(0,0,0,0)")?;
        ctx.set_fill_style(&line_grad);
        ctx.set_stroke_style(&line_grad2);
        ctx.fill_rect(10.0, 10.0, 130.0, 130.0);
        ctx.stroke_rect(50.0, 50.0, 50.0, 50.0);
    }
    ctx.translate(0.0, 160.0)?;
    {
        let rad_grad1 = ctx.create_radial_gradient(45.0, 45.0, 10.0, 52.0, 50.0, 30.0)?;
        rad_grad1.add_color_stop(0.0, "#A7D30C")?;
        rad_grad1.add_color_stop(0.9, "#019F62")?;
        rad_grad1.add_color_stop(1.0, "rgba(1, 159, 98,0)")?;
        let rad_grad2 = ctx.create_radial_gradient(105.0, 105.0, 20.0, 112.0, 120.0, 50.0)?;
        rad_grad2.add_color_stop(0.0, "#FF6F98")?;
        rad_grad2.add_color_stop(0.75, "#FF0188")?;
        rad_grad2.add_color_stop(1.0, "rgba(255, 1, 136, 0)")?;
        let rad_grad3 = ctx.create_radial_gradient(95.0, 15.0, 15.0, 102.0, 20.0, 40.0)?;
        rad_grad3.add_color_stop(0.0, "#00C9FF")?;
        rad_grad3.add_color_stop(0.8, "#00B5E2")?;
        rad_grad3.add_color_stop(1.0, "rgba(0, 201, 255, 0)")?;
        let rad_grad4 = ctx.create_radial_gradient(0.0, 150.0, 50.0, 0.0, 140.0, 90.0)?;
        rad_grad4.add_color_stop(0.0, "#F4F201")?;
        rad_grad4.add_color_stop(0.8, "#E4C700")?;
        rad_grad4.add_color_stop(1.0, "rgba(228,199,0,0)")?;
        //
        ctx.set_fill_style(&rad_grad4);
        ctx.fill_rect(0.0, 0.0, 150.0, 150.0);
        ctx.set_fill_style(&rad_grad3);
        ctx.fill_rect(0.0, 0.0, 150.0, 150.0);
        ctx.set_fill_style(&rad_grad2);
        ctx.fill_rect(0.0, 0.0, 150.0, 150.0);
        ctx.set_fill_style(&rad_grad1);
        ctx.fill_rect(0.0, 0.0, 150.0, 150.0);
    }
    ctx.translate(160.0, -320.0)?;
    {
        ctx.save();
        //
        ctx.set_shadow_offset_x(2.0);
        ctx.set_shadow_offset_y(2.0);
        ctx.set_shadow_blur(2.0);
        ctx.set_shadow_color("rgba(0,0,0,0.5)");
        //
        ctx.set_font("20px Times New Roman");
        ctx.set_fill_style(&"#000".into());
        ctx.fill_text("Sample String", 5.0, 30.0)?;
        //
        ctx.restore();
    }
    ctx.translate(0.0, 160.0)?;
    {
        ctx.set_fill_style(&"#000".into());
        ctx.fill_rect(0.0, 0.0, 150.0, 150.0);
        ctx.save();
        ctx.set_fill_style(&"#09F".into());
        ctx.fill_rect(15.0, 15.0, 120.0, 120.0);
        ctx.save();
        ctx.set_fill_style(&"#FFF".into());
        ctx.set_global_alpha(0.5);
        ctx.fill_rect(30.0, 30.0, 90.0, 90.0);
        ctx.restore();
        ctx.fill_rect(45.0, 45.0, 60.0, 60.0);
        ctx.restore();
        ctx.fill_rect(60.0, 60.0, 30.0, 30.0);
    }
    ctx.translate(0.0, 160.0)?;
    {
        ctx.save();
        ctx.translate(75.0, 75.0)?;
        for i in 0..6 {
            ctx.save();
            ctx.set_fill_style(&format!(
                "rgb({},{},255)",
                (51.0 * i as f64).floor() as i32,
                (255.0 - 51.0 * i as f64).floor() as i32,
            ).into());
            for _j in 0..(i * 6) {
                ctx.rotate(TAU / (6.0 * i as f64))?;
                ctx.begin_path();
                ctx.arc_with_anticlockwise(
                    0.0, 12.5 * i as f64,
                    5.0, 0.0,
                    TAU,
                    true,
                )?;
                ctx.fill();
            }
            ctx.restore();
        }
        ctx.restore();
    }
    ctx.translate(160.0, -320.0)?;
    {
        ctx.set_stroke_style(&"#FC0".into());
        ctx.set_line_width(1.5);
        ctx.fill_rect(0.0, 0.0, 150.0, 150.0);
        ctx.save();
        ctx.translate(75.0, 75.0)?;
        let (r1, r2, o) = (38.0, 8.0, 6.0);
        ctx.begin_path();
        ctx.move_to(r1 - o, 0.0);
        for i in 1..20000 {
            let p = (i as f64) * PI / 72.0;
            let x2 = (r1 + r2) * p.cos() - (r2 + o) * (((r1 + r2) / r2) * p).cos();
            let y2 = (r1 + r2) * p.sin() - (r2 + o) * (((r1 + r2) / r2) * p).sin();

            ctx.line_to(x2, y2);
            if x2 == r1 - o && y2 == 0.0 {
                break;
            }
        }
        ctx.stroke();
        ctx.restore();
    }
    ctx.translate(0.0, 160.0)?;
    {
        ctx.save();
        let sin = FRAC_PI_6.sin();
        let cos = FRAC_PI_6.cos();
        ctx.translate(75.0, 75.0)?;
        for i in 0..=12 {
            ctx.set_fill_style(&format!(
                "rgb({0},{0},{0})",
                (255.0 / 12.0 * i as f64).floor() as i32,
            ).into());
            ctx.fill_rect(0.0, 0.0, 72.0, 10.0);
            ctx.transform(cos, sin, -sin, cos, 0.0, 0.0)?;
        }
        ctx.transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)?;
        ctx.set_fill_style(&"rgba(255,128,255,0.5)".into());
        ctx.fill_rect(0.0, 0.0, 50.0, 50.0);
        ctx.restore();
    }
    Ok(())
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(800);
    canvas.set_height(480);

    let context = canvas.get_context("2d")?.unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    draw(&context)
}