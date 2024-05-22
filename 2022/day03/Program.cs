var lines = File.ReadAllLines("input.txt");

int r = 0;
List<char> chars = new List<char>();
for (int i = 0; i < lines.Length; ++i)
{
     string a = lines[i].Substring(0, lines[i].Length / 2);
     string b = lines[i].Substring(lines[i].Length / 2, lines[i].Length / 2);
     List<char> ddd = new List<char>();
     for (int j = 0; j < a.Length; ++j)
     {
          for (int k = 0; k < b.Length; ++k)
          {
               if (a[j] == b[k])
               {
                    ddd.Add(a[j]);
               }
          }
     }
     chars.AddRange(ddd.Distinct());
}

for (int i = 0; i < chars.Count; ++i)
{
     if ('a' <= chars[i] && chars[i] <= 'z') r += chars[i] - 'a' + 1;
     if ('A' <= chars[i] && chars[i] <= 'Z') r += chars[i] - 'A' + 27;
}

Console.WriteLine(r);

r = 0;
chars = new List<char>();
for (int i = 0; i < lines.Length; i+=3)
{
     bool added = false;
     List<char> ccc = new List<char>();
     for (int i1 = 0; i1 < lines[i].Length; ++i1)
     {
          for (int i2 = 0; i2 < lines[i + 1].Length; ++i2)
          {
               for (int i3 =0; i3 < lines[i + 2].Length; ++i3)
               {
                    if (lines[i][i1] == lines[i + 1][i2] && lines[i + 1][i2] == lines[i + 2][i3])
                    {
                         ccc.Add(lines[i][i1]);
                    }
               }
          }
     }
     chars.AddRange(ccc.Distinct());
}

for (int i = 0; i < chars.Count; ++i)
{
     if ('a' <= chars[i] && chars[i] <= 'z') r += chars[i] - 'a' + 1;
     if ('A' <= chars[i] && chars[i] <= 'Z') r += chars[i] - 'A' + 27;
}

Console.WriteLine(r);
