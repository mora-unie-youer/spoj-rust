use std::io::BufRead;

/**
 * QCJ1 - Mountain Walking
 *
 * In this problem your task is to program a robot that will output some data about a terrain after traversing it. Input
 * will be in the form a 2D picture containing only 4 types of characters:
 *   '/' : Forward slash, indicating ascent.
 *   '\' : Backward slash, indicating descent.
 *   '_' : Underscore, denoting horizontal plane.
 *
 * Additionally there will be only SPACE (Ascii value = 32) characters. (Refer to the below figure).
 *
 * The robot starts its journey at bottom left corner of the terrain and after traversing stops at the bottom right corner.
 * Also note that the robot will always start and end at the SAME LEVEL.
 *
 * Given the picture as input you will have to output 2 things. The "Total Walk Distance" i.e, the total length of the path
 * and the type of steps taken to complete the Journey. For the sake of simplification we will assume that each character
 * ('/' , '\' and '_') has length = 1.
 *
 * Now Consider the following example:
 *        _
 *     / \/\
 *    /     \
 *   /       \
 *
 * The robot starts at the bottom left corner and takes the following path:
 *   Ascends 3 steps
 *   Moves forward by 1 step
 *   Descends 1 step
 *   Ascends 1 step
 *   Descends 3 steps
 * and robot ends it journey at bottom right corner (At the same level). The Total Walk Distance = 9.
 *
 * Input
 * First line of input will be an integer N (N < 20). The next line will be an empty. Then exactly N lines follow describing
 * the terrain.
 *
 * You can assume the following for the input (terrain).
 *   Input will contain only four types of characters (" ", "_", "/", "\").
 *   The terrain will start and end at the same level.
 *   Each line is guaranteed to have at least one non white space character.
 *   Maximum width of any line won't be larger than 200.
 *   There will be no trailing white spaces.
 *
 * Output
 * First line of output should be the Total Walk Distance followed by the description of the terrain. Each line must be ONE
 * of the following
 *   Up xx steps
 *   Down xx steps
 *   Walk xx steps
 * Where xx is an integer. Refer to the examples for exact specification.
 *
 * Example
 *   Input:
 *   3
 *
 *     /\
 *    /  \
 *   /    \
 *
 *   Output:
 *   Total Walk Distance = 6
 *   Up 3 steps
 *   Down 3 steps
 *
 *   Input:
 *   2
 *    _____  ___
 *   /     \/   \
 *
 *   Output:
 *   Total Walk Distance = 12
 *   Up 1 steps
 *   Walk 5 steps
 *   Down 1 steps
 *   Up 1 steps
 *   Walk 3 steps
 *   Down 1 steps
 *
 *   Input:
 *   5
 *
 *           _
 *      /\__/ \
 *     /       \
 *    /         \/\_
 *   /              \
 *
 *   Output:
 *   Total Walk Distance = 16
 *   Up 4 steps
 *   Down 1 steps
 *   Walk 2 steps
 *   Up 1 steps
 *   Walk 1 steps
 *   Down 3 steps
 *   Up 1 steps
 *   Down 1 steps
 *   Walk 1 steps
 *   Down 1 steps
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let _ = lines.next().unwrap().unwrap();

    let mountains: Vec<String> = std::iter::once(String::new())
        .chain(lines.take(n).map(Result::unwrap))
        .collect();
    let mountains: Vec<&[u8]> = mountains.iter().map(|s| s.as_bytes()).collect();

    let total_distance = mountains.last().unwrap().len();
    println!("Total Walk Distance = {}", total_distance);

    let mut prev_char = mountains.last().unwrap()[0];
    let mut char_count = 0;
    let mut current_height = 0;
    for i in 0..total_distance {
        let mut height_index = mountains.len() - current_height - 1;
        let mut current_char = *mountains[height_index].get(i).unwrap_or(&b' ');

        if current_char != prev_char {
            match prev_char {
                b'/' => println!("Up {} steps", char_count),
                b'_' => println!("Walk {} steps", char_count),
                b'\\' => println!("Down {} steps", char_count),
                _ => unreachable!(),
            }

            char_count = 0;
            match current_char {
                b' ' if prev_char == b'\\' => {
                    current_height += 1;
                    height_index -= 1;
                    current_char = mountains[height_index][i];
                }
                b' ' if prev_char == b'/' || prev_char == b'_' => {
                    current_height -= 1;
                    height_index += 1;
                    current_char = mountains[height_index][i];
                }
                _ => (),
            }

            prev_char = current_char;
        }

        char_count += 1;
        match prev_char {
            b'/' => current_height += 1,
            b'_' if i == total_distance - 1 => println!("Walk {} steps", char_count),
            b'\\' if i == total_distance - 1 => println!("Down {} steps", char_count),
            b'\\' if current_height == 0 => continue,
            b'\\' => current_height -= 1,
            _ => (),
        }
    }
}
