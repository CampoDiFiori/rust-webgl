const rust = import('./pkg/rust_webgl.js');

// rust.then(m => m.say_hello_from_rust()).catch(console.error);

const canvas = document.getElementById('rustCanvas');
const gl = canvas.getContext('webgl', {antialias: true});

rust.then(m => {
  if (!gl) {
    alert('Failed initialize WebGl');
    return;
  }
 
  const client = new m.Client();
  const initialTime = Date.now();
  
  const FPS_THROTTLE = 1000.0 / 30.0;
  let lastDrawTime = -1;
  
  function render() {
    window.requestAnimationFrame(render);
    const currTime = Date.now();
    
    if (currTime >= lastDrawTime + FPS_THROTTLE) {
      lastDrawTime = currTime;
      
      if (window.innerHeight != canvas.height || window.innerWidth != canvas.width) {
        canvas.height = window.innerHeight;
        canvas.clientHeight = window.innerHeight;
        canvas.style.height = window.innerHeight;
        
        canvas.width = window.innerWidth;
        canvas.clientWidth = window.innerWidth;
        canvas.style.width = window.innerWidth;
        
        gl.viewport(0, 0, window.innerWidth, window.innerHeight);
      }
      
      let elapsedTime = currTime - initialTime;
      client.update(elapsedTime, window.innerHeight, window.innerWidth);
      client.render();
    }
  }
  
  render();
});