document.getElementById('login-form').addEventListener('submit', async e => {
  e.preventDefault();
  const username = document.getElementById('username').value;
  const password = document.getElementById('password').value;

  // Відправляємо запит на сервер для автентифікації
  const response = await fetch('/login', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ username, password }),
  });

  if (response.ok) {
    const data = await response.json();
    // Зберігаємо токен у localStorage
    localStorage.setItem('token', data.token);
    // Перенаправляємо на сторінку чату
    window.location.href = '/chat.html';
  } else {
    const errorText = await response.text();
    alert(`Помилка входу: ${errorText}`);
  }
});
