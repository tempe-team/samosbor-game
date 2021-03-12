use serde::{Deserialize, Serialize};
use std::cmp::{max, min};
use std::ops::RangeInclusive;

use crate::location::Position;

#[derive(PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct Renderable {
    pub glyph: char
}

fn make_border_interval (
    axis_size: usize, // vertical|horizontal
    segment_radius: usize,
    coord: usize,
) -> RangeInclusive <usize> {
    let start = if coord + segment_radius > axis_size {
        coord - segment_radius - (coord + segment_radius - axis_size)
    } else {
        max(0, coord as i32 - segment_radius as i32) as usize
    };
    let end = if (coord as i32) - (segment_radius as i32) < 0 {
        coord + segment_radius + (segment_radius - coord)
    } else {
        min(axis_size, coord + segment_radius)
    };
    start..=end
}

/// Segment of map which user see on screen
/// not equal to field of view or map size
pub fn
    get_segment(
        world_width:usize,
        world_height:usize,
        segment_radius: usize,
        view_point: Position,
        world: Vec<char>)
    -> String
{
    let row_range = make_border_interval(
        world_height,
        segment_radius,
        view_point.y,
    );
    let col_range = make_border_interval(
        world_width,
        segment_radius,
        view_point.x,
    );
    let mut result = String::new();
    world
        .chunks(world_width)
        .enumerate()
        .filter(|(i,_)|row_range.contains(i))
        .for_each(|(_, row)|{
            row.iter()
                .enumerate()
                .filter(|(j, _)|col_range.contains(j))
                .for_each(|(_, &c)|{
                    result.push(c)
                });
            result.push('\n')
        });
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_border_interval() {
        assert_eq!(
            make_border_interval(10, 2, 3),
            1..=5
        );
        assert_eq!(
            make_border_interval(10, 2, 0),
            0..=4
        );
        assert_eq!(
            make_border_interval(10, 2, 9),
            6..=10
        );
        assert_eq!(
            make_border_interval(10, 2, 20),
            6..=10
        );
    }

    #[test]
    fn test_get_segment() {
        let vec = vec![
            '#','#','#','#','#','#','#','#','#','#',
            '#','.','.','.','#','#','.','.','.','#',
            '#','.','.','.','#','#','.','.','.','#',
            '#','.','.','.','.','.','.','.','.','#',
            '#','#','#','.','.','.','.','#','#','#',
            '#','#','#','.','.','.','.','#','#','#',
            '#','.','.','.','.','.','.','.','.','#',
            '#','.','.','.','#','#','.','.','.','#',
            '#','.','.','.','#','#','.','.','.','#',
            '#','#','#','#','#','#','#','#','#','#',
        ];
        /*
        ##########
        #...##...#
        #...##...#
        #........#
        ###....###
        ###....###
        #........#
        #...##...#
        #...##...#
        ##########
         */

        assert_eq!(
            get_segment(10,10,2,Position {x:1, y:1}, vec.clone()),
            "#####\n#...#\n#...#\n#....\n###..\n".to_string(),
        );
        assert_eq!(
            get_segment(10,10,2,Position {x:9, y:9}, vec.clone()),
            "...#\n...#\n...#\n####\n".to_string(),
        );
    }
}
