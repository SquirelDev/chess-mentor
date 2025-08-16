import { useState, useEffect } from 'react';
import { Chessboard } from 'react-chessboard';
import { Chess } from 'chess.js';
import { invoke } from '@tauri-apps/api/core';

const EnginePlayer = () => {
    console.log('--- EnginePlayer component re-rendered ---');
    const [game, setGame] = useState(new Chess());
    const [depth, setDepth] = useState(5);
    const [playerColor, setPlayerColor] = useState<'white' | 'black'>('white');
    const [isEngineThinking, setIsEngineThinking] = useState(false);

    useEffect(() => {
        console.log(`useEffect triggered. Player color: ${playerColor}, Game turn: ${game.turn()}`);
        if (game.turn() !== playerColor) {
            console.log('Engine\'s turn to move.');
            getEngineMove(game.fen());
        }
    }, [playerColor, game]);

    function handleNewGame() {
        console.log('--- handleNewGame called ---');
        const newGame = new Chess();
        setGame(newGame);
        if (playerColor === 'black') {
            console.log('Player is black, engine to move first.');
            setTimeout(() => getEngineMove(newGame.fen()), 250);
        }
    }

    async function getEngineMove(fen: string) {
        console.log(`--- getEngineMove called. FEN: ${fen}, Depth: ${depth} ---`);
        if (game.isGameOver()) {
            console.log('Game is over, not getting engine move.');
            return;
        }
        setIsEngineThinking(true);
        try {
            console.log('Invoking backend for engine move...');
            const bestMove = await invoke<string>('get_engine_move', { fen, depth: Number(depth) });
            console.log(`Backend returned best move: ${bestMove}`);
            const gameCopy = new Chess(fen);
            gameCopy.move(bestMove);
            setGame(gameCopy);
            console.log('Game state updated with engine move.');
        } catch (error) {
            console.error("!!! Failed to get engine move:", error);
        } finally {
            console.log('Engine thinking finished.');
            setIsEngineThinking(false);
        }
    }

    function onDrop({ sourceSquare, targetSquare }: { sourceSquare: string, targetSquare: string | null }) {
        console.log(`--- onDrop called. From: ${sourceSquare}, To: ${targetSquare} ---`);
        console.log(`Current state: isEngineThinking: ${isEngineThinking}, playerColor: ${playerColor}, game.turn(): ${game.turn()}`);

        if (!targetSquare) {
            console.log('Invalid drop: targetSquare is null.');
            return false;
        }
        if (isEngineThinking) {
            console.log('Invalid drop: Engine is thinking.');
            return false;
        }
        if (game.turn() !== playerColor) {
            console.log('Invalid drop: Not player\'s turn.');
            return false;
        }

        const gameCopy = new Chess(game.fen());
        try {
            console.log(`Attempting move: ${sourceSquare}-${targetSquare}`);
            const moveResult = gameCopy.move({
                from: sourceSquare,
                to: targetSquare,
                promotion: 'q',
            });

            if (moveResult === null) {
                console.log('Move is illegal (chess.js returned null).');
                return false;
            }

            console.log('Move successful, updating game state.');
            setGame(gameCopy);
            console.log('Requesting engine move in 250ms.');
            setTimeout(() => getEngineMove(gameCopy.fen()), 250);
            return true;
        } catch (error) {
            console.error('!!! Illegal move (chess.js threw an error):', error);
            return false;
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
                            <button className="button" onClick={() => { console.log('Setting player color to white'); setPlayerColor('white'); }}>White</button>
                            <button className="button" onClick={() => { console.log('Setting player color to black'); setPlayerColor('black'); }}>Black</button>
                        </div>
                        <button className="button" onClick={handleNewGame}>New Game</button>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default EnginePlayer;
