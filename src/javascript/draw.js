
const canvas = document.getElementById("canvas");

const GRID_COLOR = "#CCCCCC";
const ctx = canvas.getContext('2d');

function resources() {
    let res = {
        player: document.createElement('img'),
        bomb: document.createElement('img'),
        fire: document.createElement('img'),
        wall: document.createElement('img'),
        brock: document.createElement('img'),
        pow: document.createElement('img')
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
    res.bomb.src = "image/bomb.png";

    //Fire
    res.fire.width = 32;
    res.fire.height = 32;
    res.fire.src = "image/fire.png";

    // Wall
    res.wall.width = 32;
    res.wall.height = 32;
    res.wall.src = "image/wall.png";

    // Brock
    res.brock.width = 32;
    res.brock.height = 32;
    res.brock.src = "image/brock.png";

    // Pow
    res.pow.width = 32;
    res.pow.height = 32;
    res.pow.src = "image/pow.png";

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
        ctx.fillStyle = "green";
        ctx.fillRect(0, 0, canvas.width, canvas.height);
    }

    draw_player(x, y, idx, reverse, id) {
        ctx.translate(x - 16 + 32 * reverse, y - 16);
        ctx.scale(1 - 2 * reverse, 1);
        ctx.drawImage(res.player, 0 + idx * 32, 0 + 32 * id, 32, 32, 0, 0, 32, 32);
        //drawImage(画像ソース, x_start, y_start, x_end, y_end, 0, 0, 32, 32)
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
    }

    draw_bomb(x, y, idx) {
        ctx.translate(x - 16, y - 16);
        ctx.drawImage(res.bomb, 0 + 32 * idx, 0, 32, 32, 0, 0, 32, 32);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
    }

    draw_fire(x, y, idx) {
        ctx.translate(x - 16, y - 16);
        ctx.drawImage(res.fire, 0 + 32 * idx, 0, 32, 32, 0, 0, 32, 32);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
    }

    draw_wall(x, y) {
        ctx.translate(x - 16, y - 16);
        ctx.drawImage(res.wall, 0, 0, 32, 32, 0, 0, 32, 32);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
    }

    draw_brock(x, y, idx) {
        ctx.translate(x - 16, y - 16);
        ctx.drawImage(res.brock, 0 + 32 * idx, 0, 32, 32, 0, 0, 32, 32);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
    }

    draw_pow(x, y, id) {
        ctx.translate(x - 16, y - 16);
        ctx.drawImage(res.pow, 0 + 32 * id, 0, 32, 32, 0, 0, 32, 32);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
    }
}
