import java.math.BigInteger
import java.security.MessageDigest
import kotlin.io.path.Path
import kotlin.io.path.readText

/**
 * Reads lines from the given input txt file.
 */
fun readTestInputLines(day: Int) = Path(getInputTestFilePath(day)).readText().trim().lines()

fun readTestInput(day: Int) = Path(getInputTestFilePath(day)).readText().trim()

fun readAllInputLines(day: Int) = Path(getInputFilePath(day)).readText().trim().lines()

fun readAllInput(day: Int) = Path(getInputFilePath(day)).readText().trim()

fun getInputTestFilePath(day: Int) : String {
    if (day < 10) {
        return "inputs/Day0" + "$day" + "_test.txt";
    }
    return "inputs/Day" + "$day" + "_test.txt";
}

fun getInputFilePath(day: Int) : String {
    if (day < 10) {
        return "inputs/Day0$day.txt";
    }
    return "inputs/Day$day.txt";
}

/**
 * Converts string to md5 hash.
 */
fun String.md5() = BigInteger(1, MessageDigest.getInstance("MD5").digest(toByteArray()))
    .toString(16)
    .padStart(32, '0')

/**
 * The cleaner shorthand for printing output.
 */
fun Any?.println() = println(this)
