<?php
    function find_matches($line) {
        $num_array = [
        "1" => "1",
        "2"=> "2",
        "3"=> "3",
        "4"=> "4",
        "5"=> "5",
        "6"=> "6",
        "7"=> "7",
        "8"=> "8",
        "9"=> "9",
        "one" => "1",
        "two"=> "2",
        "three"=> "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9"
    ];
        $first_match = "";
        $last_match = "";

        for ($i = 0; $i < strlen($line); $i++) {
            for ($j = $i; $j < $i + 6; $j++) {
                $cur = substr($line, $i, $j - $i);
                if (array_key_exists($cur,$num_array)) {
                    if ($first_match == "") {
                        $first_match = $num_array[$cur];
                    };
                    $last_match = $num_array[$cur];
                };
            };
        };
        
        return array($first_match, $last_match);
    };


    $myfile = fopen("../question", "r") or die("Unable to open file!");

    $file = fread($myfile,filesize("../question"));
    fclose($myfile);
    
    $file = explode("\n", $file);
    $sum = 0;
    foreach ($file as $line) {
        [$first_match, $last_match] = find_matches($line);
        $match = $first_match . $last_match;
        $sum += intval($match);
    }
    echo $sum . "\n";
?>