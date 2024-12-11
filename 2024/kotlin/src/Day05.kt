fun main() {
    val day = 5

    fun check_order( pairs: MutableList<Pair<Int,Int>>, nums : List<Int>) : Int
    {
        var errors : Int = 0
        val mutableSet : MutableSet<Int> = mutableSetOf()
        for (j in nums.indices) {
            for (p in pairs.indices){
                if (pairs[p].first == nums[j]){
                    if (mutableSet.contains(pairs[p].second)) {
                        errors += 1
                    }
                }
            }
            mutableSet.add(nums[j]);
        }
        return errors
    }

    fun part1(lines: List<String>): Int {
        var ind = 0
        var pairs : MutableList<Pair<Int,Int>> = mutableListOf()
        for (i in 0 until lines.size)
        {
            if (lines[i]==""){
                ind = i+1
                break
            }
            val nums = lines[i].split('|')
            pairs.add(Pair(nums[0].toInt(), nums[1].toInt()))
        }
        var sumFirst = 0
        for(i in ind until lines.size) {
            var nums = lines[i].split(',').map { it.toInt() }.toList().toMutableList()
            if (check_order(pairs, nums) == 0) {
                sumFirst += nums[nums.size / 2]
            }
        }

        return sumFirst
    }

    fun part2(lines: List<String>): Int {
        var ind = 0
        var pairs : MutableList<Pair<Int,Int>> = mutableListOf()
        for (i in 0 until lines.size)
        {
            if (lines[i]==""){
                ind = i+1
                break
            }
            val nums = lines[i].split('|')
            pairs.add(Pair(nums[0].toInt(), nums[1].toInt()))
        }
        var sumSecond = 0
        for(i in ind until lines.size) {
            var nums = lines[i].split(',').map { it.toInt() }.toList().toMutableList()
            if (check_order(pairs, nums) != 0) {
                var errors = 1000
                while (true)
                {
                    val x = (0..nums.size-1).random()
                    val y = (0..nums.size-1).random()

                    var t :Int = nums[x]
                    nums[x] = nums[y]
                    nums[y] = t
                    val current = check_order(pairs, nums)
                    if (current == 0)
                    {
                        sumSecond += nums[nums.size / 2]
                        break
                    }
                    else
                    {
                        if (errors < current){
                            var t :Int = nums[x]
                            nums[x] = nums[y]
                            nums[y] = t
                        }else{
                            errors = current
                        }
                    }
                }
            }
        }
        return sumSecond
    }


    val testInput = readTestInputLines(day)
    check(part1(testInput) == 143)

    val input = readAllInputLines(day)
    part1(input).println()
    part2(input).println()
}