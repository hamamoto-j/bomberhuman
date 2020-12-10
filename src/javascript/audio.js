export function play_audio(audio_name) {
    var audio = new Audio();
    audio.src = 'audio/' + audio_name + '.wav';
    audio.play();
}