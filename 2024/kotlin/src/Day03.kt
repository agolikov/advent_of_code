fun main() {
    val day = 3

    data class Operation (
        val type: String, // "do", "dont", or "mul"
        val value: Int? = null, // Value for mul, null for others
        val position: Int
    )

    fun part1(content: String): Int {
        val regex = Regex("mul\\(\\d+,\\d+\\)")

        val sum = regex.findAll(content).sumOf { match ->
            val fullMatch = match.groupValues[0]
            val numberRegex = Regex("\\d+")
            val numbers = numberRegex.findAll(fullMatch).map { it.value.toInt() }.toList()
            numbers[0] * numbers[1]
        }
        return sum
    }

    fun part2(content: String): Int {
        val regex = Regex("do\\(\\)|don't\\(\\)|mul\\((\\d+),(\\d+)\\)")

        val operations = regex.findAll(content).map { match ->
            when {
                match.value == "do()" -> Operation(type = "do",position = match.range.first)
                match.value == "don't()" -> Operation(type = "dont",position = match.range.first)
                match.value.startsWith("mul") -> {
                    // Extract numbers for mul(a,b)
                    val numberRegex = Regex("\\d+")
                    val numbers = numberRegex.findAll(match.value).map { it.value.toInt() }.toList()
                    val product = (numbers[0] * numbers[1]).toInt()
                    Operation(type = "mul", value = product, position = match.range.first)
                }
                else -> throw IllegalArgumentException("Unexpected match: ${match.value}")
            }
        }.toList()

        val sorted = operations //.sortedBy { it.position }

        var doo: Boolean = true
        var sum: Int = 0
        sorted.forEach {
            if (it.type == "do"){
                doo = true
            }
            if (it.type == "dont"){
                doo = false
            }
            if (it.type == "mul" && doo && it.value != null ){
                sum += it.value
            }
        }
        return sum
    }

    val testInput = readTestInput(day)
    check(part1(testInput) == 161)
    check(part2(testInput) == 48)

    val input = readAllInput(day)
    part1(input).println()
    part2(input).println()
}
