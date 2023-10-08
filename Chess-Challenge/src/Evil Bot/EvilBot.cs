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
			//return Move.NullMove;
			Move m = SF(board, timer);
			//Move m = BarschOld(board, timer);

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
	}
}