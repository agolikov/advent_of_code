import java.io.File

val word : String = "XMAS"
var count : Int = 0

val dx = arrayOf(1, 0,-1, 0, 1, -1, 1, -1)
val dy = arrayOf(0,-1, 0, 1, 1, -1, -1, 1)

fun searchWord(type: Int, ind: Int, posX : Int, posY : Int, v : Array<BooleanArray>, chars : List<String>, visited : Array<BooleanArray>)
{
    if (chars[posX][posY] == word[ind])
    {
        if (ind+1 == word.length) {
            count += 1
            for (i in v.indices){
                for (j in v[0].indices)
                {
                    visited[i][j] = visited[i][j] or v[i][j]
                }
            }
        }
        else {
            val nx = dx[type] + posX
            val ny = dy[type] + posY
            if (nx>=0&&nx < chars.count() && ny >=0 && ny < chars[0].length && !v[nx][ny])
            {
                v[nx][ny] = true
                searchWord(type,ind+1, nx, ny, v, chars, visited)
                v[nx][ny] = false
            }

        }
    }
}

fun main() {
    fun part1(lines: List<String>): Int {
        count = 0
        val rows = lines.count()
        val cols = lines[0].count()
        val v = Array(rows) { BooleanArray(cols) { false } }
        val visited = Array(rows) { BooleanArray(cols) { false } }
        for (i in lines.indices) {
            for (j in lines[i].indices) {
                if (lines[i][j] == 'X') {
                    for (t in 0 until 8) {
                        v[i][j] = true
                        searchWord(t, 0, i, j, v, lines, visited)
                        v[i][j] = false
                    }
                }
            }
        }
        return count
    }

    fun xmas_match(x: Int, y: Int, lines : List<String>) : Boolean
    {
        if (lines[x][y]=='A')
        {
            val a = lines[x-1][y-1]
            val b = lines[x-1][y+1]
            val c = lines[x+1][y-1]
            val d = lines[x+1][y+1]
            val vals = arrayOf(lines[x-1][y-1],lines[x-1][y+1], lines[x+1][y-1], lines[x+1][y+1])
            if (vals.count { it == 'M'} == 2 && vals.count {it == 'S'} == 2)
            {
                if (a==b || a == c)
                    return true
            }
        }
        return false
    }

    fun part2(lines: List<String>): Int {
        val rows = lines.count()
        val cols = lines[0].count()
        var count :Int = 0
        for (i in 1 until rows - 1) {
            for (j in 1 until cols - 1) {
                if (xmas_match(i,j, lines)){
                    count+=1
                }
            }
        }
        return count
    }

    val testInput = readInput("Day04_test")
    check(part1(testInput) == 18)

    val input = readInput("Day04")
    part1(input).println()
    part2(input).println()
}