
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

    draw_timer(second) {
        ctx.fillStyle = "black";
        ctx.font = "16px serif";
        ctx.textAlign = "left";
        ctx.fillText("残り時間 " + second, 550, 100, 200);
    }
}
