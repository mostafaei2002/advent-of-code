<?php
    function find_matches($line) {
        $num_array = array("1","2","3","4","5","6","7","8","9");
        $first_match = "";
        $last_match = "";

        for ($i = 0; $i < strlen($line); $i++) {
            $cur = $line[$i];
            if (in_array($cur, $num_array)) {
                if ($first_match == "") {
                    $first_match = $cur;
                };
                $last_match = $cur;
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