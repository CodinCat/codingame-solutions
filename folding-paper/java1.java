import java.util.*;
import java.io.*;
import java.math.*;

class Paper {
    private int R = 1;
    private int L = 1;
    private int U = 1;
    private int D = 1;

    public int get(String key) {
        if (key.equals("R")) return this.R;
        if (key.equals("L")) return this.L;
        if (key.equals("U")) return this.U;
        return this.D;
    }
    public void fold(char key) {
        if (key == 'R') {
            this.L = this.R + this.L;
            this.R = 1;
            this.U *= 2;
            this.D *= 2;
        }
        if (key == 'L') {
            this.R = this.R + this.L;
            this.L = 1;
            this.U *= 2;
            this.D *= 2;
        }
        if (key == 'U') {
            this.D = this.U + this.D;
            this.U = 1;
            this.R *= 2;
            this.L *= 2;
        }
        if (key == 'D') {
            this.U = this.U + this.D;
            this.D = 1;
            this.R *= 2;
            this.L *= 2;
        }
    }
}

class Solution {
    public static void main(String args[]) {
        Scanner in = new Scanner(System.in);
        String order = in.nextLine();
        String side = in.nextLine();
        Paper paper = new Paper();
        
        for (int i = 0, n = order.length(); i < n; i++) {
            char c = order.charAt(i);
            paper.fold(c);
        }

        System.out.println(paper.get(side));
    }
}