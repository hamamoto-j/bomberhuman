import { GameData } from "bomberhuman";


const gamedata = GameData.new();

gamedata.add_player(4);
gamedata.add_brock();

////////////////////////////////////////////////////////////////
// Gamepads handling
////////////////////////////////////////////////////////////////

// Needs "gamepadconnected" handler even if empty.
function init_gamepads(gp) {
    if (debug) {
        console.log("Gamepad connected at index:%d buttons:%d axes:%d [%s]",
            gp.index, gp.buttons.length, gp.axes.length, gp.id);
    }
}

function scan_gamepads() {
    // Chrome should refresh gamepads everytime you read.
    var gamepads = navigator.getGamepads ? navigator.getGamepads() : [];

    for (var i = 0; i < gamepads.length; i++) {
        var pad = gamepads[i];

        if (pad) {
            // Send state to WASM
            gamedata.gp_action(i, "Left", pad.axes[0] < -0.5);
            gamedata.gp_action(i, "Right", pad.axes[0] > 0.5);
            gamedata.gp_action(i, "Up", pad.axes[1] < -0.5);
            gamedata.gp_action(i, "Down", pad.axes[1] > 0.5);
            gamedata.gp_action(i, "Button1", pad.buttons[0].pressed);
        }
    }
}

document.addEventListener('keydown', e => gamedata.action(e.key, true));
document.addEventListener('keyup', e => gamedata.action(e.key, false));
document.addEventListener("gamepadconnected", e => init_gamepads(e.gamepad));


document.addEventListener('keydown', e => console.log(e.key));



////////////////////////////////////////////////////////////////
// Play BGM
////////////////////////////////////////////////////////////////

var bgm = new Audio();
bgm.src = './audio/bgm.mp3';
bgm.volume = 0.06;
bgm.autoplay = true;
bgm.addEventListener("ended", function () {
    bgm.currentTime = 0;
    bgm.play();
}, false);


////////////////////////////////////////////////////////////////
// Update Screen
////////////////////////////////////////////////////////////////
const renderLoop = () => {
    scan_gamepads();
    gamedata.update();
    gamedata.draw();
    bgm.play();

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);