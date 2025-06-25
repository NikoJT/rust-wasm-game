import init, { move_down, move_right, move_up, move_left, get_sprite_position } from "./pkg/wasm_game.js";

async function main() {
  console.log("Starting init...");
  await init();
  console.log("WASM initialized!");

  const canvas = document.getElementById('canvas');
  const ctx = canvas.getContext('2d');

  const bgImage = new Image();
  bgImage.src = 'assets/big-bg.png';

  const spriteSheet = new Image();
  spriteSheet.src = 'assets/player_run.png';

  const frameWidth = 32;
  const frameHeight = 32;

  await Promise.all([
    new Promise(resolve => bgImage.onload = resolve),
    new Promise(resolve => spriteSheet.onload = resolve)
  ]);

  let lastTime = performance.now();

  const keysPressed = new Set();

  document.addEventListener('keydown', (e) => {
    keysPressed.add(e.key.toLowerCase());
  });

  document.addEventListener('keyup', (e) => {
    keysPressed.delete(e.key.toLowerCase());
  });

  function loop(timestamp) {
    const delta = (timestamp - lastTime) / 1000;
    lastTime = timestamp;

    if (keysPressed.has('w')) move_up(delta);
    if (keysPressed.has('a')) move_left(delta);
    if (keysPressed.has('s')) move_down(delta);
    if (keysPressed.has('d')) move_right(delta);

    const sprite = get_sprite_position();
    console.log(sprite.x, sprite.y, sprite.frame_index);
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.drawImage(bgImage, 0, 0, canvas.width, canvas.height);
    ctx.drawImage(
      spriteSheet,
      sprite.frame_index * frameWidth, 0, frameWidth, frameHeight,
      sprite.x, sprite.y, frameWidth, frameHeight
    );

    requestAnimationFrame(loop);
  }

  requestAnimationFrame(loop);
}

main();