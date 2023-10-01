using ChessChallenge.API;
using Microsoft.CodeAnalysis;
using System;
using System.Collections.Generic;
using System.Linq;
using static System.Math;

public class MyBot : IChessBot
{
	const int PAWN = 100;
	const int CHECK_MATE_EVAL = PAWN * 1000;
	const int MAX_VAL = CHECK_MATE_EVAL * 100;
	const int SLACK = 5;
	const int IN_CHECK_PENALITY = 20;
	const int MOVE_COUNT_WEIGHT = 2;
	const int CENTER_PAWN_SCORE = 3;
	const int CENTER_WEIGHT = 8;
	const int CASTLE_VALUE = 50;
	const int MAX_TABLE_COUNT = 100_000_000;

	//Bitboards
	const ulong CenterSquares = 258704670720;
	const ulong DOUBLE_PAWN_CENTER_ATTACK_WHITE = 404226048;
	const ulong DOUBLE_PAWN_CENTER_ATTACK_BLACK = 26491358281728;
	const ulong PAWN_CENTER_ATTACK_WHITE = 606339072;
	const ulong PAWN_CENTER_ATTACK_BLACK = 39737037422592;

	//None, Pawn, Knight, Bishop, Rook, Queen, King
	int[] PIECE_WEIGHT = { 0, PAWN, 280, 320, 460, 910, CHECK_MATE_EVAL };
	int[] PAWN_PUSH_BONUS = { 0, 1, 3, 10, 20, 30, 50, 0 };

	Dictionary<ulong, int> book = new()
	{
		{  13227872743731781434, 16 }, //Start position, e4
		{  15607329186585411972, 16 },  //Start position + e4, e5
		{ 17664629573069429839,  11 }
	};

	Dictionary<ulong, int>[] bestMoveIndex = new Dictionary<ulong, int>[3];
	Dictionary<ulong, int>[] branchEval = new Dictionary<ulong, int>[20];

	class Comp : IComparer<(int, Move[])>
	{
		public int Compare((int, Move[]) x, (int, Move[]) y) => x.Item1.CompareTo(y.Item1);
	}

	static Comp comp = new();

	public MyBot()
	{
		for(int i = 0; i < bestMoveIndex.Length; i++)
		{
			bestMoveIndex[i] = new Dictionary<ulong, int>();
		}
	}

