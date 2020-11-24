import { ClientState } from 'samosbor-client';

const pre = document.getElementById('game-canvas');
const highlighter = document.getElementById('mouse-highlighter');

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

highlighter.addEventListener('contextmenu', (e) => {
  e.preventDefault()
}, false)

function handleClientEvent(kind, event) {
  if (event.target.tagName !== 'SPAN') { return }
  let message = constructClientEvent(kind, event);

  let msg2server = window.clientState.handle_client_input(
    JSON.stringify(message)
  );

  if (msg2server != '') {
    window.ws.send(msg2server)
  }
}

function constructClientEvent (kind, event) {
  const x = [...event.target.parentElement.children].indexOf(event.target)
  const y = [...event.target.parentElement.parentElement.children].indexOf(event.target.parentElement)
  if (kind == 'wheel') {
    return {
      "Wheel" : {
        position: {
          x:x,
          y:y
        }
      }
    }
  } else if (kind == 'mousedown') {
    return {
      "MouseDown":{
        position: {
          x:x,
          y:y
        }
      }
    }
  } else if (kind == 'mouseup') {
    return {
      'MouseUp':{
        position:{
          x:x,
          y:y
        }
      }
    }
  } else if (kind == 'keydown') {
    return {
      'KeyDown':{
        key_code: event.keyCode
      }
    }
  } else if (kind == 'keyup') {
    return {
      'KeyUp':{
        key_code: event.keyCode
      }
    }
  }
}

document.addEventListener('keydown', (e) => {
  handleClientEvent('keydown', e)
})

document.addEventListener('keyup', (e) => {
  handleClientEvent('keyup', e)
})

highlighter.addEventListener('wheel', (event) => {
    handleClientEvent('wheel', event)
})

highlighter.addEventListener('mousedown', (event) => {
    handleClientEvent('mousedown', event)
})

highlighter.addEventListener('mouseup', (event) => {
    handleClientEvent('mouseup', event)
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

