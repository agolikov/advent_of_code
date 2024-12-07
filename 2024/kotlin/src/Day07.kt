import java.io.File

fun main() {

    fun search(index:Int, nums :List<Long>, cur:Long, target:Long) : Boolean {
        if (index == nums.size-1)
            return cur == target

        var next = nums[index+1]
        return search(index + 1, nums, cur + next, target) ||
                search(index + 1, nums, cur * next, target);
    }

    fun search2(index:Int, nums :List<Long>, cur:Long, target:Long) : Boolean {
        if (index == nums.size - 1)
            return cur == target

        var next = nums[index+1]
        val concat = "$cur$next".toLong()
        return search2(index + 1,     nums, cur + next, target)
                || search2(index + 1, nums, cur * next, target)
                || search2(index + 1, nums, concat, target)
    }

    fun part1(lines: List<String>): Long {
        var vals =  lines.map { it.split(':')[0]} .map { it.toLong() }.toList()
        var operands =  lines.map { it.split(':')[1].trim().split(" ").map { it.toLong() } }.toList()
        var sum : Long = 0
        for (v in vals.indices)
        {
            if (search(0, operands[v], operands[v][0], vals[v])) {
                sum+=vals[v]
            }
        }
        return sum
    }

    fun part2(lines: List<String>): Long {
        var vals =  lines.map { it.split(':')[0]} .map { it.toLong() }.toList()
        var operands =  lines.map { it.split(':')[1].trim().split(" ").map { it.toLong() } }.toList()
        var sum : Long = 0
        for (v in vals.indices)
        {
            if (search2(0, operands[v], operands[v][0], vals[v]))
            {
                sum+=vals[v]
            }
        }
        return sum
    }

    val testInput = readInput("Day07_test")
    check(part1(testInput) == 3749.toLong())
    check(part2(testInput) == 11387.toLong())

    val input = readInput("Day07")
    part1(input).println()
    part2(input).println()
}