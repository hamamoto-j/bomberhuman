
const canvas = document.getElementById("canvas");

const GRID_COLOR = "#CCCCCC";
const ctx = canvas.getContext('2d');

function resources() {
    let res = {
        player: document.createElement('img'),
        bomb: document.createElement('canvas'),
        fire: document.createElement('canvas'),
        wall: document.createElement('canvas'),
        brock: document.createElement('canvas'),
        pow: document.createElement('canvas')
    }

    //Player
    res.player.width = 32;
    res.player.height = 32;
    res.player.src = "image/player.png";
//    let plCtx = res.player.getContext('2d');
//    plCtx.beginPath();
//    plCtx.strokeStyle = GRID_COLOR;
    // arc(x座標, 
    //     y座標,
    //     半径, 
    //     開始角度: 0度 (0 * Math.PI / 180), 
    //     終了角度: 360度 (360 * Math.PI / 180),  
    //     方向: true=反時計回りの円、false=時計回りの円);
//    plCtx.arc(16, 16, 16, 0 * Math.PI / 180, 360 * Math.PI / 180, false);
    // 塗りつぶしの色    
//    plCtx.fillStyle = "rgba(200,200,200,0.8)";
    // 塗りつぶしを実行 
//    plCtx.fill();
    // 線を描画を実行
//    plCtx.stroke();

    //Bomb
    res.bomb.width = 32;
    res.bomb.height = 32;
    let bmCtx = res.bomb.getContext('2d');
    bmCtx.beginPath();
    bmCtx.strokeStyle = GRID_COLOR;
    // arc(x座標, 
    //     y座標,
    //     半径, 
    //     開始角度: 0度 (0 * Math.PI / 180), 
    //     終了角度: 360度 (360 * Math.PI / 180),  
    //     方向: true=反時計回りの円、false=時計回りの円);
    bmCtx.arc(16, 16, 16, 0 * Math.PI / 180, 360 * Math.PI / 180, false);
    // 塗りつぶしの色    
    bmCtx.fillStyle = "rgba(30,30,90,0.8)";
    // 塗りつぶしを実行 
    bmCtx.fill();
    // 線を描画を実行
    bmCtx.stroke();

    //Fire
    res.fire.width = 32;
    res.fire.height = 32;
    let frCtx = res.fire.getContext('2d');
    frCtx.beginPath();
    frCtx.strokeStyle = GRID_COLOR;
    // arc(x座標, 
    //     y座標,
    //     半径, 
    //     開始角度: 0度 (0 * Math.PI / 180), 
    //     終了角度: 360度 (360 * Math.PI / 180),  
    //     方向: true=反時計回りの円、false=時計回りの円);
    frCtx.arc(16, 16, 16, 0 * Math.PI / 180, 360 * Math.PI / 180, false);
    // 塗りつぶしの色    
    frCtx.fillStyle = "rgba(200,30,30,0.8)";
    // 塗りつぶしを実行 
    frCtx.fill();
    // 線を描画を実行
    frCtx.stroke();

    // Wall
    res.wall.width = 32;
    res.wall.height = 32;
    let wallCtx = res.wall.getContext('2d');
    wallCtx.fillStyle = "rgba(30,60,200,0.8)";
    wallCtx.beginPath();
    wallCtx.fillRect(0, 0, res.wall.width, res.wall.height);

    // Brock
    res.brock.width = 32;
    res.brock.height = 32;
    let brockCtx = res.brock.getContext('2d');
    brockCtx.fillStyle = "rgba(150,100,10,0.8)";
    brockCtx.beginPath();
    brockCtx.fillRect(0, 0, res.brock.width, res.brock.height);

    // Pow
    res.pow.width = 32;
    res.pow.height = 32;
    let powCtx = res.pow.getContext('2d');
    powCtx.beginPath();
    powCtx.strokeStyle = GRID_COLOR;
    // arc(x座標, 
    //     y座標,
    //     半径, 
    //     開始角度: 0度 (0 * Math.PI / 180), 
    //     終了角度: 360度 (360 * Math.PI / 180),  
    //     方向: true=反時計回りの円、false=時計回りの円);
    powCtx.arc(16, 16, 13, 0 * Math.PI / 180, 360 * Math.PI / 180, false);
    // 塗りつぶしの色    
    powCtx.fillStyle = "rgba(200,200,30,0.8)";
    // 塗りつぶしを実行 
    powCtx.fill();
    // 線を描画を実行
    powCtx.stroke();

    return res;
}

const res = resources();

export class Draw {
    width(x) {
        //        canvas.width = window.innerWidth * 0.8;
        canvas.width = x;
        console.log(canvas.width);
        return canvas.width;
    }

    height(y) {
        //        canvas.height = window.innerHeight * 0.8;
        canvas.height = y;
        console.log(canvas.height);
        return canvas.height;
    }

    clear_screen() {
        ctx.fillStyle = "black";
        ctx.fillRect(0, 0, canvas.width, canvas.height);
    }

    draw_player(x, y) {
        ctx.translate(x - 16, y - 16);
        ctx.drawImage(res.player, 0, 0, 640, 640, 0, 0, 32, 32);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
    }

    draw_bomb(x, y) {
        ctx.translate(x - 16, y - 16);
        ctx.drawImage(res.bomb, 0, 0);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
    }

    draw_fire(x, y) {
        ctx.translate(x - 16, y - 16);
        ctx.drawImage(res.fire, 0, 0);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
    }

    draw_wall(x, y) {
        ctx.translate(x - 16, y - 16);
        ctx.drawImage(res.wall, 0, 0);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
    }

    draw_brock(x, y) {
        ctx.translate(x - 16, y - 16);
        ctx.drawImage(res.brock, 0, 0);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
    }

    draw_pow(x, y) {
        ctx.translate(x - 16, y - 16);
        ctx.drawImage(res.pow, 0, 0);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
    }
}
