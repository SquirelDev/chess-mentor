import { useState, useEffect } from 'react';
import { Chessboard } from 'react-chessboard';
import { Chess } from 'chess.js';
import { invoke } from '@tauri-apps/api/core';

interface Puzzle {
    puzzle_id: string;
    fen: string;
    moves: string;
    rating: number;
    rating_deviation: number;
    popularity: number;
    nb_plays: number;
    themes: string;
    game_url: string;
    opening_tags: string;
}

const PuzzleTrainer = () => {
    const [puzzle, setPuzzle] = useState<Puzzle | null>(null);
    const [game, setGame] = useState(new Chess());
    const [level, setLevel] = useState('Easy');
    const [solution, setSolution] = useState<string[]>([]);
    const [moveIndex, setMoveIndex] = useState(0);
    const [message, setMessage] = useState('');
    const [loading, setLoading] = useState(false);

    useEffect(() => {
        if (puzzle) {
            const newGame = new Chess(puzzle.fen);
            setGame(newGame);
            setSolution(puzzle.moves.split(' '));
            setMoveIndex(0);
            setMessage('');

            // Play the first move of the puzzle (opponent's move)
            setTimeout(() => {
                try {
                    const firstMove = puzzle.moves.split(' ')[0];
                    const from = firstMove.substring(0, 2);
                    const to = firstMove.substring(2, 4);

                    const gameAfterMove = new Chess(puzzle.fen);
                    gameAfterMove.move({ from, to, promotion: 'q' });

                    setGame(gameAfterMove);
                    setMoveIndex(1);
                } catch (error) {
                    console.error("Error making first puzzle move:", error);
                    setMessage("Failed to play the puzzle's starting move.");
                }
            }, 500);
        }
    }, [puzzle]);

    const fetchPuzzle = async () => {
        setLoading(true);
        setMessage('');
        try {
            const newPuzzle = await invoke<Puzzle>('get_random_puzzle', { level });
            setPuzzle(newPuzzle);
        } catch (error) {
            console.error('Failed to fetch puzzle:', error);
            setMessage(`Failed to fetch puzzle: ${error}`);
        } finally {
            setLoading(false);
        }
    };

    const onDrop = ({ sourceSquare, targetSquare }: { sourceSquare: string, targetSquare: string | null }) => {
        if (!targetSquare || !puzzle || moveIndex >= solution.length) {
            return false;
        }

        const gameCopy = new Chess(game.fen());
        try {
            gameCopy.move({
                from: sourceSquare,
                to: targetSquare,
                promotion: 'q',
            });
        } catch (error) {
            return false;
        }

        const expectedMove = solution[moveIndex];
        const playerMove = `${sourceSquare}${targetSquare}`;

        if (playerMove === expectedMove) {
            setGame(gameCopy);
            setMoveIndex(moveIndex + 1);
            setMessage('Correct!');

            if (moveIndex + 1 < solution.length) {
                setTimeout(() => {
                    const opponentMove = solution[moveIndex + 1];
                    const from = opponentMove.substring(0, 2);
                    const to = opponentMove.substring(2, 4);
                    gameCopy.move({ from, to, promotion: 'q' });
                    setGame(new Chess(gameCopy.fen()));
                    setMoveIndex(moveIndex + 2);
                }, 500);
            } else {
                setMessage('Puzzle complete!');
            }
            return true;
        } else {
            setMessage('Incorrect move, try again.');
            return false;
        }
    };

    return (
        <div className="trainer-container">
            <div className="chessboard-area">
                <h2>Puzzle Trainer</h2>
                <div style={{ width: '450px', maxWidth: '100%' }}>
                    <Chessboard options={{ position: game.fen(), onPieceDrop: onDrop }} />
                </div>
                {loading && <p className="status-message">Loading puzzle...</p>}
                {message && <p className="status-message">{message}</p>}
            </div>
            <div className="info-area">
                <div className="card">
                    <h3>Controls</h3>
                    <div className="controls">
                        <label htmlFor="level-select">Difficulty: </label>
                        <select id="level-select" className="select" value={level} onChange={(e) => setLevel(e.target.value)}>
                            <option value="Easy">Easy</option>
                            <option value="Medium">Medium</option>
                            <option value="Hard">Hard</option>
                        </select>
                        <button className="button" onClick={fetchPuzzle}>New Puzzle</button>
                    </div>
                </div>
                <div className="card">
                    <h3>Puzzle Info</h3>
                    {puzzle && !loading ? (
                        <div className="puzzle-info">
                            <p><strong>Rating:</strong> {puzzle.rating}</p>
                            <p><strong>Themes:</strong> {puzzle.themes}</p>
                            <p><strong>Played:</strong> {puzzle.nb_plays.toLocaleString()} times</p>
                            <p><strong>From Game:</strong> <a href={puzzle.game_url} target="_blank" rel="noopener noreferrer">View on Lichess</a></p>
                        </div>
                    ) : !loading && (
                        <p>Click "New Puzzle" to begin.</p>
                    )}
                </div>
            </div>
        </div>
    );
};

export default PuzzleTrainer;
