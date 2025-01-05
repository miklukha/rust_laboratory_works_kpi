// // document.addEventListener('DOMContentLoaded', () => {
// //   // Отримати токен і ім'я користувача з localStorage
// //   const token = localStorage.getItem('token');
// //   const username = localStorage.getItem('username');

// //   if (!token || !username) {
// //     alert('Token or username not found. Please log in again.');
// //     window.location.href = 'login.html';
// //     return;
// //   }

// //   // Створити URL для WebSocket
// //   const wsUrl = `ws://${window.location.hostname}:8080/ws?token=${token}`;
// //   const ws = new WebSocket(wsUrl);

// //   // Отримати елементи DOM
// //   const chatForm = document.getElementById('chat-form');
// //   const messageInput = document.getElementById('message');
// //   const messagesDiv = document.getElementById('messages');

// //   if (!chatForm || !messageInput || !messagesDiv) {
// //     console.error(
// //       'Chat form, message input, or messages container not found in the DOM',
// //     );
// //     return;
// //   }

// //   // Обробка події відкриття WebSocket-з'єднання
// //   ws.onopen = () => {
// //     console.log('WebSocket connection established');
// //   };

// //   // Обробка отримання повідомлення через WebSocket
// //   ws.onmessage = event => {
// //     try {
// //       const data = JSON.parse(event.data);
// //       const newMessage = document.createElement('div');
// //       newMessage.textContent = `${data.username}: ${data.message}`;
// //       messagesDiv.appendChild(newMessage);
// //       messagesDiv.scrollTop = messagesDiv.scrollHeight; // Прокрутка вниз
// //     } catch (e) {
// //       console.error('Error parsing WebSocket message:', e);
// //     }
// //   };

// //   // Обробка закриття WebSocket-з'єднання
// //   ws.onclose = event => {
// //     console.error('WebSocket connection closed', event);
// //     alert('WebSocket connection closed. Please refresh the page.');
// //   };

// //   // Обробка помилок WebSocket
// //   ws.onerror = error => {
// //     console.error('WebSocket error:', error);
// //   };

// //   // Обробка відправки повідомлення
// //   chatForm.addEventListener('submit', e => {
// //     e.preventDefault();
// //     const message = messageInput.value.trim();
// //     if (message) {
// //       const payload = {
// //         username: username,
// //         message: message,
// //       };

// //       try {
// //         ws.send(JSON.stringify(payload)); // Відправити повідомлення через WebSocket
// //         messageInput.value = ''; // Очистити поле введення
// //       } catch (e) {
// //         console.error('Error sending message:', e);
// //       }
// //     }
// //   });
// // });

// // document.addEventListener('DOMContentLoaded', () => {
// //   const clientId = Math.random().toString(36).substring(2, 15);

// //   const username = localStorage.getItem('username');

// //   const ws = new WebSocket('ws://127.0.0.1:3000/ws');

// //   const messages = document.getElementById('messages');
// //   const input = document.getElementById('input');
// //   const sendBtn = document.getElementById('send');

// //   ws.onopen = () => {
// //     console.log('WebSocket connection established');
// //   };

// //   ws.onmessage = event => {
// //     console.log(event);

// //     try {
// //       if (!event.data) {
// //         console.warn('Received empty message');
// //         return;
// //       }

// //       if (event.data.trim() === 'connected') {
// //         console.log('Received server status message: connected');
// //         return;
// //       }

// //       const data = JSON.parse(event.data);

// //       const messageDiv = document.createElement('div');
// //       messageDiv.className = 'message';

// //       if (data.clientId === clientId) {
// //         messageDiv.classList.add('self');
// //       } else {
// //         messageDiv.classList.add('other');
// //       }

// //       messageDiv.textContent = `${data.username}: ${data.message}`;
// //       messages.appendChild(messageDiv);
// //       messages.scrollTop = messages.scrollHeight;
// //     } catch (e) {
// //       console.error(
// //         'Error parsing WebSocket message:',
// //         e,
// //         'Received:',
// //         event.data,
// //       );
// //     }
// //   };

// //   ws.onclose = event => {
// //     console.error('WebSocket connection closed:', event);
// //     alert('WebSocket connection closed. Please refresh the page.');
// //   };

