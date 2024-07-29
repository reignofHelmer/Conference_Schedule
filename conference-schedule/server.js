const express = require('express');
const mongoose = require('mongoose');
const bodyParser = require('body-parser');

const app = express();
const port = 3000;

// Middleware
app.use(bodyParser.json());

// MongoDB connection
mongoose.connect('mongodb://localhost:27017/conference', { useNewUrlParser: true, useUnifiedTopology: true });

// Define a schema and model for sessions
const sessionSchema = new mongoose.Schema({
  title: String,
  speaker: String,
  time: String,
  duration: Number,
  description: String
});

const Session = mongoose.model('Session', sessionSchema);

// Routes
app.get('/', (req, res) => {
  res.send('Welcome to the Conference Schedule Management System');
});

// Add a session
app.post('/sessions', async (req, res) => {
  const session = new Session(req.body);
  await session.save();
  res.send(session);
});

// View all sessions
app.get('/sessions', async (req, res) => {
  const sessions = await Session.find();
  res.send(sessions);
});

// Update a session
app.put('/sessions/:id', async (req, res) => {
  const session = await Session.findByIdAndUpdate(req.params.id, req.body, { new: true });
  res.send(session);
});

// Delete a session
app.delete('/sessions/:id', async (req, res) => {
  await Session.findByIdAndDelete(req.params.id);
  res.send({ message: 'Session deleted' });
});

app.listen(port, () => {
  console.log(`Server running on port ${port}`);
});