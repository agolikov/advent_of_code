var lines = File.ReadAllLines("input.txt");
int r = 0;
for (int i = 0; i < lines.Length; ++i)
{
    //A -rock B - paper C -scissorts
    //X - lose
    //Y - draw
    //Z - win
    var ab = lines[i].Replace(" ","");
    char a = ab[0];
    char b = ab[1];
    if (b == 'X') { r += 0;
    }
    if (b == 'Y') { r += 3;
    }
    if (b == 'Z') { r += 6;}

    if (a == 'A' && b == 'X') r += 3;
    if (a == 'A' && b == 'Y') r += 1;
    if (a == 'A' && b == 'Z') r += 2;
    
    if (a == 'B' && b == 'X') r += 1;
    if (a == 'B' && b == 'Y') r += 2;
    if (a == 'B' && b == 'Z') r += 3;

    if (a == 'C' && b == 'X') r += 2;
    if (a == 'C' && b == 'Y') r += 3;
    if (a == 'C' && b == 'Z') r += 1;

}
Console.WriteLine(r);