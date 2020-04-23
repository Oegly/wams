// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const rust = import('./pkg/index.js');

const canvas = document.createElement("canvas");
canvas.width = 1024;
canvas.height = 768;


const ctx = canvas.getContext("2d");

document.body.appendChild(canvas);

class Clock {
  constructor(tick_length) {
    this.start = performance.now();
    this.last = this.start;
    this.tick_count = 0;
    this.sleep = 0;
    this.tick_length = tick_length;
  }

  tick() {
    if (!(this.tick_count % 300)) {
      let now = performance.now();
      console.log(now, this.last, this.tick_length - (now - this.last));
      this.speak();
    }

    this.tick_count += 1;
    let now = performance.now();
    let nap = this.tick_length - (now - this.last);

    this.last = now;
    this.sleep += nap;
    return nap;
  }

  speak() {
    console.log("Tick #" + this.tick_count + ". Slept for a total of " + this.sleep + " seconds");
  }

}

//const clock = new Clock(1000/60);

const update = (game) => {
  let start = performance.now();

  if (game.update()) {
    window.requestAnimationFrame(() => game.render(ctx));
    window.setTimeout(() => update(game), 1000/60 - (performance.now() - start));
  } else {
    console.log("u ded");
  }
};

const init = (m) => {
  game = m.start();

  document.addEventListener("mousedown", (event) => {
    game.mouse_pressed();
  });

  document.addEventListener("mouseup", (event) => {
    game.mouse_released();
  });

  document.addEventListener("keydown", (event) => {
    game.pressed(event.keyCode);
  });

  document.addEventListener("keyup", (event) => {
    game.released(event.keyCode);
  });

  document.addEventListener("mousemove", (event) => {
    game.cursor_moved(event.layerX, event.layerY);
  });

  let clock = new Clock(1000/60)
  console.log();
  console.log(m.start());

  update(game, clock);
};

rust
  .then(m => init(m))
  .catch(console.error);