// //   ws.onerror = error => {
// //     console.error('WebSocket error:', error);
// //   };

// //   input.addEventListener('keypress', e => {
// //     console.log(input);
// //     if (e.key === 'Enter' && input.value) {
// //       const messageData = {
// //         clientId: clientId,
// //         message: input.value,
// //       };
// //       ws.send(JSON.stringify(messageData));
// //       input.value = '';
// //     }
// //   });

// //   sendBtn.addEventListener('click', e => {
// //     const messageData = {
// //       clientId: clientId,
// //       message: input.value,
// //       username: username,
// //     };
// //     console.log(messageData);
// //     ws.send(JSON.stringify(messageData));
// //     input.value = '';
// //   });
// // });

// // document.addEventListener('DOMContentLoaded', () => {
// //   const clientId = Math.random().toString(36).substring(2, 15);
// //   const username = localStorage.getItem('username') || 'Anonymous';

// //   const ws = new WebSocket('ws://127.0.0.1:3000/ws');

// //   const messages = document.getElementById('messages');
// //   const input = document.getElementById('input');
// //   const sendBtn = document.getElementById('send');

// //   ws.onopen = () => {
// //     console.log('WebSocket connection established');
// //   };

// //   ws.onmessage = event => {
// //     try {
// //       const data = JSON.parse(event.data);

// //       const messageDiv = document.createElement('div');
// //       messageDiv.className = 'message';

// //       if (data.clientId === clientId) {
// //         messageDiv.classList.add('self');
// //       } else {
// //         messageDiv.classList.add('other');
// //       }

// //       messageDiv.textContent = `${data.username}: ${data.message}`;
// //       messages.appendChild(messageDiv);
// //       messages.scrollTop = messages.scrollHeight;
// //     } catch (e) {
// //       console.error('Error parsing WebSocket message:', e, event.data);
// //     }
// //   };

// //   ws.onclose = () => {
// //     alert('WebSocket connection closed. Please refresh the page.');
// //   };

// //   sendBtn.addEventListener('click', () => {
// //     if (input.value.trim()) {
// //       const messageData = {
// //         clientId,
// //         username,
// //         message: input.value.trim(),
// //       };
// //       ws.send(JSON.stringify(messageData));
// //       input.value = '';
// //     }
// //   });
// // });

// document.addEventListener('DOMContentLoaded', () => {
//   const clientId = Math.random().toString(36).substring(2, 15);
//   const username = localStorage.getItem('username') || 'Anonymous';

//   const ws = new WebSocket('ws://127.0.0.1:3000/ws');

//   const messages = document.getElementById('messages');
//   const input = document.getElementById('input');
//   const sendBtn = document.getElementById('send');

//   ws.onopen = () => {
//     console.log('WebSocket connection established');
//   };

//   ws.onmessage = event => {
//     try {
//       const message = event.data.trim();

//       // Перевірка, чи це дійсний JSON
//       if (message === 'connected') {
//         console.log('Server status: connected');
//         return;
//       }

//       const data = JSON.parse(message);

//       const messageDiv = document.createElement('div');
//       messageDiv.className = 'message';

//       if (data.clientId === clientId) {
//         messageDiv.classList.add('self');
//       } else {
//         messageDiv.classList.add('other');
//       }

//       messageDiv.textContent = `${data.username}: ${data.message}`;
//       messages.appendChild(messageDiv);
//       messages.scrollTop = messages.scrollHeight;
//     } catch (e) {
//       console.error('Error parsing WebSocket message:', e, event.data);
//     }
//   };

//   ws.onclose = () => {
//     alert('WebSocket connection closed. Please refresh the page.');
//   };

//   // sendBtn.addEventListener('click', () => {
//   //   if (input.value.trim()) {
//   //     const messageData = {
//   //       clientId,
//   //       username,
//   //       message: input.value.trim(),
//   //     };
//   //     ws.send(JSON.stringify(messageData));
//   //     input.value = '';
//   //   }
//   // });
//   sendBtn.addEventListener('click', () => {
//     if (input.value.trim()) {
//       const messageData = {
//         clientId,
//         username,
//         message: input.value.trim(),
//       };
//       console.log('Sending message:', messageData); // Логування перед відправкою
//       ws.send(JSON.stringify(messageData));
//       input.value = '';
//     }
//   });
// });
