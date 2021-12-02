#![allow(dead_code)]
use array2d::Array2D;

pub fn str_to_image(input: &str, w: usize, h: usize) -> Vec<Array2D<u8>> {
    let base_array: Vec<u8> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let stride = w * h;

    assert_eq!(base_array.len() % stride, 0);

    let mut out: Vec<Array2D<u8>> = Vec::new();
    let num_layers = base_array.len() / stride;
    for i in 0..num_layers {
        out.push(Array2D::from_row_major(
            &base_array[i * stride..(i + 1) * stride],
            h,
            w,
        ));
    }
    out
}
pub fn layer_to_str(layer: &Array2D<u8>) -> String {
    let strings: Vec<String> = layer
        .elements_row_major_iter()
        .map(|i| i.to_string())
        .collect();
    let res = strings.concat();
    let stride = layer.num_columns();
    for i in 0..layer.num_rows() {
        println!("{}", &res[i * stride..(i + 1) * stride].replace('0', " "));
    }
    res
}

fn count_number(layer: &Array2D<u8>, needle: u8) -> usize {
    layer
        .elements_row_major_iter()
        .fold(0, |count, elem| count + (*elem == needle) as usize)
}

fn fewest_zeros(input: &[Array2D<u8>]) -> usize {
    input
        .iter()
        .map(|layer| count_number(layer, 0u8))
        .enumerate()
        .min_by_key(|&(_, item)| item)
        .unwrap()
        .0
}

fn flatten(img: &[Array2D<u8>]) -> Array2D<u8> {
    let w = img.first().unwrap().num_columns();
    let h = img.first().unwrap().num_rows();

    let mut out: Array2D<u8> = Array2D::filled_with(0, h, w);
    for x in 0..w {
        for y in 0..h {
            let mut color = 2; // = transparent
            for layer in img {
                match layer.get(y, x).unwrap() {
                    0 => {
                        color = 0;
                        break;
                    }
                    1 => {
                        color = 1;
                        break;
                    }
                    2 => continue,
                    _ => panic!(),
                }
            }
            out.set(y, x, color).unwrap();
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_eg() {
        let img = str_to_image("123456789012", 3, 2);
        assert_eq!(fewest_zeros(&img), 0);

        let layer = &img[fewest_zeros(&img)];
        assert_eq!(count_number(layer, 1) * count_number(layer, 2), 1)
    }

    #[test]
    fn part1_prob() {
        let in_str = include_str!("test_input.txt");
        let img = str_to_image(in_str, 25, 6);
        let target_idx = fewest_zeros(&img);
        let target_layer = &img[target_idx];
        assert_eq!(
            count_number(target_layer, 1) * count_number(target_layer, 2),
            1548
        )
    }

    #[test]
    fn part2_eg() {
        let img = str_to_image("0222112222120000", 2, 2);
        let out = flatten(&img);
        assert_eq!(layer_to_str(&out), "0110");
    }

    #[test]
    fn part2_prob() {
        let in_str = include_str!("test_input.txt");
        let img = str_to_image(in_str, 25, 6);
        let out = flatten(&img);
        assert_eq!(layer_to_str(&out), "011001111010010100100110010010100001010010010100101000011100110001001010010100001000010100100101111010010100001010010010100100110011110100100110010010");
        // It's 'CEKUA', but I'm not implementing image recognition :/
    }
}
