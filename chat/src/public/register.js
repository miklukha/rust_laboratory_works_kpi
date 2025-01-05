document.getElementById('register-form').addEventListener('submit', async e => {
  e.preventDefault();
  const email = document.getElementById('email').value;
  const password = document.getElementById('password').value;

  const response = await fetch('http://localhost:3000/signup', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ email, password }),
  });
  console.log(response);

  if (response.ok) {
    alert('Реєстрація успішна! Тепер ви можете увійти.');
    window.location.href = 'login.html';
  } else {
    const errorText = await response.text();
    alert(`Помилка реєстрації: ${errorText}`);
  }
});
