use bls12_381::{
    multi_miller_loop, pairing, G1Affine, G1Projective, G2Affine, G2Prepared, G2Projective, Gt,
    MillerLoopResult, Scalar,
};

pub fn bytes_g1_mul(g1_bytes: [u8; 96], x: u64) -> [u8; 96] {
    let g1 = bytes_to_g1(g1_bytes);
    let x = Scalar::from(x);
    let g1_result = G1Affine::from(g1 * x);
    g1_to_bytes(g1_result)
}

pub fn bytes_g2_mul(g2_bytes: [u8; 192], x: u64) -> [u8; 192] {
    let g2 = bytes_to_g2(g2_bytes);
    let x = Scalar::from(x);
    let g2_result = G2Affine::from(g2 * x);
    g2_to_bytes(g2_result)
}

pub fn bytes_g1_add(g1_1_bytes: [u8; 96], g1_2_bytes: [u8; 96]) -> [u8; 96] {
    let g1_1 = bytes_to_g1(g1_1_bytes);
    let g1_2 = bytes_to_g1(g1_2_bytes);
    let g1_1 = G1Projective::from(&g1_1);
    let g1_2 = G1Projective::from(&g1_2);
    let g1_result = G1Affine::from(g1_1 + g1_2);
    g1_to_bytes(g1_result)
}

pub fn bytes_g2_add(g2_1_bytes: [u8; 192], g2_2_bytes: [u8; 192]) -> [u8; 192] {
    let g2_1 = bytes_to_g2(g2_1_bytes);
    let g2_2 = bytes_to_g2(g2_2_bytes);
    let g2_1 = G2Projective::from(&g2_1);
    let g2_2 = G2Projective::from(&g2_2);
    let g2_result = G2Affine::from(g2_1 + g2_2);
    g2_to_bytes(g2_result)
}

use std::mem::*;
pub fn g1_to_bytes(g1: G1Affine) -> [u8; 96] {
    g1.to_uncompressed()
}

pub fn bytes_to_g1(bytes: [u8; 96]) -> G1Affine {
    G1Affine::from_uncompressed(&bytes).unwrap()
}

pub fn g1_bytes_neg(bytes: [u8; 96]) -> G1Affine {
    let g1 = G1Affine::from_uncompressed(&bytes).unwrap();
    -g1
}

pub fn g2_to_bytes(g2: G2Affine) -> [u8; 192] {
    g2.to_uncompressed()
}

pub fn bytes_to_g2(bytes: [u8; 192]) -> G2Affine {
    G2Affine::from_uncompressed(&bytes).unwrap()
}

pub fn g2_bytes_neg(bytes: [u8; 192]) -> G2Affine {
    let g2 = G2Affine::from_uncompressed(&bytes).unwrap();
    -g2
}

pub fn bytes_to_gt(bytes: [u64; 72]) -> Gt {
    let gt = unsafe { transmute::<[u64; 72], Gt>(bytes) };
    gt
}

pub fn gt_to_bytes(gt: Gt) -> [u64; 72] {
    let bytes = unsafe { transmute::<Gt, [u64; 72]>(gt) };
    bytes
}

pub fn gt_bytes_neg(bytes: [u64; 72]) -> Gt {
    let gt = unsafe { transmute::<[u64; 72], Gt>(bytes) };
    -gt
}

pub fn bytes_multi_miller_loop(g1_bytes: [u8; 96], g2_bytes: [u8; 192]) -> [u64; 72] {
    let g1 = G1Affine::from_uncompressed(&g1_bytes).unwrap();
    let g2 = G2Affine::from_uncompressed(&g2_bytes).unwrap();
    let g2_pre = G2Prepared::from(g2);
    let result = multi_miller_loop(&[(&g1, &g2_pre)]).final_exponentiation();
    gt_to_bytes(result)
}

