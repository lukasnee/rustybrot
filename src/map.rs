macro_rules! map {
    ($input_value:expr, $input_min:expr, $input_max:expr, $output_min:expr, $output_max:expr) => {
        if $input_max == $input_min {
            $output_min
        } else {
            (($input_value - $input_min) * ($output_max - $output_min)
                / ($input_max - $input_min))
                + $output_min
        }
    };
}
