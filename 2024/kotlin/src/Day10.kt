import java.util.*

fun main() {
    val day = 10

    val dx = arrayOf(1, 0,-1, 0)
    val dy = arrayOf(0,-1, 0, 1)

    fun search1(p:Pair<Int,Int>, visited : MutableSet<Pair<Int,Int>>, lines : List<String>, d:Int, found : MutableSet<Pair<Int,Int>>)
    {
        if (lines[p.first][p.second]=='9') found.add(Pair(p.first,p.second));
        val rows = lines.size
        val cols = lines[0].length
        visited.add(p)
        for (i in 0 until 4){
            var nx = p.first + dx[i]
            var ny = p.second + dy[i]
            if (nx>=0 && nx < rows && ny>=0 && ny < cols && !visited.contains(Pair(nx,ny))){
                if (lines[nx][ny]==lines[p.first][p.second]+1) {
                    var p = Pair(nx,ny);
                    visited.add(p)
                    search1(p, visited, lines, d+1, found)
                    visited.remove(p)
                }
            }
        }
        visited.remove(p)
    }

    fun search2(p:Pair<Int,Int>, visited : MutableSet<Pair<Int,Int>>, lines : List<String>, d:Int) : Int
    {
        if (lines[p.first][p.second]=='9')
            return 1

        var score = 0
        val rows = lines.size
        val cols = lines[0].length
        visited.add(p)
        for (i in 0 until 4) {
            var nx = p.first + dx[i]
            var ny = p.second + dy[i]
            if (nx>=0 && nx < rows && ny>=0 && ny < cols && !visited.contains(Pair(nx,ny))){
                if (lines[nx][ny]==lines[p.first][p.second]+1) {
                    var p = Pair(nx,ny);
                    visited.add(p)
                    score +=search2(p, visited, lines, d+1);
                    visited.remove(p)
                }
            }
        }
        visited.remove(p)
        return score
    }

    fun part1(lines: List<String>): Int {

        val rows = lines.size
        val cols = lines[0].length
        var starts:MutableList<Pair<Int,Int>> = mutableListOf()
        for (i in 0 until rows){
            for (j in 0 until cols){
                if (lines[i][j]=='0'){
                    starts.add(Pair(i,j))
                }
            }
        }
        var visited : MutableSet<Pair<Int,Int>> = mutableSetOf()
        var result = 0
        for (s in starts) {
            var found : MutableSet<Pair<Int,Int>> = mutableSetOf()
            search1(s, visited, lines,0, found)
            result +=found.count()
        }

        return result
    }

    fun part2(lines: List<String>): Int {
        val rows = lines.size
        val cols = lines[0].length
        var starts:MutableList<Pair<Int,Int>> = mutableListOf()
        for (i in 0 until rows){
            for (j in 0 until cols){
                if (lines[i][j]=='0'){
                    starts.add(Pair(i,j))
                }
            }
        }
        var visited : MutableSet<Pair<Int,Int>> = mutableSetOf()
        var result = 0
        for (s in starts) {
           result += search2(s, visited, lines,0)
        }

        return result
    }

    val testInput = readTestInputLines(day)
    check(part1(testInput) == 36)
    check(part2(testInput) == 81)

    val input = readAllInputLines(day)
    part1(input).println()
    part2(input).println()
}