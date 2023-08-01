using ChessChallenge.API;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using static System.Math;

namespace ChessChallenge.Example
{
	// A simple bot that can spot mate in one, and always captures the most valuable piece it can.
	// Plays randomly otherwise.
	public class EvilBot : IChessBot
	{
		public Move Think(Board board, Timer timer)
		{
			return Move.NullMove;
			//Move m = SF(board, timer);
			Move m = BarschOld(board, timer);

			return m;
		}
		public Move SF(Board board, Timer timer)
		{
			var p = new Process();
			p.StartInfo.FileName = "C:\\Users\\hmart\\Desktop\\stockfish\\stockfish-windows-x86-64-avx2.exe";
			p.StartInfo.UseShellExecute = false;
			p.StartInfo.RedirectStandardInput = true;
			p.StartInfo.RedirectStandardOutput = true;

			p.Start();
			string setupString = "position fen " + board.GetFenString();
			p.StandardInput.WriteLine(setupString);

			const int skill = 5;
			const int time = 300;
			//setoption name SyzygyPath value C:\\Users\\hmart\\Documents\\GitHub\\Chess-Challenge\\stockfish\\Syzygy345 \n
			// Process for 5 seconds
			string processString = "setoption name Skill Level value " + skill + "\ngo movetime " + time;
			//string processString = "go";

			// Process 20 deep
			// string processString = "go depth 20";

			//p.StandardInput.WriteLine("");
			p.StandardInput.WriteLine(processString);

			string move;
			while (true)
			{
				string line = p.StandardOutput.ReadLine();
				//Console.WriteLine("SF: " + line);
				string[] split = line.Split(" ");
				if (split[0] == "bestmove")
				{
					move = split[1];
					break;
				}
			}

			string bestMoveInAlgebraicNotation = move;

			Console.WriteLine("Stockfish: " + bestMoveInAlgebraicNotation + " time: " + timer.MillisecondsElapsedThisTurn);


			p.Close();

			return new Move(bestMoveInAlgebraicNotation, board);

		}

		const int PAWN = 124;
		const int CHECKMATE_SCORE = PAWN * 1000;
		const int MIN_CHECKMATE_SCORE = CHECKMATE_SCORE * 7 / 8;
		const long MIN_BOARDS_SEARCHED = 1 << 14;
		const long MIN_BOARDS_PANIC = 1 << 10;
		const int START_SLACK = 3;
		int[] PIECE_VALUES = { PAWN, 781, 825, 1276, 2538, CHECKMATE_SCORE };

		int[] PawnBonusMidGame = new int[64];
		int[] PieceBonusMidGame = new int[160];
		Random rnd = new Random();

		//BBV2 Depth: 5 Positions: 000 432 693 Time: 1694 Line: [Move: 'h6g7' Move: 'g8g7' Move: 'h1h7' Move: 'g7h7' Move: 'f3f6' Move: 'h7g7' Move: 'a1h1' Eval: 994,000
		public EvilBot()
		{
			ulong[] mg = { 5348067507372035, 18445336728892014608, 4222176189349879, 18440551645698195488, 5629529597542396, 18444492291076849704, 562894118846477, 1688798320525323, 6473898693754885, 18444773688742772728, 18443366361104646137, 18444492321139458053 };

			for (int i = 8; i < 56; i++)
			{
				PawnBonusMidGame[i] = GetVal(i - 8) + PAWN;
			}

			//Console.WriteLine(string.Join(" ", PawnBonusMidGame));

			mg = new ulong[] { 18426477561847807825, 18442803312363831219, 3377729784250307, 13792445658103773, 14355412791721950, 14918422875275255, 10414595611426749, 18439706963063209783, 18440551594156883915, 1125981511811057, 5066528107397113, 10977631466684411, 8725818769276916, 3096229039243248, 25768951791, 18440551564092506064, 18445618117966888929, 1970294771417067, 1125899906187239, 18445336685940834291, 1125887021023205, 3377729785233386, 5066618301120510, 2814749765926895, 1407357703356419, 3377734080659453, 1970380672008189, 2251838468718596, 1407426424078336, 2251825584209916, 2251842763816955, 18446181132345999358, 55733209346277647, 50385025873412374, 33777723071660227, 27585140435583140, 19703699353043098, 8726072179884155, 9288953412190296, 18446462792012202043 };

			for (int i = 0; i < 160; i++)
			{
				PieceBonusMidGame[i] = GetVal(i) + PIECE_VALUES[1 + (i / 32)];
			}

			//Console.WriteLine(string.Join(" ", PieceBonusMidGame));

			short GetVal(int index)
			{
				return (short)((mg[index / 4] >> (16 * (index % 4))) & ushort.MaxValue);
			}
		}

