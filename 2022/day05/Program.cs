var lines = File.ReadAllLines("input.txt");
Stack<char>[] stacks = new Stack<char>[9];
for (int i = 0; i < 9; ++i)
{
    stacks[i] = new Stack<char>();
}
stacks[0].Push('S');
stacks[0].Push('T');
stacks[0].Push('H');
stacks[0].Push('F');
stacks[0].Push('W');
stacks[0].Push('R');


stacks[1].Push('S');
stacks[1].Push('G');
stacks[1].Push('D');
stacks[1].Push('Q');
stacks[1].Push('W');

stacks[2].Push('B');
stacks[2].Push('T');
stacks[2].Push('W');

stacks[3].Push('D');
stacks[3].Push('R');
stacks[3].Push('W');
stacks[3].Push('T');
stacks[3].Push('N');
stacks[3].Push('Q');
stacks[3].Push('Z');
stacks[3].Push('J');

stacks[4].Push('F');
stacks[4].Push('B');
stacks[4].Push('H');
stacks[4].Push('G');
stacks[4].Push('L');
stacks[4].Push('V');
stacks[4].Push('T');
stacks[4].Push('Z');

stacks[5].Push('L');
stacks[5].Push('P');
stacks[5].Push('T');
stacks[5].Push('C');
stacks[5].Push('V');
stacks[5].Push('B');
stacks[5].Push('S');
stacks[5].Push('G');

stacks[6].Push('Z');
stacks[6].Push('B');
stacks[6].Push('R');
stacks[6].Push('T');
stacks[6].Push('W');
stacks[6].Push('G');
stacks[6].Push('P');

stacks[7].Push('N');
stacks[7].Push('G');
stacks[7].Push('M');
stacks[7].Push('T');
stacks[7].Push('C');
stacks[7].Push('J');
stacks[7].Push('R');

stacks[8].Push('L');
stacks[8].Push('G');
stacks[8].Push('B');
stacks[8].Push('W');
//1
// for (int i = 10; i < lines.Length; ++i)
// {
//     //move 4 from 2 to 1
//     string[] sp = lines[i].Split(' ');
//     int cnt = int.Parse(sp[1]);
//     int fr = int.Parse(sp[3]);
//     int to = int.Parse(sp[5]);
//     
//     for (int j = 0; j < cnt; ++j)
//     {
//         var x = stacks[fr - 1].Pop();
//         stacks[to - 1].Push(x);
//     }
// }
// string ans = "";
// for (int i = 0; i < 9; ++i)
// {
//     ans+=stacks[i].Peek();
//     
// }
// Console.WriteLine(ans);
//2
for (int i = 10; i < lines.Length; ++i)
{
     //move 4 from 2 to 1
     string[] sp = lines[i].Split(' ');
     int cnt = int.Parse(sp[1]);
     int fr = int.Parse(sp[3]);
     int to = int.Parse(sp[5]);
     Stack<char> q = new Stack<char>();
     for (int j = 0; j < cnt; ++j)
     {
         var x = stacks[fr - 1].Pop();
         q.Push(x);
         //stacks[to - 1].Push(x);
     }
     for (int j = 0; j < cnt; ++j)
     {
         stacks[to - 1].Push(q.Pop());
     }
 }
 string ans = "";
 for (int i = 0; i < 9; ++i)
 {
     ans+=stacks[i].Peek();
     
 }
 Console.WriteLine(ans);