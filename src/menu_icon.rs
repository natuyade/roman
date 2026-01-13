use std::f64::consts::PI;

pub fn draw_menu_icon(ctx: &web_sys::CanvasRenderingContext2d){
    
    // 描画処理部分
    // ctx.set_stroke_style_str("white");
    // ctx.set_fill_style_str("white");
    // 背景生成
    // ctx.fill_rect(0.0, 0.0, 320.0, 320.0);
    /*
     * set_global_composite_operation
     *  destnation-out=描いた場所を透過
     *  source-over=通常の描画
     */
    ctx.set_global_composite_operation("destination-out").unwrap();
    // 左上
    ctx.begin_path();
    ctx.arc(64.0, 64.0, 64.0, PI, -PI/2.0).unwrap();
    ctx.line_to(0.0, 0.0);
    ctx.close_path();
    ctx.fill();
    // 右上
    ctx.begin_path();
    ctx.arc(256.0, 64.0, 64.0, -PI/2.0, 0.0).unwrap();
    ctx.line_to(320.0, 0.0);
    ctx.close_path();
    ctx.fill();
    // 左下
    ctx.begin_path();
    ctx.arc(64.0, 256.0, 64.0, PI/2.0, PI).unwrap();
    ctx.line_to(0.0, 320.0);
    ctx.close_path();
    ctx.fill();
    // 右下
    ctx.begin_path();
    ctx.arc(256.0, 256.0, 64.0, 0.0, PI/2.0).unwrap();
    ctx.line_to(320.0, 320.0);
    ctx.close_path();
    ctx.fill();
    // 通常の描画
    ctx.set_global_composite_operation("source-over").unwrap();
    
    // rectangle生成(Pの直線)
    // ctx.set_fill_style_str("black");
    ctx.fill_rect(50.0, 60.0, 30.0, 140.0);
    ctx.fill_rect(50.0, 30.0, 60.0, 30.0);
    ctx.fill_rect(50.0, 130.0, 60.0, 30.0);
    
    // パスを使用し線や図形を描く(Pの左下のびよ～ん)
    // パス開始の宣言(ペンを持つ)
    ctx.begin_path();
    // 開始地点に移動(ペンを動かす)
    ctx.move_to(50.0, 200.0);
    // 次の地点を決める(どこまで線を引くか決める)
    ctx.line_to(80.0, 200.0);
    ctx.line_to(10.0, 260.0);
    // 開始地点に戻る
    ctx.close_path();
    // 輪郭線を描画(線を描く)
    //  ctx.stroke();
    // パス内部を塗りつぶす(バケツ)
    // fillする場合strokeは書かなくてもいい
    ctx.fill();
    
    // 円や弧を描く(今回は1/2ドーナツを描く)
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
    ctx.arc(115.0, 95.0, 65.0, -PI/2.0, PI/2.0).unwrap();
    // 今回作る物の関係で隙間が空くので少し左にずらす
    ctx.line_to(110.0, 160.0);
    /* 
     * 弧の描画方向を変えたいときは、
     * arc_with_anticlockwiseを使い第6引数にtrue, falseを追加する
     */
    // 少し円の右を太くしたいので中心をずらす
    ctx.arc_with_anticlockwise(110.0, 95.0, 35.0, PI/2.0, -PI/2.0, true).unwrap();
    // 今回作る物の関係で隙間が空くので真上に伸ばす
    ctx.line_to(110.0, 30.0);
    // 開始地点に戻る
    ctx.close_path();
    ctx.fill();
    
    // Pの右上にあるやつ
    ctx.begin_path();
    ctx.arc(145.0, 65.0, 35.0, -PI/2.0, 0.0).unwrap();
    ctx.line_to(180.0, 30.0);
    ctx.close_path();
    ctx.fill();
    
    // R上部分
    ctx.begin_path();
    ctx.arc(215.0, 95.0, 65.0, -PI/2.0, PI/2.0).unwrap();
    ctx.line_to(190.0, 150.0);
    ctx.arc_with_anticlockwise(210.0, 95.0, 35.0, PI/2.0, -PI/2.0, true).unwrap();
    ctx.line_to(200.0, 60.0);
    ctx.line_to(200.0, 30.0);
    ctx.close_path();
    ctx.fill();
    
    // R下部分
    ctx.begin_path();
    ctx.move_to(200.0, 140.0);
    ctx.line_to(300.0, 280.0);
    ctx.line_to(260.0, 280.0);
    ctx.line_to(175.0, 165.0);
    ctx.close_path();
    ctx.fill();
    
    // 2下上部分
    ctx.begin_path();
    ctx.move_to(150.0, 160.0);
    ctx.line_to(170.0, 180.0);
    ctx.line_to(100.0, 250.0);
    ctx.line_to(60.0, 250.0);
    ctx.close_path();
    ctx.fill();
    
    // 2下下部分
    ctx.fill_rect(60.0, 250.0, 120.0, 30.0);
}

pub fn draw_menu_icon_false(ctx: &web_sys::CanvasRenderingContext2d){
}