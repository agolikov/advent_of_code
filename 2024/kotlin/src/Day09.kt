import org.jetbrains.annotations.Nullable
import java.util.*

fun main() {
    val day = 9

    data class Item(var id: Int, var isFile: Boolean)

    fun part1(lines: List<String>): Long {
        val input = lines[0]
        var id = 0
        var isFile = true
        var items :MutableList<Item> = mutableListOf()
        for (c in 0 until input.length) {
            for (x in 0 until input[c].toString().toInt()) {
                items.add(Item(id = id, isFile = isFile))
            }
            if (isFile) id++
            isFile = !isFile
        }
        var read = items.size - 1
        var write = 0
        while (true)
        {
            while(!items[read].isFile) read--
            while(items[write].isFile) write++
            if (read <= write) break;

            items[write].id = items[read].id
            items[write].isFile = true
            items[read].isFile = false
        }
        var result: Long = 0
        for (i in 0 until items.size) {
            result += i.toLong() * (if (items[i].isFile) items[i].id else 0);
        }
        return result
    }

    fun part2(lines: List<String>): Long {
        val input = lines[0]
        var id = 0
        var isFile = true
        var items: MutableList<Item> = mutableListOf()
        for (c in 0 until input.length) {
            for (x in 0 until input[c].toString().toInt()) {
                items.add(Item(id = id, isFile = isFile))
            }
            if (isFile) id++
            isFile = !isFile
        }
        var from = items.size - 1
        var lastStart = 0
        while (true) {
            var end = from
            while (end >= 0 && !items[end].isFile) end--
            if (end <= 1) break;
            var charToMove = items[end]
            from = end
            while (from >= 0 && items[from].id == items[end].id && items[from].isFile) from--
            var needed = end - from
            var start =lastStart
            var first:Boolean = true;
            while (start < from) {

                if (items[start].isFile) {
                    start++; continue
                }
                while (start < items.size && items[start].isFile) start++
                var to = start
                while (to < items.size && !items[to].isFile) to++
                var available = to - start

                if (available >= needed) {
                    for (i in 0 until needed) {
                        items[i + start].isFile = charToMove.isFile
                        items[i + start].id = charToMove.id
                        items[i + from + 1].isFile = false
                    }
                    break
                } else {
                    start += 1
                    if (first) {
                        lastStart = start-1
                        first = false
                    }
                }
            }
        }
        var result: Long = 0
        for (i in 0 until items.size) {
            result += i.toLong() * (if (items[i].isFile) items[i].id else 0);
        }
        return result
    }

    val testInput = readTestInputLines(day)
    check(part1(testInput) == 1928.toLong())
    check(part2(testInput) == 2858.toLong())

    val input = readAllInputLines(day)
    part1(input).println()
    part2(input).println()
}