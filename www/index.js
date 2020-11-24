import {GameData} from "bomberhuman";


const gamedata = GameData.new();

gamedata.add_player(4);

document.addEventListener('keydown', e => gamedata.action(e.key, true));
document.addEventListener('keyup', e => gamedata.action(e.key, false));


document.addEventListener('keydown', e => console.log(e.key));

const renderLoop = () => {    
    gamedata.update();
    gamedata.draw();
    
    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);