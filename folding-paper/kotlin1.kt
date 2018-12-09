import java.util.*
import java.io.*
import java.math.*

fun main(args : Array<String>) {
    val input = Scanner(System.`in`)
    val order = input.nextLine()
    val side = input.nextLine()
    val paper = Paper()

    for (i in 0..order.length - 1) {
        val c = order[i]
        paper.fold(c)
    }

    println(paper.get(side))
}

class Paper {
    var R: Int = 1
    var L: Int = 1
    var U: Int = 1
    var D: Int = 1
    
    fun get(key: String): Int {
        if (key == "R") return this.R
        if (key == "L") return this.L
        if (key == "U") return this.U
        return this.D
    }

    fun fold(key: Char) {
        if (key == 'R') {
            this.L = this.R + this.L
            this.R = 1
            this.U *= 2
            this.D *= 2
        }
        if (key == 'L') {
            this.R = this.R + this.L
            this.L = 1
            this.U *= 2
            this.D *= 2
        }
        if (key == 'U') {
            this.D = this.U + this.D
            this.U = 1
            this.R *= 2
            this.L *= 2
        }
        if (key == 'D') {
            this.U = this.U + this.D
            this.D = 1
            this.R *= 2
            this.L *= 2
        }
    }
}
