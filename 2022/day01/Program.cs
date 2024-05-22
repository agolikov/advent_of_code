var lines = File.ReadAllLines("input.txt");
int c = 0;
List<long> mx = new List<long>();
for (int i = 0; i < lines.Length; ++i)
{
    if (lines[i] == "")
    {
        mx.Add(c);
        c = 0;
    }
    else c += int.Parse(lines[i]);
}
Console.WriteLine(mx.OrderByDescending(x=>x).Take(3).Sum());