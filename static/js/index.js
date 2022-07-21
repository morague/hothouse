

// buble panel selection with shape selector
const shape = document.querySelector('#shape-selector');
shape.addEventListener('change', () => {
    console.log('event accepted ')
    let shapeValue = document.getElementById('shape-selector').value;
    let rect = document.getElementById('rect-panel');
    let trap = document.getElementById('trap-panel');
    
    if (shapeValue == 'rect') {
        rect.style.display = 'flex';
        trap.style.display = 'none';
    } else if (shapeValue == 'trap') {
        rect.style.display = 'none';
        trap.style.display = 'flex';
    }
})


