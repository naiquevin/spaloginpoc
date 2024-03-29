window.onload = async () => {
  const response = await fetch('/xhr/info');
  if (response.ok) {
    const data = await response.json();
    document.getElementById('greeting').innerHTML = `Hello, ${data.user}` + ' (<a href="/logout">logout</a>)';
  } else if (response.status == 401) {
    let errContainer = document.getElementById('error');
    errContainer.innerHTML = 'Please <a href="/login">login</a>';
  } else {
    console.log(`An unexpected error occured ${response}`);
  }
};

