// Basic HTTP server with Vikas.js
// Run with: vikas dev (from project root) or cargo run -p vikas-cli -- dev

console.log('🚀 Starting basic server example...');
console.log('Use: vikas dev --port 3000 to start the dev server');

const routes = {
    '/': () => ({
        status: 200,
        headers: { 'Content-Type': 'text/html' },
        body: '<h1>Hello from Vikas.js!</h1><p>This is a basic server example.</p>'
    }),
    '/api/hello': () => ({
        status: 200,
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            message: 'Hello World!',
            timestamp: new Date().toISOString()
        })
    }),
    '/api/users': () => ({
        status: 200,
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify([
            { id: 1, name: 'John Doe' },
            { id: 2, name: 'Jane Smith' },
            { id: 3, name: 'Bob Johnson' }
        ])
    })
};

console.log('Registered routes:', Object.keys(routes));