		//BBV2 Depth: 5 Positions: 000 996 612 Time: 3139 Line: [Move: 'h6g7' Move: 'g8g7' Move: 'h1h7' Move: 'g7h7' Move: 'f3f6' Move: 'h7g7' Move: 'a1h1' Eval: 994,000
		public Move BarschOld(Board board, Timer timer)
		{

			Dictionary<ulong, int> evalTable = new Dictionary<ulong, int>();
			long posCounter = 0;
			var pair = (0, new List<Move>() { board.GetLegalMoves()[0] });
			int maxdepth = 0;

			int bestVal = board.IsWhiteToMove ? int.MinValue : int.MaxValue;

			if (board.GetLegalMoves().Length == 1)
				return board.GetLegalMoves()[0];

			var answer = Move.NullMove;

			//List<Move> bestLine = new List<Move>();
			//Stack<Move> stack = new Stack<Move>();

			while (posCounter < MIN_BOARDS_SEARCHED && !(Abs(pair.Item1) > MIN_CHECKMATE_SCORE) && !(timer.MillisecondsRemaining < 3000 && posCounter > MIN_BOARDS_PANIC))
			{
				pair = DFS(0, int.MinValue, int.MaxValue, 0, START_SLACK);

				maxdepth++;
			}

			Console.WriteLine("BBV2 Depth: " + maxdepth + " Positions: " + posCounter.ToString("000 000 000") + " Time: " + timer.MillisecondsElapsedThisTurn + " Nodes/s: " + (posCounter * 1000 / (timer.MillisecondsElapsedThisTurn + 1)).ToString("000 000") + " Line: [" + string.Join(" ", pair.Item2) + " Eval: " + (pair.Item1 / (float)PAWN).ToString("00.000"));

			return pair.Item2[0];

			(int, List<Move>) DFS(int depth, int alpha, int beta, int eval, int slack)
			{
				posCounter++;

				Move[] moves = board.GetLegalMoves();

				var maximize = board.IsWhiteToMove;
				var factor = maximize ? -1 : 1;

				if (board.IsInCheckmate() || board.IsDraw())
					return (eval, new List<Move>() { Move.NullMove });

				if (depth >= maxdepth)
					return (eval, new List<Move>() { });

				int best = maximize ? int.MinValue : int.MaxValue;
				List<Move> bestLine = new List<Move>();

				(int, Move)[] next = moves.Select(x =>
				{
					board.MakeMove(x);
					var eval = Eval();
					board.UndoMove(x);

					return (eval, x);
				}).ToArray();

				//TODO optimize
				Array.Sort(next, (x, y) => factor * x.Item1.CompareTo(y.Item1));

				if (Abs(next[0].Item1) > MIN_CHECKMATE_SCORE)
					return (next[0].Item1, new List<Move>() { next[0].Item2 });


				foreach (var (local, move) in next)
				{

					board.MakeMove(move);

					//stack.Push(move);
					int extraDepth = 1;
					if (slack > 0 && (board.IsInCheck() || move.IsCapture))
					{
						extraDepth--;
					}

					var (val, line) = DFS(depth + extraDepth, alpha, beta, local, slack - 1 + extraDepth);

					if (maximize)
					{
						if (val > best)
						{
							best = val;
							bestLine.Clear();
							bestLine.Add(move);
							bestLine.AddRange(line);
						}


						alpha = Max(alpha, best);
					}
					else
					{
						if (val < best)
						{
							best = val;

							bestLine.Clear();
							bestLine.Add(move);
							bestLine.AddRange(line);
						}

						beta = Min(beta, best);
					}

					board.UndoMove(move);
					//stack.Pop();

					if (beta <= alpha)
						break;
				}

				return (best, bestLine);

				int Eval()
				{
					var factor = board.IsWhiteToMove ? -1 : 1;

					if (board.IsInCheckmate())
						return factor * (CHECKMATE_SCORE - (depth + (START_SLACK - slack)) * PAWN);

					if (board.IsDraw())
						return 0;

					if (evalTable.TryGetValue(board.ZobristKey, out var val))
					{
						return val;
					}

					int sum = CountPieces();

					int enemryMovesLength = 0;
					board.MakeMove(Move.NullMove);
					enemryMovesLength = board.GetLegalMoves().Length;
					board.UndoMove(Move.NullMove);

					sum -= factor * ((board.GetLegalMoves().Length - enemryMovesLength));

					sum += EdgeDistance(board.GetKingSquare(true)) - EdgeDistance(board.GetKingSquare(false));

					int EdgeDistance(Square s)
					{
						return Min(Min(s.Rank, 7 - s.Rank), Min(s.File, 7 - s.File));
					}

					//sum += rnd.Next(2);

					evalTable.Add(board.ZobristKey, sum);

					return sum;

					//TODO Late game values
					int CountPieces()
					{
						/// Pawns(white), Knights (white), Bishops (white), Rooks (white), Queens (white), King (white),
						/// Pawns (black), Knights (black), Bishops (black), Rooks (black), Queens (black), King (black)

						var list = board.GetAllPieceLists();
						int ret = 0;

						for (int i = 1; i < 5; i++)
						{
							//4 * 8
							int pieceOffset = 32 * (i - 1);
							foreach (var piece in list[i])
							{
								Square s = piece.Square;
								ret += PieceBonusMidGame[pieceOffset + s.Rank * 4 + Min(s.File, 7 - s.File)] / 3;
							}

							foreach (var piece in list[i + 6])
							{
								Square s = piece.Square;
								ret -= PieceBonusMidGame[pieceOffset + (7 - s.Rank) * 4 + Min(s.File, 7 - s.File)] / 3;
							}
						}

						foreach (var pawn in list[0])
							ret += PawnBonusMidGame[pawn.Square.Index];


						foreach (var pawn in list[6])
							ret -= PawnBonusMidGame[new Square(pawn.Square.File, 7 - pawn.Square.Rank).Index];

						return ret;
					}
				}
			}
		}
	}
}