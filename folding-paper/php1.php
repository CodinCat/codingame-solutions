<?php
$order = stream_get_line(STDIN, 8 + 1, "\n");
$side = stream_get_line(STDIN, 5 + 1, "\n");

class Paper {
    public $R = 1;
    public $L = 1;
    public $U = 1;
    public $D = 1;
    public function fold($c) {
        switch ($c) {
        case 'R':
            $this->L += $this->R;
            $this->R = 1;
            $this->U *= 2;
            $this->D *= 2;
            break;
        case 'L':
            $this->R += $this->L;
            $this->L = 1;
            $this->U *= 2;
            $this->D *= 2;
            break;
        case 'U':
            $this->D += $this->U;
            $this->U = 1;
            $this->L *= 2;
            $this->R *= 2;
            break;
        case 'D':
            $this->U += $this->D;
            $this->D = 1;
            $this->L *= 2;
            $this->R *= 2;
            break;
        }
    }
}

$paper = new Paper();
$order_arr = str_split($order);
foreach ($order_arr as $c) {
    $paper->fold($c);
}

$ans = $paper->$side;
echo("$ans\n");
