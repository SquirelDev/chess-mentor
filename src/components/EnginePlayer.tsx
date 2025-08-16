import { useState, useEffect } from 'react';
import { Chessboard } from 'react-chessboard';
import { Chess } from 'chess.js';
import { invoke } from '@tauri-apps/api/core';

const EnginePlayer = () => {
    const [game, setGame] = useState(new Chess());
    const [depth, setDepth] = useState(5);
    const [playerColor, setPlayerColor] = useState<'white' | 'black'>('white');
    const [isEngineThinking, setIsEngineThinking] = useState(false);

    useEffect(() => {
        // If it's the engine's turn to start, get a move.
        if (game.turn() !== playerColor) {
            getEngineMove(game.fen());
        }
    }, [playerColor, game]); // Also run when game state changes and it's engine's turn

    function handleNewGame() {
        const newGame = new Chess();
        setGame(newGame);
        if (playerColor === 'black') {
            setTimeout(() => getEngineMove(newGame.fen()), 250);
        }
    }

    async function getEngineMove(fen: string) {
        if (game.isGameOver()) return;
        setIsEngineThinking(true);
        try {
            const bestMove = await invoke<string>('get_engine_move', { fen, depth: Number(depth) });
            const gameCopy = new Chess(fen);
            gameCopy.move(bestMove);
            setGame(gameCopy);
        } catch (error) {
            console.error("Failed to get engine move:", error);
        } finally {
            setIsEngineThinking(false);
        }
    }

    function onDrop({ sourceSquare, targetSquare }: { sourceSquare: string, targetSquare: string | null }) {
        if (!targetSquare || game.turn() !== playerColor || isEngineThinking) {
            return false;
        }

        const gameCopy = new Chess(game.fen());
        try {
            const moveResult = gameCopy.move({
                from: sourceSquare, // Corrected variable
                to: targetSquare,   // Corrected variable
                promotion: 'q',
            });

            // This check is redundant for chess.js v1 which throws, but good for safety.
            if (moveResult === null) {
                return false;
            }

            setGame(gameCopy);
            setTimeout(() => getEngineMove(gameCopy.fen()), 250);
            return true;
        } catch (error) {
            return false; // Catches exceptions from chess.js for illegal moves
        }
    }

    return (
        <div className="trainer-container">
            <div className="chessboard-area">
                <h2>Play vs. Engine</h2>
                <div style={{ width: '450px', maxWidth: '100%' }}>
                    <Chessboard
                        options={{
                            position: game.fen(),
                            onPieceDrop: onDrop,
                            boardOrientation: playerColor,
                            arePiecesDraggable: !isEngineThinking,
                        }}
                    />
                </div>
                {isEngineThinking && <p className="status-message">Engine is thinking...</p>}
            </div>
            <div className="info-area">
                <div className="card">
                    <h3>Engine Controls</h3>
                    <div className="controls vertical">
                        <div className="control-group">
                            <label htmlFor="depth-select">Engine Depth: </label>
                            <select id="depth-select" className="select" value={depth} onChange={(e) => setDepth(Number(e.target.value))}>
                                <option value={2}>2</option>
                                <option value={5}>5 (Medium)</option>
                                <option value={8}>8</option>
                                <option value={12}>12 (Hard)</option>
                                <option value={15}>15</option>
                            </select>
                        </div>
                         <div className="control-group">
                            <label>Play as: </label>
                            <button className="button" onClick={() => setPlayerColor('white')}>White</button>
                            <button className="button" onClick={() => setPlayerColor('black')}>Black</button>
                        </div>
                        <button className="button" onClick={handleNewGame}>New Game</button>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default EnginePlayer;
