import { useState, useEffect } from 'react';
import { Chessboard } from 'react-chessboard';
import { Chess } from 'chess.js';
import { invoke } from '@tauri-apps/api/core';

interface Opening {
    eco: string;
    name: string;
    pgn: string;
    uci: string;
    epd: string;
}

const OpeningTrainer = () => {
    const [openings, setOpenings] = useState<Opening[]>([]);
    const [selectedOpening, setSelectedOpening] = useState<Opening | null>(null);
    const [game, setGame] = useState(new Chess());
    const [moveHistory, setMoveHistory] = useState<string[]>([]);
    const [currentMove, setCurrentMove] = useState(0);

    useEffect(() => {
        const fetchOpenings = async () => {
            try {
                const result = await invoke<Opening[]>('get_all_openings');
                setOpenings(result);
            } catch (error) {
                console.error('Failed to fetch openings:', error);
            }
        };
        fetchOpenings();
    }, []);

    useEffect(() => {
        if (selectedOpening) {
            const newGame = new Chess();
            const moves = selectedOpening.pgn.split(' ').filter(m => !m.endsWith('.'));
            setMoveHistory(moves);
            setCurrentMove(0);
            setGame(newGame);
        }
    }, [selectedOpening]);

    const handleOpeningChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
        const openingName = e.target.value;
        const opening = openings.find(o => o.name === openingName) || null;
        setSelectedOpening(opening);
    };

    const navigateMoves = (direction: 'next' | 'prev' | 'start' | 'end') => {
        const newGame = new Chess();
        let newMoveIndex = currentMove;

        if (direction === 'next' && currentMove < moveHistory.length) {
            newMoveIndex++;
        } else if (direction === 'prev' && currentMove > 0) {
            newMoveIndex--;
        } else if (direction === 'start') {
            newMoveIndex = 0;
        } else if (direction === 'end') {
            newMoveIndex = moveHistory.length;
        }

        for (let i = 0; i < newMoveIndex; i++) {
            newGame.move(moveHistory[i]);
        }

        setGame(newGame);
        setCurrentMove(newMoveIndex);
    };

    return (
        <div>
            <h2>Opening Trainer</h2>
            <div>
                <select onChange={handleOpeningChange} defaultValue="">
                    <option value="" disabled>Select an opening</option>
                    {openings.map(o => (
                        <option key={o.name} value={o.name}>
                            {o.eco} - {o.name}
                        </option>
                    ))}
                </select>
            </div>
            {selectedOpening && (
                <div style={{ width: '400px', marginTop: '1rem' }}>
                    <Chessboard position={game.fen()} />
                    <div>
                        <p><strong>{selectedOpening.name}</strong> ({selectedOpening.eco})</p>
                        <p>{selectedOpening.pgn}</p>
                        <div>
                            <button onClick={() => navigateMoves('start')}>&lt;&lt;</button>
                            <button onClick={() => navigateMoves('prev')}>&lt;</button>
                            <button onClick={() => navigateMoves('next')}>&gt;</button>
                            <button onClick={() => navigateMoves('end')}>&gt;&gt;</button>
                        </div>
                        <p>Move {currentMove} of {moveHistory.length}</p>
                    </div>
                </div>
            )}
        </div>
    );
};

export default OpeningTrainer;
