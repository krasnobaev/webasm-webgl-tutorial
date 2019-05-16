import './style.sass';
var mod = null;

import('./pkg')
.then(module => {
  mod = module;
  redraw();
})
.catch(console.error);

window.onhashchange = redraw;

function redraw () {
  var sampleid = window.location.hash.split('-')[1];
  console.assert(!Number.isNaN(Number(sampleid)), 'incorrect url param');

  mod.drawwebgl(Number(sampleid));
};
