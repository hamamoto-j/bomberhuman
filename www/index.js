import {GameData} from "bomberhuman";


const gamedata = GameData.new();

document.addEventListener('keydown', e => gamedata.action(e.key, true));
document.addEventListener('keyup', e => gamedata.action(e.key, false));

const renderLoop = () => {    
    gamedata.update();
    gamedata.draw();
    
    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);