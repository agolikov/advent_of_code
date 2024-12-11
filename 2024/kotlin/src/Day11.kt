import java.util.*

fun main() {
    val day = 11

    var cache : MutableMap<Long, List<Long>> = mutableMapOf()

    fun next(stone: Long): List<Long> {
        var stones : MutableList<Long> = mutableListOf()
        if (!cache.containsKey(stone)) {
            if (stone == 0.toLong()){
                stones.add(1)
            }else{
                var str = stone.toString()
                if (str.length % 2 == 0){
                    var mid = str.length / 2;
                    var left = str.substring(0, mid);
                    var right = str.substring(mid);
                    stones.add(left.toLong());
                    stones.add(right.toLong());
                }else {
                    stones.add(stone*2024)
                }
            }
            cache[stone] = stones.toList()
        }
        return cache[stone]!!
    }

    fun calc(cur: MutableMap<Long, Long>) : MutableMap<Long, Long>{
        var new : MutableMap<Long,Long> = mutableMapOf();
        for (p in cur) {
            val stones = next(p.key)
            for (s in stones){
                if (!new.containsKey(s)) new[s] = p.value else
                    new[s] = new[s]!! + p.value
            }
        }
        return new;
    }

    fun part1(lines: List<String>, leng:Long): Long {
        var nums = lines[0].split(" ").map({ it.toLong()}).toList()
        var set = mutableMapOf<Long, Long>()
        nums.forEach { number -> set[number] = 1 }
        for (i in 0 until leng) {
            set = calc(set)
        }
        return set.values.sum().toLong()
    }

    val testInput = readTestInputLines(day)
    check(part1(testInput,25) == 55312.toLong())

    val input = readAllInputLines(day)
    part1(input,25).println()
    part1(input,75).println()
}