use web_sys::CanvasRenderingContext2d;
use std::f64::consts::PI;

pub fn draw_menu_icon(ctx: &CanvasRenderingContext2d){
    
    // 描画処理部分
    // rectangle生成
    ctx.set_fill_style_str("white");
    ctx.fill_rect(0.0, 0.0, 320.0, 320.0);
    ctx.set_fill_style_str("red");
    ctx.fill_rect(60.0, 30.0, 30.0, 180.0);
    ctx.set_fill_style_str("blue");
    ctx.fill_rect(60.0, 30.0, 60.0, 30.0);
    ctx.set_fill_style_str("blue");
    ctx.fill_rect(60.0, 130.0, 60.0, 30.0);
    
    // パスを使用し線や図形を描く
    ctx.set_stroke_style_str("yellow");
    ctx.set_fill_style_str("yellow");
    // パス開始の宣言(ペンを持つ)
    ctx.begin_path();
    // 開始地点に移動(ペンを動かす)
    ctx.move_to(60.0, 210.0);
    // 次の地点を決める(どこまで線を引くか決める)
    ctx.line_to(90.0, 210.0);
    ctx.line_to(10.0, 260.0);
    // 開始地点に戻る
    ctx.close_path();
    // 輪郭線を描画(線を描く)
    //  ctx.stroke();
    // パス内部を塗りつぶす(バケツ)
    // fillする場合strokeは書かなくてもいい
    ctx.fill();
    
    // 円や弧を描く(今回は1/2ドーナツを描く)
    ctx.set_stroke_style_str("orange");
    ctx.set_fill_style_str("orange");
    ctx.begin_path();
    /*
     * ctx.arc(x, y, radius, startAngle, endAngle)
     * xy=中心の座標, radius=半径,
     * startAngle=開始地点, endAngle=次の地点(radianで書く)
     * 360°=2PIradian
     * 1radian=弧の長さと半径の長さが同じ時の中心角
     * =約57.29578°
     */
    // 開始地点(円の中心)を決め半径nのxRadian~yRadianまで線を引く
    ctx.arc(125.0, 95.0, 65.0, -PI/2.0, PI/2.0).unwrap();
    // 今回作る物の関係で隙間が空くので少し左にずらす
    ctx.line_to(120.0, 160.0);
    /* 
     * 弧の描画方向を変えたいときは、
     * arc_with_anticlockwiseを使い第6引数にtrue, falseを追加する
     */
    // 少し円の右を太くしたいので中心をずらす
    ctx.arc_with_anticlockwise(120.0, 95.0, 35.0, PI/2.0, -PI/2.0, true).unwrap();
    // 今回作る物の関係で隙間が空くので真上に伸ばす
    ctx.line_to(120.0, 30.0);
    // 開始地点に戻る
    ctx.close_path();
    ctx.fill();
    
    // Pの右上にあるやつ
    ctx.begin_path();
    ctx.move_to(180.0, 30.0);
    ctx.line_to(200.0, 30.0);
    ctx.line_to(200.0, 50.0);
    ctx.close_path();
    ctx.fill();
    
    // Rの上部分
    ctx.begin_path();
    ctx.arc(225.0, 95.0, 65.0, -PI/2.0, PI/2.0).unwrap();
    ctx.line_to(220.0, 160.0);
    ctx.arc_with_anticlockwise(220.0, 95.0, 35.0, PI/2.0, -PI/2.0, true).unwrap();
    ctx.line_to(220.0, 30.0);
    ctx.close_path();
    ctx.fill();
    
    // Rの下部分1
    ctx.begin_path();
    ctx.move_to(220.0, 130.0);
    ctx.line_to(250.0, 150.0);
    ctx.line_to(200.0, 200.0);
    ctx.line_to(175.0, 175.0);
    ctx.close_path();
    ctx.fill();
    
    // Rの下部分2
    ctx.begin_path();
    ctx.move_to(220.0, 180.0);
    ctx.line_to(300.0, 260.0);
    ctx.line_to(300.0, 280.0);
    ctx.line_to(280.0, 280.0);
    ctx.line_to(200.0, 200.0);
    ctx.close_path();
    ctx.fill();
    
    // 2の下の上部分
    ctx.begin_path();
    ctx.move_to(150.0, 170.0);
    ctx.line_to(170.0, 190.0);
    ctx.line_to(110.0, 250.0);
    ctx.line_to(70.0, 250.0);
    ctx.close_path();
    ctx.fill();
    
    // 2の下の下部分
    ctx.fill_rect(70.0, 250.0, 120.0, 30.0);
}