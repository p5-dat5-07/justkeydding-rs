# justkeydding-rs
A re-implemetation of [justkeydding](https://github.com/napulen/justkeydding) specifcally the [symbolic detecting](https://github.com/napulen/keytracker) optimizing the algorithm. Reducing the time to find the key drastically. This can be seen with the maestro dataset (1282 midi files) which is estimated to take more than 4 hours was reduced to 47 seconds.


```
Usage: justkeydding-rs [OPTIONS] --input-path <INPUT_PATH> --output-file <OUTPUT_FILE>

Options:
  -f, --input-path <INPUT_PATH>
          Input path
  -o, --output-file <OUTPUT_FILE>
          Output file path
  -a, --major-profile <MAJOR_PROFILE>
          Major profile [default: sapp] [possible values: krumhansl-kessler, aarden-essen, sapp, bellman-budge, temperley]
  -i, --minor-profile <MINOR_PROFILE>
          Minor profile [default: sapp] [possible values: krumhansl-kessler, aarden-essen, sapp, bellman-budge, temperley]
  -t, --transition-profile <TRANSITION_PROFILE>
          Transition profile [default: key-transitions-exponential10] [possible values: key-transitions-linear, key-transitions-exponential, key-transitions-exponential10, key-transitions-null, neighbour-level]      
      --major-profile-normalized
          Major profile normalized
      --minor-profile-normalized
          Major profile normalized
  -n, --profile-normalized

  -r, --recursive
          Recursively travels through the input path finding midi files
  -h, --help
          Print help information
  -V, --version
          Print version information
```
