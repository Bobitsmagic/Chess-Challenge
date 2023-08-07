using ChessChallenge.API;
using System;
using System.Collections.Generic;
using System.Linq;
using static System.Math;

public class MyBot : IChessBot
{
	const int PAWN = 100;
	const int CHECK_MATE_EVAL = PAWN * 1000;
	int MAX_VAL = CHECK_MATE_EVAL * 100;
	int SLACK = 30;
	int IN_CHECK_PENALITY = 20;
	int MOVE_COUNT_WEIGHT = 2;
	int CENTER_PAWN_SCORE = 3;
	int CENTER_WEIGHT = 8;
	int CASTLE_VALUE = 50;
	int MAX_TABLE_COUNT = 10_000_000;

	//Bitboards
	ulong CenterSquares = 258704670720;
	ulong DOUBLE_PAWN_CENTER_ATTACK_WHITE = 404226048;
	ulong DOUBLE_PAWN_CENTER_ATTACK_BLACK = 26491358281728;
	ulong PAWN_CENTER_ATTACK_WHITE = 606339072;
	ulong PAWN_CENTER_ATTACK_BLACK = 39737037422592;

	//None, Pawn, Knight, Bishop, Rook, Queen, King
	int[] PIECE_WEIGHT = { 0, PAWN, 280, 320, 460, 910, CHECK_MATE_EVAL };
	int[] PAWN_PUSH_BONUS = { 0, 1, 3, 10, 20, 30, 50, 0};

	class Comp : IComparer<(int, Move[])>
	{
		public int Compare((int, Move[]) x, (int, Move[]) y) => x.Item1.CompareTo(y.Item1);
	}

	static Comp comp = new();

