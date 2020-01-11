use std::fs;

fn val_at(row_num: usize, col_num: usize, layers: &[Vec<Vec<u32>>]) -> u32 {
    layers.iter().map(|layer| {
        layer.get(row_num).unwrap().get(col_num).unwrap()
    }).fold(2, |acc, &x| {
        match acc {
            2 => x,
            _ => acc
        }
    })
}

fn into_layers(s: &str, width: u8, height: u8) -> Vec<Vec<Vec<u32>>> {
    let mut layers = Vec::new();
    let mut current_layer = Vec::new();
    let mut current_row = Vec::new();
    for c in s.chars() {
        if let Some(digit) = c.to_digit(10) {
            current_row.push(digit);
            if current_row.len() == width as usize {
                current_layer.push(current_row);
                current_row = Vec::new();

                if current_layer.len() == height as usize {
                    layers.push(current_layer);
                    current_layer = Vec::new();
                }
            }
        }
    }
    layers
}

fn decoded_image(layers: &[Vec<Vec<u32>>]) -> Vec<Vec<u32>> {
    let height = layers[0].len();
    let width = layers[0][0].len();
    let mut rows = Vec::new();
    for row_num in 0..height {
        let mut row = Vec::new();
        for col_num in 0..width {
            let val = val_at(row_num, col_num, layers);
            row.push(val);
        }
        rows.push(row);
    }

    rows
}

fn render_layer(layer: &[Vec<u32>]) -> String {
    let mut s = String::new();
    for row in layer {
        for x in row {
            let pixel = match x {
                0 => ' ',
                1 => 'O',
                _ => panic!("unknown pixel value {}", x)
            };
            s.push(pixel);
        }
        s.push('\n');
    }
    s
}

fn count_digits_eq(layer: &[Vec<u32>], digit: u32) -> u32 {
    layer.iter().map(|row| {
        row.iter().filter(|&&d| d == digit).count() as u32
    }).sum()
}

pub fn pt1(path_to_input: &str) -> u32 {
    let s = fs::read_to_string(path_to_input).unwrap();
    let layers = into_layers(&s, 25, 6);
    let layer_with_fewest_zeros = layers.iter().min_by_key(|layer| {
        count_digits_eq(layer, 0)
    }).unwrap();
    let num_1s = count_digits_eq(layer_with_fewest_zeros, 1);
    let num_2s = count_digits_eq(layer_with_fewest_zeros, 2);
    
    num_1s * num_2s
}

pub fn pt2(path_to_input: &str) -> String {
    let s = fs::read_to_string(path_to_input).unwrap();
    let layers = into_layers(&s, 25, 6);
    let master_layer = decoded_image(&layers);
    render_layer(&master_layer)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn into_layers_example1() {
        assert_eq!(
            into_layers("123456789012", 3, 2),
            vec![
                vec![vec![1, 2, 3], vec![4, 5, 6]],
                vec![vec![7, 8, 9], vec![0, 1, 2]]
            ]
        );
    }

    #[test]
    fn pt1_test() {
        assert_eq!(
            pt1("input"),
            2480
        )
    }

    #[test]
    fn pt2_test() {
        assert_eq!(
            pt2("input"),
            "\
OOOO O   OOOO  O    O  O 
   O O   OO  O O    O  O 
  O   O O OOO  O    OOOO 
 O     O  O  O O    O  O 
O      O  O  O O    O  O 
OOOO   O  OOO  OOOO O  O 
"
        );
    }
}
