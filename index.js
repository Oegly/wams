// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const rust = import('./pkg/index.js');

const canvas = document.createElement("canvas");
canvas.width = 1024;
canvas.height = 768;

const ctx = canvas.getContext("2d");

document.body.appendChild(canvas);

const resizeCanvas = () => {
  let chunkX = window.innerWidth / 4;
  let chunkY = window.innerHeight / 3;

  if (chunkX <= chunkY) {
    canvas.width = window.innerWidth;
    canvas.height = chunkX * 3;
  } else {
    canvas.width = chunkY * 4;
    canvas.height = window.innerHeight;
  }

  //ctx.scale(1024 / canvas.width, 768 / canvas.height);
  console.log(ctx.scaleWidth, ctx.scaleHeight);
};

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
    let nap = 1000/60 - (now - this.last);
    this.last = now;
    this.sleep += nap;
    
    //console.log(now - this.last);
    return nap;
  }

  speak() {
    console.log("Tick #" + this.tick_count + ". Slept for a total of " + this.sleep + " seconds");
  }
}

//const clock = new Clock(1000/60);

const update = (game, clock) => {
  clock.last = performance.now();

  if (game.update()) {
    window.requestAnimationFrame(() => game.render(ctx));
    window.setTimeout(() => update(game, clock), clock.tick());
  } else {
    game.clock.speak();
    console.log("u ded");
  }
};

async function init(m) {
  let p = new URLSearchParams(window.location.search);
  let level = p.has("level") ? p.get("level") : "game";
  let s = await fetch("./data/" + level + ".json").then(r => r.text());

  let game = m.start(s);

  window.addEventListener("resize", () => resizeCanvas(game));

  document.addEventListener("mousedown", (event) => {
    game.mouse_pressed();
  });

  document.addEventListener("mouseup", (event) => {
    game.mouse_released();
  });

  document.addEventListener("keydown", (event) => {
    game.pressed(event.key.toLowerCase());
  });

  document.addEventListener("keyup", (event) => {
    game.released(event.key.toLowerCase());
  });

  document.addEventListener("mousemove", (event) => {
    game.cursor_moved(event.layerX, event.layerY);
  });

  //var clock = new Clock(1000/60);
  update(game, new Clock(1000/60));
}

rust
  .then(m => init(m))
  .catch(console.error);
