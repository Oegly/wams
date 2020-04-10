// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const rust = import('./pkg/index.js');

//import rust from './pkg';

console.log(rust);
const canvas = document.createElement("canvas");
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;


const ctx = canvas.getContext("2d");

document.body.appendChild(canvas);

let mouse_pressed = false;
let pointer = [0, 0];

document.addEventListener("mousedown", (event) => {
  mouse_pressed = true;
});

document.addEventListener("mouseup", (event) => {
  mouse_pressed = false;
});

document.addEventListener("mousemove", (event) => {
  pointer = [event.layerX, event.layerY];
  //console.log(pointer);
  //console.log(event);
});


const update = (game) => {
  if (game.update(mouse_pressed, pointer[0], pointer[1])) {
    window.requestAnimationFrame(() => game.render(ctx));
    window.setTimeout(() => update(game), 1000/60);
  } else {
    console.log("u ded");
  }
};

const init = (m) => {
  console.log(m);
  game = m.start();

  //game.update();
  //window.requestAnimationFrame(() => game.render(ctx));

  console.log(ctx);
  console.log(m.start());

  update(game);
};

rust
  .then(m => init(m))
  .catch(console.error);
