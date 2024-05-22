

var lines = File.ReadAllLines("input.txt");
//1
int ans = 0;
for (int i = 0; i < lines.Length; ++i)
{
   ;
    string[] ps = lines[i].Split(',');
    string[] st1 = ps[0].Split('-');
    string[] st2 = ps[1].Split('-');

    int l1 = int.Parse(st1[0]);
    int r1 = int.Parse(st1[1]);
    int l2 = int.Parse(st2[0]);
    int r2 = int.Parse(st2[1]);

    bool[] s = new bool[100];
    for (int j = l1; j <= r1; ++j)
    {
        s[j] = true;
    }

    bool ansB = true;
    for (int j = l2; j <= r2; ++j)
    {
        if (!s[j])
        {
            ansB = false;
            break;
        }
    }

    if (ansB)
    {
        ans++;
        continue;
    }
    s = new bool[100];
    for (int j = l2; j <= r2; ++j)
    {
        s[j] = true;
    }

    ansB = true;
    for (int j = l1; j <= r1; ++j)
    {
        if (!s[j])
        {
            ansB = false;
            break;
        }
    }

    if (ansB)
    {
        ans++;
    }
}
Console.WriteLine(ans);

//2
int ans2 = 0;
for (int i = 0; i < lines.Length; ++i)
{
    ;
    string[] ps = lines[i].Split(',');
    string[] st1 = ps[0].Split('-');
    string[] st2 = ps[1].Split('-');

    int l1 = int.Parse(st1[0]);
    int r1 = int.Parse(st1[1]);
    int l2 = int.Parse(st2[0]);
    int r2 = int.Parse(st2[1]);

    bool[] b = new bool[100];
    for (int j = l1; j <= r1; ++j) b[j] = true;
    for (int j = l2; j <= r2; ++j)
        if (b[j])
        {
            ans2++;
            break;
        }
}
Console.WriteLine(ans2);