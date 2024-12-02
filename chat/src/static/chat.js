const token = localStorage.getItem('token');
if (!token) {
  window.location.href = '/login.html';
}

const ws = new WebSocket(
  `ws://127.0.0.1:8080/ws?token=${encodeURIComponent(token)}`,
);
const messages = document.getElementById('messages');
const input = document.getElementById('input');

ws.onopen = () => {
  console.log('WebSocket connection established');
};

ws.onmessage = event => {
  const data = JSON.parse(event.data);
  const messageDiv = document.createElement('div');
  messageDiv.className = 'message';

  if (data.username === 'your_username') {
    messageDiv.classList.add('self');
  } else {
    messageDiv.classList.add('other');
  }

  messageDiv.textContent = `${data.username}: ${data.message}`;
  messages.appendChild(messageDiv);
  messages.scrollTop = messages.scrollHeight;
};

input.addEventListener('keypress', e => {
  if (e.key === 'Enter' && input.value) {
    ws.send(input.value);
    input.value = '';
  }
});
