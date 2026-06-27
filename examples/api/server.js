// REST API example
// Run with: vikas run examples/api/server.js

console.log('🚀 Starting REST API example...');

let users = [
    { id: 1, name: 'Alice', email: 'alice@example.com' },
    { id: 2, name: 'Bob', email: 'bob@example.com' }
];
let nextId = 3;

console.log(`Loaded ${users.length} users`);

function listUsers() {
    return JSON.stringify({ users, total: users.length });
}

function getUser(id) {
    const user = users.find(u => u.id === id);
    if (!user) {
        return JSON.stringify({ error: 'User not found' });
    }
    return JSON.stringify({ user });
}

function createUser(name, email) {
    const user = {
        id: nextId++,
        name,
        email,
        createdAt: new Date().toISOString()
    };
    users.push(user);
    return JSON.stringify({ user });
}

console.log('API endpoints:');
console.log('  GET  /api/users');
console.log('  GET  /api/users/:id');
console.log('  POST /api/users');
console.log('  PUT  /api/users/:id');
console.log('  DELETE /api/users/:id');
console.log('  GET  /api/health');

console.log('Sample data:', listUsers());
