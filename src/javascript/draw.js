
const canvas = document.getElementById("canvas");

const GRID_COLOR = "#CCCCCC";
const ctx = canvas.getContext('2d');

function resources(){
    let res = {
        player: document.createElement('canvas'),
        bomb: document.createElement('canvas'),
        wall: document.createElement('canvas')
    }

    //Player
    res.player.width = 32;
    res.player.height = 32;
    let plCtx = res.player.getContext('2d');
    plCtx.beginPath();
    plCtx.strokeStyle = GRID_COLOR;
    // arc(x座標, 
    //     y座標,
    //     半径, 
    //     開始角度: 0度 (0 * Math.PI / 180), 
    //     終了角度: 360度 (360 * Math.PI / 180),  
    //     方向: true=反時計回りの円、false=時計回りの円);
    plCtx.arc(16, 16, 16, 0 * Math.PI / 180, 360 * Math.PI / 180, false ) ;
    // 塗りつぶしの色    
    plCtx.fillStyle = "rgba(200,200,200,0.8)" ;
    // 塗りつぶしを実行 
    plCtx.fill() ;
    // 線を描画を実行
    plCtx.stroke();

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
    bmCtx.arc(16, 16, 16, 0 * Math.PI / 180, 360 * Math.PI / 180, false ) ;
    // 塗りつぶしの色    
    bmCtx.fillStyle = "rgba(30,30,90,0.8)" ;
    // 塗りつぶしを実行 
    bmCtx.fill() ;
    // 線を描画を実行
    bmCtx.stroke();

    // Wall
    res.wall.width = 32;
    res.wall.height = 32;
    let wallCtx = res.wall.getContext('2d');
    wallCtx.fillStyle = "rgba(150,120,0,0.8)";
    wallCtx.beginPath();
    wallCtx.fillRect(0, 0, res.wall.width, res.wall.height);

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
        ctx.translate(x-16, y-16);
        ctx.drawImage(res.player, 0, 0);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";  
    }

    draw_bomb(x, y) {
        ctx.translate(x-16, y-16);
        ctx.drawImage(res.bomb, 0, 0);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";  
    }

    draw_wall(x, y) {
        ctx.translate(x-16, y-16);
        ctx.drawImage(res.wall, 0, 0);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";  
    }
}