
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
        pow: document.createElement('img'),
        powup: document.createElement('img'),
        particle: document.createElement('img'),
        shadow: document.createElement('canvas'),
    }

    //Player
    res.player.width = 32;
    res.player.height = 32;
    res.player.src = "image/player.png";

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

    // Powup
    res.powup.width = 32;
    res.powup.height = 32;
    res.powup.src = "image/powup.png";

    // Particle
    res.particle.width = 32;
    res.particle.height = 32;
    res.particle.src = "image/particle.png";

    // Shadow
    res.shadow.width = 32;
    res.shadow.height = 32;
    let shCtx = res.shadow.getContext('2d');
    // arc(x座標, 
    //     y座標,
    //     半径, 
    //     開始角度: 0度 (0 * Math.PI / 180), 
    //     終了角度: 360度 (360 * Math.PI / 180),  
    //     方向: true=反時計回りの円、false=時計回りの円);
    shCtx.arc(16, 16, 15, 0 * Math.PI / 180, 360 * Math.PI / 180, false);
    // 塗りつぶしの色    
    shCtx.fillStyle = "rgba(80,80,120,0.6)";
    // 塗りつぶしを実行 
    shCtx.fill();

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
        this.draw_shadow(x, y);
        ctx.translate(x - 16 + 32 * reverse, y - 16);
        ctx.scale(1 - 2 * reverse, 1);
        ctx.drawImage(res.player, 0 + idx * 32, 0 + 32 * id, 32, 32, 0, 0, 32, 32);
        //drawImage(画像ソース, x_start, y_start, x_end, y_end, 0, 0, 32, 32)
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
    }

    draw_bomb(x, y, idx) {
        this.draw_shadow(x, y);
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
        this.draw_shadow(x, y);
        ctx.translate(x - 16, y - 16);
        ctx.drawImage(res.pow, 0 + 32 * id, 0, 32, 32, 0, 0, 32, 32);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
    }

    draw_powup(x, y, id, type) {
        ctx.translate(x - 16, y - 16);
        ctx.drawImage(res.powup, 0 + 32 * id, 0 + 32 * type, 32, 32, 0, 0, 32, 32);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
    }

    draw_particle(x, y, id) {
        ctx.translate(x - 16, y - 16);
        ctx.drawImage(res.particle, 0 + 32 * id, 0, 32, 32, 0, 0, 32, 32);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
    }

    draw_shadow(x, y) {
        ctx.translate(x - 16, y + 12);
        ctx.scale(1, 0.2);
        ctx.drawImage(res.shadow, 0, 0);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.fillStyle = "black";
    }

    draw_timer(second) {
        ctx.fillStyle = "black";
        ctx.font = "24px serif";
        ctx.textAlign = "center";
        ctx.fillText("残り時間 " + second, 580, 100, 200);
    }

    draw_timeup() {
        ctx.fillStyle = "white";
        ctx.font = "100px serif";
        ctx.textAlign = "center";
        ctx.fillText("Time Up!!", canvas.width / 2, canvas.height / 2);
        ctx.strokeText("Time Up!!", canvas.width / 2, canvas.height / 2);
    }

    draw_winner(id) {
        ctx.fillStyle = "white";
        ctx.font = "100px serif";
        ctx.textAlign = "center";
        ctx.fillText("Winner", canvas.width / 2, canvas.height / 2 - 32 * 2);
        ctx.strokeText("Winner", canvas.width / 2, canvas.height / 2 - 32 * 2);
        ctx.fillText((id + 1) + "P", canvas.width / 2, canvas.height / 2 + 32 * 2);
        ctx.strokeText((id + 1) + "P", canvas.width / 2, canvas.height / 2 + 32 * 2);

    }

    draw_draw() {
        ctx.fillStyle = "white";
        ctx.font = "100px serif";
        ctx.textAlign = "center";
        ctx.fillText("Draw", canvas.width / 2, canvas.height / 2);
        ctx.strokeText("Draw", canvas.width / 2, canvas.height / 2);
    }
}