	public Move Think(Board board, Timer timer)
	{
		long MEM_BEFORE = GC.GetTotalMemory(true);
		
		Dictionary<ulong, int> evalTable = new Dictionary<ulong, int>();
		Dictionary<ulong, int> qTable = new Dictionary<ulong, int>();

		//key, (depth left, eval)
		//Dictionary<ulong, (int, int)> depthTable = new Dictionary<ulong, (int, int)>();

		//board.Print();

		var pair = (0, new List<Move>());
		long posCounter = 0;
		var firstMoves = board.GetLegalMoves();

		long evalHitCounter = 0;
		long quietCounter = 0;
		long qHitCounter = 0;
		long checkEvalCounter = 0;

		if(firstMoves.Length == 1)
			return firstMoves[0];
		
		int maxDepth = 1;
		while (posCounter < 1_000_000 && !(Abs(pair.Item1) + 10 >= CHECK_MATE_EVAL))
		{
			pair = AlphaBetaNega(-MAX_VAL, MAX_VAL, maxDepth, 1, firstMoves);
			maxDepth++;

			pair.Item1 *= board.IsWhiteToMove ? 1 : -1;

            //Console.WriteLine("Memory: " + (GC.GetTotalMemory(true) - MEM_BEFORE).ToString("000 000 000"));
			Console.Write("Depth: " + (maxDepth - 1).ToString("00") + " StaticEvals: " + posCounter.ToString("000 000 000") + " EvalHits: " + ((double)evalHitCounter / posCounter) .ToString("0.000") + " QHits: " + ((double)qHitCounter / quietCounter).ToString("0.000") + " Time: " + timer.MillisecondsElapsedThisTurn.ToString("000 000") + " Nodes/s: " + (posCounter * 1000 / (timer.MillisecondsElapsedThisTurn + 1)).ToString("0 000 000") + " Best Line: "); //#DEBUG
			PrintLine(); //#DEBUG
			Console.WriteLine(" Eval: " + (pair.Item1 / (float)PAWN).ToString("00.000")); //#DEBUG

            Console.WriteLine(checkEvalCounter);
        }
		

		void PrintLine()														//#DEBUG
		{																		//#DEBUG
			for (int i = 0; i < pair.Item2.Count; i++)                          //#DEBUG
			{																	//#DEBUG
				Console.Write(pair.Item2[i].GetSANString(board) + " ");			//#DEBUG
																				//#DEBUG
				board.MakeMove(pair.Item2[i]);									//#DEBUG
			}																	//#DEBUG
																				//#DEBUG
			for (int i = pair.Item2.Count - 1; i >= 0; i--)						//#DEBUG
			{																	//#DEBUG
				board.UndoMove(pair.Item2[i]);									//#DEBUG
			}																	//#DEBUG
		}																		//#DEBUG

		return pair.Item2[0];

		(int, List<Move>) AlphaBetaNega(int alpha, int beta, int depthLeft, int localEval, Move[] moves)
		{
			//Finished result || Patt
            if (moves.Length == 0 || localEval == 0)
			{
				//board.Print();
				return (localEval, new List<Move>());
			}

			if (depthLeft == 0)
			{
				ulong key = board.ZobristKey;
				quietCounter++;

				if (qTable.TryGetValue(board.ZobristKey, out var val))
				{
					qHitCounter++;
					return (val, new List<Move>());
				}

				var eval = Quiscence(alpha, beta, SLACK, localEval, moves);

				qTable.Add(key, eval);

				return (eval,  new List<Move>());
			}

            (int, Move[])[] evals = new (int, Move[])[moves.Length];

			for(int i = 0; i < moves.Length; i++)
			{
				board.MakeMove(moves[i]);

				evals[i] = StaticEval(moves[i]);

				board.UndoMove(moves[i]);
			}

			//Smallest value first
			Array.Sort(evals, moves, comp);

			List<Move> bestLine = new List<Move>();
			for (int i = 0; i < moves.Length; i++) //Reverse so it starts with best move
			{
				var m = moves[i];
                //Console.WriteLine(new string('\t', maxDepth - depthLeft) + "N: " + m.GetSANString(board) + " [" + evals[i].Item2.Length + "] Eval: " + (evals[i].Item1 * (board.IsWhiteToMove ? -1 : 1)));
                board.MakeMove(m);

				int kek = 1;

				if (evals[i].Item2.Length == 1)
					kek = 0;

				var (score, line) = AlphaBetaNega(-beta, -alpha, depthLeft - kek, evals[i].Item1, evals[i].Item2);
				score = -score;
                board.UndoMove(m);

				if (score >= beta)
				{
                    //Console.WriteLine("Alpha beta break");
                    return (beta, line);   //  fail hard beta-cutoff
				}

				if (score > alpha)
				{
					alpha = score; // alpha acts like max in MiniMax

					bestLine.Clear();
					bestLine.Add(m);
					bestLine.AddRange(line);
				}
			}

			return (alpha, bestLine);

            //TODO Static Exchange Evaluation
            int Quiscence(int alpha, int beta, int depthLeft, int localEval, Move[] moves)
			{
                if (localEval >= beta)
                    return beta;

                if (alpha < localEval)
                    alpha = localEval;

                //Finished result || Patt
                if (moves.Length == 0 || localEval == 0 || depthLeft == 0)
                    return localEval;

                (int, Move[])[] evals = new (int, Move[])[moves.Length];

                //Console.WriteLine("Legal moves: " + string.Join(" ", moves.Select(x => x.GetSANString(board))));

                for (int i = 0; i < moves.Length; i++)
                {
                    board.MakeMove(moves[i]);
                    evals[i] = StaticEval(moves[i]);
                    board.UndoMove(moves[i]);
                }

                Array.Sort(evals, moves, comp);

				for (int i = 0; i < moves.Length; i++)
				{
					var m = moves[i];

					if (!m.IsCapture) continue;

                   // Console.WriteLine(new string('\t', SLACK - depthLeft + maxDepth) + "Q: " + m.GetSANString(board) + " [" + evals[i].Item2.Where(x => x.IsCapture).Count() + "] Eval: " + (evals[i].Item1 * (board.IsWhiteToMove ? -1 : 1)));
                    board.MakeMove(m);

                    //Console.WriteLine("Legal moves: " + string.Join(" ", evals[i].Item2.Select(x => x.GetSANString(board))));
                    var score = -Quiscence(-beta, -alpha, depthLeft - 1, evals[i].Item1, evals[i].Item2);
                    board.UndoMove(m);

                    if (score >= beta)
						return beta;   //  fail hard beta-cutoff
                    
                    if (score > alpha)
						alpha = score; // alpha acts like max in MiniMax
                }

				return alpha;
            }

            //Returns positive value if next moving player is better
            (int, Move[]) StaticEval(Move lastMove)
			{
				posCounter++;

				//board.Print();

				var legalMoves = board.GetLegalMoves();
				var factor = board.IsWhiteToMove ? 1 : -1;

				if (board.IsDraw())
					return (0, legalMoves);

				if (board.IsInCheckmate())
					return (-(CHECK_MATE_EVAL - (maxDepth - depthLeft)), legalMoves);

				if (evalTable.TryGetValue(board.ZobristKey, out var val))
				{
					evalHitCounter++;
					return (val, legalMoves);
				}

				//Whites perspective
				int sum = 0;

				var pieceList = board.GetAllPieceLists();

				sum += pieceList[0].Sum(x => PAWN_PUSH_BONUS[x.Square.Rank]);
				sum -= pieceList[6].Sum(x => PAWN_PUSH_BONUS[7 - x.Square.Rank]);		

				for (int i = 0; i < 6; i++)
				{
					var cost = PIECE_WEIGHT[i + 1];
					sum += pieceList[i].Count * cost;
					sum -= pieceList[i + 6].Count * cost;
				}

				if (board.TrySkipTurn())
				{
					Span<Move> opponentMoves = stackalloc Move[218];
					board.GetLegalMovesNonAlloc(ref opponentMoves);
					board.UndoSkipTurn();

					sum += (legalMoves.Length - opponentMoves.Length) * factor * MOVE_COUNT_WEIGHT;

					//Center 
					int centerScore = 0;

					foreach (Move m in legalMoves)
						centerScore += CenterScore(m);
					
					foreach (Move m in opponentMoves)
						centerScore -= CenterScore(m);

					int CenterScore(Move m)
					{
						int index = m.TargetSquare.Index;
						return (index >= 26 && index < 30 ||
							index >= 34 && index < 38) &&
							(m.MovePieceType == PieceType.Knight || m.MovePieceType == PieceType.Bishop) ? 1 : 0;
					}

					ulong whitePawns = board.GetPieceBitboard(PieceType.Pawn, true);
					ulong blackPawns = board.GetPieceBitboard(PieceType.Pawn, false);

					centerScore += CENTER_PAWN_SCORE * (
						2 * BitboardHelper.GetNumberOfSetBits(whitePawns & DOUBLE_PAWN_CENTER_ATTACK_WHITE) +
						BitboardHelper.GetNumberOfSetBits(whitePawns & PAWN_CENTER_ATTACK_WHITE) -
						2 * BitboardHelper.GetNumberOfSetBits(blackPawns & DOUBLE_PAWN_CENTER_ATTACK_BLACK) -
						BitboardHelper.GetNumberOfSetBits(blackPawns & PAWN_CENTER_ATTACK_BLACK));

					sum += centerScore * CENTER_WEIGHT * factor;
				}
				else
				{
					sum -= IN_CHECK_PENALITY * factor;

					checkEvalCounter++;
                    //Console.WriteLine("This should not happen alot");
                }


				sum += CastleValue(true) - CastleValue(false);
				
				int CastleValue(bool white) => 
					(board.HasKingsideCastleRight(white) || board.HasQueensideCastleRight(white)) ? CASTLE_VALUE : 0;

				if(pieceList.Sum(x => x.Count) < 5)
					sum += EdgeDistance(board.GetKingSquare(true)) - EdgeDistance(board.GetKingSquare(false));
				

				int EdgeDistance(Square s) =>
					Min(Min(s.Rank, 7 - s.Rank), Min(s.File, 7 - s.File));
				

				sum *= factor;

				//Eval != Draw
				if (sum == 0)
					sum = 1;

				
				if(evalTable.Count < MAX_TABLE_COUNT)
					evalTable.Add(board.ZobristKey, sum);

				return (sum, legalMoves);
			}
		}	
	}
}