	public Move Think(Board board, Timer timer)
	{	
		Dictionary<ulong, int> evalTable = new();
		long MEM_BEFORE = GC.GetTotalMemory(true); 

		foreach (Dictionary<ulong, int> dic in bestMoveIndex)
			dic.Clear();

		var pair = (0, Move.NullMove);
		long posCounter = 0;
		var firstMoves = board.GetLegalMoves();

		long evalHitCounter = 0; 
		long checkEvalCounter = 0;

		if (firstMoves.Length == 1)
			return firstMoves[0];

		if (book.TryGetValue(board.ZobristKey, out var index))
		{
			return firstMoves[index];
		}

		int currentMaxDepth = 1;
		while (posCounter < 1_000_000 && !(Abs(pair.Item1) + 10 >= CHECK_MATE_EVAL))
		{
			AlphaBetaNega(-MAX_VAL, MAX_VAL, currentMaxDepth, 1, firstMoves, false);

			pair.Item1 *= board.IsWhiteToMove ? 1 : -1;
			board.Print();
   //         Console.WriteLine("BmCount: " + string.Join(" ", bestMoves.Select(x => x.Count)) + " matchCount: " + matchCount + " usefulCount: " + usefullCount);
			//Console.WriteLine("Memory: " + ((GC.GetTotalMemory(true) - MEM_BEFORE) / 1_000_000).ToString("000") + " mb");
			Console.Write("Depth: " + currentMaxDepth.ToString("00") + " StaticEvals: " + posCounter.ToString("000 000 000") + " EvalHits: " + ((double)evalHitCounter / posCounter) .ToString("0.000") + " Time: " + timer.MillisecondsElapsedThisTurn.ToString("000 000") + " Nodes/s: " + (posCounter * 1000 / (timer.MillisecondsElapsedThisTurn + 1)).ToString("0 000 000"));

			Console.Write(" Move: " + pair.Item2.GetSANString(board));
			
			Console.WriteLine(" Eval: " + (pair.Item1 / (float)PAWN).ToString("00.000"));

            currentMaxDepth++;
        }

		return pair.Item2;

		int AlphaBetaNega(int alpha, int beta, int depthLeft, int localEval, Move[] moves, bool quisccence)
		{
			//Finished result || Patt
            if (moves.Length == 0 || localEval == 0)
			{
				return localEval;
			}

			if (quisccence)
			{
				if (localEval >= beta)
					return beta;

				if (alpha < localEval)
					alpha = localEval;

				//Finished result || Patt
				if (moves.Length == 0 || localEval == 0 || depthLeft == 0)
					return localEval;
			}
			else
			{
				if (depthLeft == 0)
				{					
					return AlphaBetaNega(alpha, beta, SLACK, localEval, moves, true);
				}
			}

			int depth = currentMaxDepth - depthLeft;

            var evals = new (int, Move[])[moves.Length];
			int lookUpIndex = -1;

			if(depth < bestMoveIndex.Length && !quisccence)
			{
				if (bestMoveIndex[depth].TryGetValue(board.ZobristKey, out int index))
					lookUpIndex = index;
			}
			
			for (int i = 0; i < moves.Length; i++)
			{
				board.MakeMove(moves[i]);
				evals[i] = StaticEval(moves[i]);
				board.UndoMove(moves[i]);
			}
			

			//Smallest value first
			Array.Sort(evals, moves, comp);

			if(lookUpIndex >= 0)
			{
				var m = moves[lookUpIndex];

				//Console.WriteLine(new string('\t', maxDepth - depthLeft) + "N: " + m.GetSANString(board) + " [" + evals[i].Item2.Length + "] Eval: " + (evals[i].Item1 * (board.IsWhiteToMove ? -1 : 1)));
				board.MakeMove(m);

				var score = AlphaBetaNega(-beta, -alpha, depthLeft - (moves.Length == 1 ? 0 : 1), evals[lookUpIndex].Item1, evals[lookUpIndex].Item2, quisccence);
				score = -score;
				board.UndoMove(m);

				if (score >= beta)
				{
					//Console.WriteLine("Alpha beta break");
					return beta;   // fail hard beta-cutoff
				}

				if (score > alpha)
				{
					alpha = score; // alpha acts like max in MiniMax

					if (!quisccence && depthLeft == currentMaxDepth)
					{
						pair = (score, m);
					}
				}
			}



			for (int i = 0; i < moves.Length; i++) 
			{
				if (lookUpIndex == i)
					continue;

				var m = moves[i];

				if (quisccence && !m.IsCapture)
					continue;

                //Console.WriteLine(new string('\t', maxDepth - depthLeft) + "N: " + m.GetSANString(board) + " [" + evals[i].Item2.Length + "] Eval: " + (evals[i].Item1 * (board.IsWhiteToMove ? -1 : 1)));
                board.MakeMove(m);

				var score = AlphaBetaNega(-beta, -alpha, depthLeft - (moves.Length == 1 ? 0 : 1), evals[i].Item1, evals[i].Item2, quisccence);
				score = -score;
                board.UndoMove(m);

				if (score >= beta)
				{
                    //Console.WriteLine("Alpha beta break");
                    return beta;   // fail hard beta-cutoff
				}

				if (score > alpha)
				{
					alpha = score; // alpha acts like max in MiniMax

					if (!quisccence)
					{
						if (depth < bestMoveIndex.Length)
						{
							if (!bestMoveIndex[depth].TryAdd(board.ZobristKey, i))
							{
								bestMoveIndex[depth][board.ZobristKey] = i;
							}
						}

						if (depth == 0)
						{
							pair = (score, m);
						}
					}

				}
			}



			return alpha;

            //Returns positive value if next moving player is better
            (int, Move[]) StaticEval(Move lastMove)
			{
				posCounter++;  //#DEBUG

				var legalMoves = board.GetLegalMoves();
				var factor = board.IsWhiteToMove ? 1 : -1;

				if (board.IsDraw())
					return (0, legalMoves);

				if (board.IsInCheckmate())
					return (-(CHECK_MATE_EVAL - (currentMaxDepth - depthLeft)), legalMoves);

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

					int mCount = legalMoves.Where(x => x.MovePieceType != PieceType.Queen).Count();
					
					foreach(Move m in opponentMoves)
						if (m.MovePieceType != PieceType.Queen)
							mCount--;
					
					sum += mCount * factor * MOVE_COUNT_WEIGHT;

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

					checkEvalCounter++; //#DEBUG
                    //Console.WriteLine("This should not happen alot");
                }


				sum += CastleValue(true) - CastleValue(false);
				
				int CastleValue(bool white) => 
					(board.HasKingsideCastleRight(white) || board.HasQueensideCastleRight(white)) ? CASTLE_VALUE : 0;
				
				int KingPosValue(bool white) => (board.GetPieceBitboard(PieceType.King, white) & (white ? 0x44UL : 0x0000000000000044UL)) > 0 ? 1 : 0;

				sum += (KingPosValue(true) - KingPosValue(false)) * 100;
				
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