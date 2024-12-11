
fun main() {
    val day = 8

    fun part1(lines: List<String>): Int {
        val rows = lines.size
        var cols = lines[0].length
        var antennas : MutableList<Pair<Int,Int>> = mutableListOf()
        for (x in 0 until rows){
            for (y in 0 until cols)
            {
                if (lines[x][y].isLetter() || lines[x][y].isDigit()){
                    antennas.add(Pair(x,y))
                }
            }
        }
        var antidotes: MutableSet<Pair<Int,Int>> = mutableSetOf()
        for (a in antennas){
            for (b in antennas) {
                if (a==b)
                    continue;
                if (lines[a.first][a.second]!=lines[b.first][b.second])
                    continue;

                val dx = a.first - b.first
                val dy = a.second - b.second
                val nx = a.first + dx
                val ny = a.second + dy
                if (nx >= 0 && nx < rows && ny >= 0 && ny < cols)
                    antidotes.add(Pair(nx,ny))
            }
        }
        return antidotes.count()
    }

    fun part2(lines: List<String>): Int {
        val rows = lines.size
        var cols = lines[0].length
        var antennas : MutableList<Pair<Int,Int>> = mutableListOf()
        for (x in 0 until rows){
            for (y in 0 until cols)
            {
                if (lines[x][y].isLetter() || lines[x][y].isDigit()){
                    antennas.add(Pair(x,y))
                }
            }
        }
        var antidotes: MutableSet<Pair<Int,Int>> = mutableSetOf()
        for (a in antennas){
            for (b in antennas) {
                if (a==b)
                    continue;
                if (lines[a.first][a.second]!=lines[b.first][b.second])
                    continue;
                antidotes.add(Pair(a.first, a.second))
                val dx = a.first - b.first
                val dy = a.second - b.second
                var nx = a.first + dx
                var ny = a.second + dy
                while (nx >= 0 && nx < rows && ny >= 0 && ny < cols) {
                    antidotes.add(Pair(nx, ny))
                    nx+=dx
                    ny+=dy
                }
            }
        }
        return antidotes.count()
    }

    val testInput = readTestInputLines(day)
    check(part1(testInput) == 14)
    check(part2(testInput) == 34)

    val input = readAllInputLines(day)
    part1(input).println()
    part2(input).println()
}