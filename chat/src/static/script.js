const clientId = Math.random().toString(36).substring(2, 15);

const ws = new WebSocket('ws://127.0.0.1:8080/ws');
const messages = document.getElementById('messages');
const input = document.getElementById('input');

ws.onmessage = event => {
  const data = JSON.parse(event.data);
  const messageDiv = document.createElement('div');
  messageDiv.className = 'message';

  if (data.clientId === clientId) {
    messageDiv.classList.add('self');
  } else {
    messageDiv.classList.add('other');
  }

  messageDiv.textContent = data.message;
  messages.appendChild(messageDiv);
  messages.scrollTop = messages.scrollHeight;
};

input.addEventListener('keypress', e => {
  if (e.key === 'Enter' && input.value) {
    const messageData = {
      clientId: clientId,
      message: input.value,
    };
    ws.send(JSON.stringify(messageData));
    input.value = '';
  }
});
