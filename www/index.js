import {GameData} from "bomberhuman";

const canvas = document.getElementById("bomberhuman-canvas");
const gamedata = GameData.new();

canvas.width = gamedata.width();
canvas.height = gamedata.height();

const GRID_COLOR = "#CCCCCC";
const ctx = canvas.getContext('2d');

const drawPlayer = (x,y) => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;
  
    // arc(x座標, 
    //     y座標,
    //     半径, 
    //     開始角度: 0度 (0 * Math.PI / 180), 
    //     終了角度: 360度 (360 * Math.PI / 180),  
    //     方向: true=反時計回りの円、false=時計回りの円);
    ctx.arc(x, y, 30, 0 * Math.PI / 180, 360 * Math.PI / 180, false ) ;
    // 塗りつぶしの色    
    ctx.fillStyle = "rgba(200,200,200,0.8)" ;
    // 塗りつぶしを実行 
    ctx.fill() ;
    // 線を描画を実行
    ctx.stroke();
};

const clear_screen = () => {
    ctx.fillStyle = "black";
    ctx.fillRect(0, 0, canvas.width, canvas.height);
};

document.addEventListener('keydown', e => gamedata.action(e.key, true));
document.addEventListener('keyup', e => gamedata.action(e.key, false));

const renderLoop = () => {    
    gamedata.update();
    clear_screen();
    
    drawPlayer(gamedata.p_x(), gamedata.p_y());
    
    requestAnimationFrame(renderLoop);
};

clear_screen();
drawPlayer(gamedata.p_x(), gamedata.p_y());
requestAnimationFrame(renderLoop);