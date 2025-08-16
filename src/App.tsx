import { useState } from 'react';
import PuzzleTrainer from './components/PuzzleTrainer';
import OpeningTrainer from './components/OpeningTrainer';
import './App.css';

function App() {
  const [activeTab, setActiveTab] = useState('puzzles');

  return (
    <main className="container">
      <h1>Chess Mentor</h1>
      <nav className="tab-nav">
        <button
          className={activeTab === 'puzzles' ? 'active' : ''}
          onClick={() => setActiveTab('puzzles')}
        >
          Puzzle Trainer
        </button>
        <button
          className={activeTab === 'openings' ? 'active' : ''}
          onClick={() => setActiveTab('openings')}
        >
          Opening Trainer
        </button>
      </nav>
      <div className="content">
        {activeTab === 'puzzles' && <PuzzleTrainer />}
        {activeTab === 'openings' && <OpeningTrainer />}
      </div>
    </main>
  );
}

export default App;
