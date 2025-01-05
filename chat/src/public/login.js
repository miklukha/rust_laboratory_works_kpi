document.getElementById('login-form').addEventListener('submit', async e => {
  e.preventDefault();
  const email = document.getElementById('email').value;
  const password = document.getElementById('password').value;

  const response = await fetch('http://localhost:3000/signin', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ email, password }),
  });
  console.log(response);

  if (response.ok) {
    const data = await response.json();
    console.log(data);
    // localStorage.setItem('token', data.token);
    // localStorage.setItem('username', username);
    // window.location.href = 'chat.html';
  } else {
    const errorText = await response.text();
    alert(`Помилка входу: ${errorText}`);
  }
});
