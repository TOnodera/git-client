import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';
import GitLogVisualizer from './components/organizations/GitLogVisualizer';

function App() {
  return (
    <div className="container">
      <GitLogVisualizer />
    </div>
  );
}

export default App;
