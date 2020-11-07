import { ClientState } from 'samosbor-client';

const pre = document.getElementById('game-canvas');
const highlighter = document.getElementById('mouse-highlighter');

const directions = {
  37: 'l',
  39: 'r',
  38: 'u',
  40: 'd'
}

const mouseHighlighterSize = {
  x: 0,
  y: 0
}

function fillMouseHighlighter (text) {
  const lines = text.trim().split('\n')
  const logicalSize = {
    x: lines[0].length,
    y: lines.length
  }

  if (mouseHighlighterSize.x === logicalSize.x && mouseHighlighterSize.y === logicalSize.y) {
    return
  }

  mouseHighlighterSize.x = logicalSize.x
  mouseHighlighterSize.y = logicalSize.y

  highlighter.innerHTML = ''

  for (let y = 0; y < logicalSize.y; y++) {
    const line = document.createElement('div')
    for (let x = 0; x < logicalSize.x; x++) {
      const span = document.createElement('span')
      span.textContent = ' '
      line.appendChild(span)
    }
    highlighter.appendChild(line)
  }
}

document.addEventListener('keydown', (e) => {
    console.log('[keyboard]', e)
    if (e.keyCode in directions) {
        const d = directions[e.keyCode]
        let event = window.clientState.step(d);
        window.ws.send(event);
    }
})

highlighter.addEventListener('contextmenu', (e) => {
  e.preventDefault()
}, false)

function handleMouseEvent(kind, e) {
  if (e.target.tagName !== 'SPAN') { return }
  const x = [...e.target.parentElement.children].indexOf(e.target)
  const y = [...e.target.parentElement.parentElement.children].indexOf(e.target.parentElement)
  console.log('[mouse]', kind, { x, y })
}

highlighter.addEventListener('wheel', (e) => {
  handleMouseEvent(e.deltaY, e)
})

highlighter.addEventListener('mousedown', (e) => {
  handleMouseEvent(e.buttons, e)
})

async function runRenderLoop () {
    const renderLoop = () => {
        if (window.clientState) {
            pre.textContent = window.clientState.render()
            fillMouseHighlighter(pre.textContent)
        } else {
            pre.textContent = ""
            fillMouseHighlighter('')
        }
        setTimeout(() => {
            requestAnimationFrame(renderLoop)
        }, 30) // some slowdown to chill the CPU
    }
    requestAnimationFrame(renderLoop)
}

async function initiateConnection () {
    const socket = new WebSocket(process.env.WS_CONNECT_STRING)
    window.ws = socket

    socket.onmessage = event => {
        if (window.clientState) {
            console.log(`[message] Data received from server: ${event.data}`)
            const msg = window.clientState.eval_message(event.data)
            if (msg != '') socket.send(msg);
        } else {
            const clientState = ClientState.from_server_response(event.data)
            window.clientState = clientState
        }
    }
}

runRenderLoop()
initiateConnection()

