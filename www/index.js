import {Game} from "snake";

const canvas = document.getElementById("snake-canvas");
const ctx = canvas.getContext("2d");
let width = 800
let height = 600
let density = 25
const game = Game.new(width, height, density)
const borderAngle = [window.innerWidth/2-width/2,window.innerHeight/2 - height/2 ,width, height]

const renderLoop = () => {
  if (!game.get_gameover()) {
    game.update()
    drawField();
    drawScore(game.get_score());
    drawPlayer(game.get_player());
    drawFood(game.get_food());
  } else {
    drawGameover(game.get_score())
  }
}

const drawGameover = (score) =>{
  ctx.font = "80px Arial";
  ctx.fillStyle = "#BFBFBF";
  ctx.textAlign = "center";
  ctx.fillText("GameOver", borderAngle[0] + 400, 100);
  ctx.fillText("Final Score: " + score, borderAngle[0] + 400, 200);
}

const drawField = () => {
  ctx.font = "80px Arial";
  ctx.fillStyle = "#BFBFBF";
  ctx.textAlign = "center";
  ctx.fillText("SNAKE WASM", borderAngle[0] + 400, 100);
  ctx.beginPath();
  ctx.rect(...borderAngle);
  ctx.strokeStyle = "#FFFFFF";
  ctx.lineWidth = 5;
  ctx.stroke();
  for (let i = 1; i < (borderAngle[2] / density); i++) {
    ctx.beginPath();
    ctx.moveTo((i * density) + borderAngle[0], borderAngle[1]);
    ctx.lineTo((i * density) + borderAngle[0], borderAngle[1] + borderAngle[3]);
    ctx.strokeStyle = "#f6f6f6";
    ctx.lineWidth = 0.5;
    ctx.stroke();
  }
  for (let i = 1; i < (borderAngle[3] / density); i++) {
    ctx.beginPath();
    ctx.moveTo(borderAngle[0], (i * density) + borderAngle[1]);
    ctx.lineTo(borderAngle[0] + borderAngle[2], (i * density) + borderAngle[1]);
    ctx.strokeStyle = "#f6f6f6";
    ctx.lineWidth = 0.5;
    ctx.stroke();
  }
  
}

const drawScore = (score) =>{
  ctx.font = "30px Arial";
  ctx.fillStyle = "#BFBFBF";
  ctx.textAlign = "center";
  ctx.fillText("Score: " + score, borderAngle[0] + 400, borderAngle[1] + 700);
}

const drawPlayer = (player) => {
  for (let i = 0; i < player.get_len(); i++) {
    let point = player.get_point(i);
    drawSquare(point.get_x(), point.get_y())
  }

}

const drawSquare = (x,y, color = "#eeeeee") =>{
  ctx.beginPath();
  ctx.rect(borderAngle[0] + (density * x), borderAngle[1] + (density * y),
           density, density);
  ctx.fillStyle = color
  ctx.fill();
}

const drawFood = (food) => {
  drawSquare(food.get_x(), food.get_y(), "#FF0000")
}

document.onkeydown = (e) => {
  e = e || window.event;
  if (e != undefined && e.keyCode >= 37 && e.keyCode <= 40)
    game.move_it(e.keyCode)
}


setInterval(() => { 
width = window.innerWidth
height = window.innerHeight
canvas.width = width
canvas.height = height
  renderLoop()
},100)
