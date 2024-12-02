document.getElementById('register-form').addEventListener('submit', async e => {
  e.preventDefault();
  const username = document.getElementById('username').value;
  const password = document.getElementById('password').value;

  const response = await fetch('/register', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ username, password }),
  });

  if (response.ok) {
    alert('Реєстрація успішна! Тепер ви можете увійти.');
    window.location.href = '/login.html';
  } else {
    const errorText = await response.text();
    alert(`Помилка реєстрації: ${errorText}`);
  }
});
