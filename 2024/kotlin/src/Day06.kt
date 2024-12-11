fun main() {
    val day = 6

    val dx = arrayOf(-1, 0, 1, 0)
    val dy = arrayOf(0,  1, 0, -1)

    fun count_steps(map : Array<CharArray>) : Pair<Int,Int> {
        var x = 0
        var y = 0
        for( i in 0 until map.size){
            for (j in 0 until map[i].size)
            {
                if (map[i][j] == '^' || map[i][j] == '>' || map[i][j]=='<'||map[i][j]=='v'){
                    x=i
                    y=j
                    break
                }
            }
        }
        var sh = 0
        var lops = 0
        var rep = 0
        while(true)
        {
            when(map[x][y]){
                '^' -> {sh = 0}
                '>' -> {sh = 1}
                'v' -> {sh = 2}
                '<' -> {sh = 3}
                'O' -> {sh = 3}
            }
            var ex = false
            while(true)
            {
                if (map[x][y]=='X') rep+=1
                map[x][y] = 'X'
                if (rep > 10000000){
                    lops = 10
                    ex = true
                    break
                }
                var nx = x + dx[sh]
                var ny = y + dy[sh]
                if (!(nx>=0 && nx<map[x].size && ny>=0 && ny<map[y].size))
                {
                    ex = true
                    break
                }
                if (map[nx][ny]=='#' || map[nx][ny] == 'O')
                {
                    sh = (sh + 1)%4
                    break
                }
                if (map[nx][ny] == 'O'){
                    lops = lops + 1
                    if (lops > 1)
                    {
                        ex = true
                        break
                    }
                    sh = (sh + 1)%4
                    break
                }
                x = nx
                y = ny
            }
            if(ex){ break }
        }
        var steps = 0
        for( i in 0 until map.size){
            for (j in 0 until map[i].size)
            {
                if (map[i][j] == 'X'){
                    steps+=1
                }
            }
        }
        return Pair(steps, lops)
    }

    fun part1(lines: List<String>): Int {
        val map = lines.map { it.toCharArray() }.toTypedArray()
        var steps = count_steps(map)
        return steps.first
    }

    fun part2(lines: List<String>): Int {
        var total = 0
        for( i in 0 until lines.size){
            for (j in 0 until lines[i].length)
            {
                if (lines[i][j] != '^' && lines[i][j] != '>' && lines[i][j]!='<'&&lines[i][j]!='v'&&lines[i][j]!='#')
                {
                    val map = lines.map { it.toCharArray() }.toTypedArray()
                    map[i][j]='O'
                    var steps =count_steps(map)
                    if (steps.second>1) total+=1
                }
            }
        }
        return total
    }

    val testInput = readTestInputLines(6)
    check(part1(testInput) == 41)
    check(part2(testInput) == 6)

    val input = readAllInputLines(6)
    part1(input).println()
    part2(input).println()
}