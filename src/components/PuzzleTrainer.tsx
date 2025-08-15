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
                const firstMove = puzzle.moves.split(' ')[0];
                const from = firstMove.substring(0, 2);
                const to = firstMove.substring(2, 4);
                newGame.move({ from, to, promotion: 'q' });
                setGame(new Chess(newGame.fen()));
                setMoveIndex(1);
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

    const onDrop = (sourceSquare: string, targetSquare: string) => {
        if (!puzzle || moveIndex >= solution.length) return false;

        const gameCopy = new Chess(game.fen());
        const move = gameCopy.move({
            from: sourceSquare,
            to: targetSquare,
            promotion: 'q', // always promote to a queen for simplicity
        });

        if (move === null) return false;

        const expectedMove = solution[moveIndex];
        const playerMove = `${sourceSquare}${targetSquare}`;

        if (playerMove === expectedMove) {
            setGame(gameCopy);
            setMoveIndex(moveIndex + 1);
            setMessage('Correct!');

            // If there's a next move for the opponent, play it
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
        <div>
            <h2>Puzzle Trainer</h2>
            <div>
                <label htmlFor="level-select">Difficulty: </label>
                <select id="level-select" value={level} onChange={(e) => setLevel(e.target.value)}>
                    <option value="Easy">Easy</option>
                    <option value="Medium">Medium</option>
                    <option value="Hard">Hard</option>
                </select>
                <button onClick={fetchPuzzle}>New Puzzle</button>
            </div>
            <div style={{ width: '400px', marginTop: '1rem' }}>
                <Chessboard position={game.fen()} onPieceDrop={onDrop} />
            </div>
            {puzzle && !loading && (
                <div>
                    <p>Rating: {puzzle.rating}</p>
                    <p>Themes: {puzzle.themes}</p>
                    <p>Opening Tags: {puzzle.opening_tags}</p>
                </div>
            )}
            {loading && <p>Loading puzzle...</p>}
            {message && <p>{message}</p>}
        </div>
    );
};

export default PuzzleTrainer;
