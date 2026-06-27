const message = document.querySelector('#message');
const button = document.querySelector('#button');

let count = 0;

message.textContent = 'This page is served by the Vikas.js Rust dev server.';

button.addEventListener('click', () => {
  count += 1;
  message.textContent = `Vikas.js handled ${count} browser click${count === 1 ? '' : 's'}.`;
});