pub fn bytes_pairing(g1_bytes: [u8; 96], g2_bytes: [u8; 192]) -> [u64; 72] {
    let g1affine = G1Affine::from_uncompressed(&g1_bytes).unwrap();
    let g2affine = G2Affine::from_uncompressed(&g2_bytes).unwrap();
    let result = pairing(&g1affine, &g2affine);
    gt_to_bytes(result)
}

pub fn bytes_gt_add(a: [u64; 72], b: [u64; 72]) -> [u64; 72] {
    let a = bytes_to_gt(a);
    let b = bytes_to_gt(b);
    gt_to_bytes(a + b)
}

#[cfg(test)]
mod test {
    #[test]
    pub fn verify() {
        use crate::bls_extern::*;
        let a: [u8; 96] = [
            2, 88, 157, 112, 250, 72, 149, 167, 140, 203, 82, 224, 80, 178, 25, 167, 81, 78, 135,
            22, 195, 186, 98, 114, 127, 47, 79, 56, 198, 115, 4, 214, 73, 171, 97, 187, 13, 111, 1,
            183, 215, 199, 12, 128, 78, 187, 113, 209, 7, 154, 178, 12, 2, 36, 66, 234, 250, 154,
            213, 149, 74, 111, 30, 120, 147, 66, 206, 26, 60, 119, 29, 67, 220, 25, 7, 110, 89, 40,
            255, 240, 101, 53, 9, 141, 104, 45, 185, 227, 204, 187, 207, 203, 158, 164, 230, 127,
        ];
        let a_point = bytes_to_g1(a);
        let b: [u8; 192] = [
            7, 246, 242, 75, 174, 218, 111, 64, 63, 27, 36, 177, 240, 72, 121, 94, 39, 63, 55, 176,
            199, 165, 152, 27, 179, 54, 3, 3, 9, 133, 97, 181, 16, 235, 109, 82, 127, 169, 66, 174,
            106, 16, 71, 106, 131, 175, 30, 175, 13, 144, 171, 46, 215, 21, 184, 238, 78, 159, 15,
            185, 177, 13, 48, 231, 51, 143, 203, 173, 213, 154, 160, 1, 242, 227, 2, 164, 124, 144,
            199, 98, 167, 175, 122, 114, 102, 137, 12, 48, 84, 131, 0, 169, 158, 55, 128, 230, 17,
            3, 24, 185, 3, 217, 103, 160, 126, 115, 201, 178, 174, 146, 199, 136, 221, 31, 54, 157,
            209, 3, 43, 21, 101, 177, 244, 200, 133, 156, 139, 62, 80, 69, 175, 8, 121, 46, 19, 50,
            51, 48, 214, 203, 1, 98, 226, 31, 17, 69, 107, 192, 117, 64, 235, 73, 178, 107, 68,
            164, 196, 70, 238, 142, 112, 231, 36, 20, 62, 45, 78, 22, 83, 127, 103, 135, 222, 247,
            183, 83, 185, 13, 59, 149, 36, 145, 90, 254, 91, 179, 87, 174, 108, 151, 101, 183,
        ];
        let b_point = bytes_to_g2(b);
        let c: [u8; 96] = [
            0, 213, 12, 150, 48, 255, 49, 226, 134, 136, 162, 136, 138, 49, 33, 226, 130, 57, 233,
            218, 189, 99, 250, 116, 33, 92, 163, 118, 177, 216, 143, 174, 221, 225, 73, 100, 94,
            84, 243, 228, 142, 40, 246, 162, 241, 48, 175, 173, 11, 10, 155, 75, 167, 34, 195, 30,
            35, 142, 99, 224, 167, 63, 180, 59, 50, 185, 183, 162, 21, 138, 43, 59, 100, 33, 36,
            58, 131, 187, 14, 220, 224, 150, 219, 231, 107, 231, 46, 219, 112, 93, 200, 175, 82,
            230, 200, 246,
        ];
        let c_point = bytes_to_g1(c);
        let alpha_g1: [u8; 96] = [
            0, 106, 167, 155, 164, 170, 67, 158, 237, 78, 91, 7, 243, 191, 186, 221, 27, 97, 6,
            190, 193, 204, 85, 206, 83, 56, 3, 209, 132, 249, 221, 94, 124, 20, 245, 113, 143, 70,
            245, 159, 104, 213, 37, 151, 209, 125, 160, 143, 22, 78, 252, 97, 225, 215, 133, 240,
            84, 149, 231, 142, 83, 116, 156, 136, 120, 45, 242, 155, 199, 169, 244, 228, 221, 245,
            160, 28, 247, 39, 83, 6, 172, 25, 126, 223, 231, 40, 94, 154, 69, 103, 123, 105, 242,
            163, 156, 220,
        ];
        let alpha_g1_point = bytes_to_g1(alpha_g1);
        let beta_g2: [u8; 192] = [
            3, 91, 30, 20, 61, 202, 142, 33, 164, 33, 215, 106, 219, 39, 136, 96, 112, 254, 117,
            55, 156, 44, 55, 125, 240, 63, 166, 206, 157, 17, 201, 11, 33, 172, 226, 58, 254, 202,
            46, 128, 2, 179, 227, 37, 230, 127, 121, 118, 6, 59, 84, 145, 104, 196, 68, 37, 209,
            54, 86, 148, 155, 251, 36, 110, 127, 190, 205, 52, 100, 136, 226, 196, 249, 172, 122,
            215, 230, 42, 92, 175, 190, 120, 19, 80, 56, 148, 236, 157, 108, 74, 45, 29, 157, 243,
            96, 94, 17, 16, 242, 165, 56, 132, 223, 128, 0, 35, 238, 8, 138, 176, 102, 236, 242,
            177, 252, 151, 152, 94, 230, 130, 111, 185, 250, 195, 15, 125, 31, 128, 3, 102, 181,
            56, 19, 195, 121, 224, 82, 228, 3, 41, 31, 122, 220, 133, 18, 212, 95, 194, 201, 185,
            75, 111, 233, 98, 80, 47, 9, 191, 178, 119, 49, 238, 30, 235, 217, 40, 8, 199, 253,
            123, 8, 85, 78, 100, 32, 111, 185, 57, 197, 240, 76, 6, 252, 16, 114, 82, 90, 163, 240,
            146, 4, 2,
        ];
        let beta_g2_point = bytes_to_g2(beta_g2);
        let gamma_g2: [u8; 192] = [
            23, 69, 47, 108, 115, 173, 254, 203, 89, 67, 183, 224, 176, 26, 127, 132, 89, 162, 99,
            241, 66, 228, 177, 17, 57, 85, 3, 13, 148, 88, 162, 54, 220, 189, 33, 172, 38, 192,
            116, 236, 13, 115, 219, 201, 51, 166, 253, 240, 12, 32, 77, 82, 161, 189, 240, 198,
            148, 184, 17, 92, 162, 145, 166, 55, 252, 245, 194, 95, 71, 208, 215, 23, 19, 95, 138,
            147, 149, 26, 35, 108, 141, 25, 139, 103, 59, 48, 189, 88, 204, 100, 255, 116, 194,
            229, 157, 5, 19, 16, 31, 158, 222, 45, 151, 86, 218, 157, 17, 252, 29, 131, 121, 107,
            168, 46, 145, 176, 122, 146, 93, 180, 181, 98, 229, 4, 29, 34, 137, 222, 93, 124, 90,
            211, 99, 51, 90, 96, 191, 203, 14, 34, 14, 5, 77, 209, 0, 62, 138, 150, 103, 94, 252,
            200, 186, 64, 197, 27, 173, 229, 189, 193, 196, 75, 40, 95, 107, 36, 50, 90, 146, 59,
            215, 202, 184, 77, 87, 20, 53, 241, 208, 72, 158, 45, 22, 81, 53, 220, 40, 222, 26, 69,
            230, 253,
        ];
        let gamma_g2_point = bytes_to_g2(gamma_g2);
        let delta_g2: [u8; 192] = [
            1, 78, 83, 175, 159, 103, 127, 217, 80, 213, 0, 194, 108, 30, 210, 241, 138, 209, 0,
            164, 117, 32, 68, 102, 121, 36, 40, 65, 89, 205, 198, 1, 14, 144, 196, 236, 176, 214,
            119, 139, 225, 118, 215, 185, 36, 216, 183, 27, 22, 126, 193, 21, 173, 212, 250, 104,
            25, 69, 107, 40, 199, 160, 228, 239, 112, 102, 144, 85, 58, 109, 122, 73, 221, 170,
            145, 188, 60, 9, 228, 178, 36, 227, 175, 140, 40, 181, 158, 175, 91, 189, 92, 169, 90,
            90, 30, 153, 4, 225, 187, 53, 206, 114, 60, 109, 51, 184, 2, 100, 39, 95, 43, 33, 82,
            141, 161, 200, 136, 146, 33, 18, 202, 141, 43, 222, 64, 81, 151, 58, 141, 146, 8, 214,
            159, 110, 167, 173, 253, 57, 190, 62, 94, 88, 245, 59, 20, 121, 233, 209, 122, 42, 13,
            184, 114, 0, 19, 32, 120, 143, 108, 118, 107, 241, 218, 182, 69, 135, 117, 42, 231,
            191, 199, 88, 88, 145, 134, 24, 133, 211, 53, 72, 23, 214, 105, 97, 134, 254, 116, 89,
            166, 119, 221, 223,
        ];
        let delta_g2_point = bytes_to_g2(delta_g2);
        let ic0: [u8; 96] = [
            14, 152, 253, 159, 101, 142, 227, 5, 166, 71, 152, 207, 32, 152, 56, 172, 191, 43, 184,
            28, 148, 40, 224, 42, 135, 137, 181, 215, 96, 34, 200, 127, 77, 151, 165, 11, 130, 57,
            91, 83, 71, 38, 253, 159, 103, 191, 139, 120, 20, 9, 91, 120, 106, 16, 209, 88, 87,
            206, 209, 233, 129, 87, 15, 139, 92, 164, 84, 150, 51, 92, 220, 188, 115, 217, 131,
            193, 213, 23, 225, 128, 244, 135, 95, 128, 181, 127, 159, 195, 219, 176, 152, 16, 186,
            80, 5, 143,
        ];
        let ic0_point = bytes_to_g1(ic0);
        let ic1: [u8; 96] = [
            17, 158, 199, 19, 137, 211, 161, 248, 118, 149, 250, 145, 46, 221, 160, 86, 40, 165,
            110, 198, 160, 203, 188, 84, 210, 83, 159, 176, 113, 111, 10, 235, 192, 243, 242, 110,
            188, 210, 98, 199, 74, 66, 118, 251, 3, 188, 58, 84, 23, 55, 88, 168, 37, 240, 121,
            248, 22, 139, 165, 151, 47, 163, 72, 114, 80, 152, 28, 160, 76, 58, 252, 162, 110, 107,
            202, 173, 57, 219, 79, 119, 196, 249, 84, 215, 233, 11, 59, 50, 204, 23, 149, 64, 88,
            135, 163, 190,
        ];
        let ic1_point = bytes_to_g1(ic1);
        let ic1_point = bytes_to_g1(ic1);
        let ic1mul0 = ic1_point * Scalar::from(0);
        let acc = ic0_point + ic1mul0;
        let left = pairing(&alpha_g1_point, &beta_g2_point);
        let leftb = gt_to_bytes(left);
        let right1 = pairing(&a_point, &b_point);
        let neg_gamma_g2_point = -gamma_g2_point;
        let right2 = pairing(&G1Affine::from(acc), &neg_gamma_g2_point);
        let neg_delta_g2_point = -delta_g2_point;
        let right3 = pairing(&c_point, &neg_delta_g2_point);
        let right = right1 + right2 + right3;
        assert_eq!(left, right);
    }
}