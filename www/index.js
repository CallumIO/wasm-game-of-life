import { Universe } from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
var btn = document.getElementById("runSimulation");

const universe = Universe.new();

const renderLoop = () => {
  pre.textContent = universe.render();
  universe.next_iteration();
  requestAnimationFrame(renderLoop);
};

function runSim() {
  requestAnimationFrame(renderLoop);
}
pre.textContent = universe.render();
btn.addEventListener("click", runSim);
