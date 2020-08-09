//! floydrivest
//!
//! `floydrivest` is a small and extremely lightweight crate that provides
//! an in-place implementation of the Floyd-Rivest algorithm.
use std::cmp::{max, min, Ordering};
/// Moves the n-th element of the given Vector in the n-th position
/// by using the Floyd-Rivest algorithm with linear time complexity.
///
/// Similar to its c++ counterpart.
///
///
/// # Examples
///
/// ```
/// let mut v = vec![10, 7, 9, 7, 2, 8, 8, 1, 9, 4]; // a vector of i64.
/// floydrivest::nth_element(&mut v, 3, &mut Ord::cmp);
///
/// assert_eq!(v[3], 7);
/// ```
///
/// # Panics
///
/// if `left`, `right` or `nth_el` are out of bounds
pub fn nth_element<T, F>(a: &mut [T], nth_el: usize, cmp: &mut F)
where
    F: FnMut(&T, &T) -> Ordering,
    T: Clone,
{
    floydrivest(a, nth_el, 0, a.len() - 1, cmp);
}

fn floydrivest<T, F>(a: &mut [T], nth_el: usize, mut left: usize, mut right: usize, cmp: &mut F)
where
    F: FnMut(&T, &T) -> Ordering,
    T: Clone,
{
    let mut i: usize;
    let mut j: usize;
    let mut t: T;
    while right > left {
        if right - left > 600 {
            // Use recursion on a sample of size s to get an estimate
            // for the (nth_el - left + 1 )-th smallest elementh into a[nth_el],
            // biased slightly so that the (nth_el - left + 1)-th element is expected
            // to lie in the smallest set after partitioning.
            let n: f64 = (right - left + 1) as f64;
            let i: f64 = (nth_el - left + 1) as f64;
            let z: f64 = n.ln();
            let s: f64 = 0.5 * (z * (2.0 / 3.0)).exp();
            let sn: f64 = s / n;
            let sd: f64 = 0.5 * (z * s * (1.0 - sn)).sqrt() * (i - n * 0.5).signum();

            let isn: f64 = i * s / n;
            let inner: f64 = nth_el as f64 - isn + sd;
            let ll: usize = max(left, inner as usize);
            let rr: usize = min(right, (inner + s) as usize);
            floydrivest(a, nth_el, ll, rr, cmp);
        }
        // The following code partitions a[l : r] about t, it is similar to Hoare's
        // algorithm but it'll run faster on most machines since the subscript range
        // checking on i and j has been removed.
        t = a[nth_el].clone();
        i = left;
        j = right;
        a.swap(left, nth_el);
        if cmp(&a[right], &t) == Ordering::Greater {
            a.swap(right, left);
        }
        while i < j {
            a.swap(i, j);
            i += 1;
            j -= 1;
            while cmp(&a[i], &t) == Ordering::Less {
                i += 1;
            }
            while cmp(&a[j], &t) == Ordering::Greater {
                j -= 1;
            }
        }
        if cmp(&a[left], &t) == Ordering::Equal {
            a.swap(left, j);
        } else {
            j += 1;
            a.swap(j, right);
        }
        // Now we adjust left and right so that they
        // surround the subset containing the
        // (k - left + 1)-th smallest element.
        if j <= nth_el {
            left = j + 1;
            if nth_el <= j {
                right = j.saturating_sub(1);
            }
        }
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::nth_element;
    #[test]
    fn test_simple() {
        let mut v = vec![10, 7, 9, 7, 2, 8, 8, 1, 9, 4];
        nth_element(&mut v, 3, &mut Ord::cmp);
        assert_eq!(v[3], 7);
    }
    #[test]
    #[cfg(not(tarpaulin_include))]
    fn test_iter() {
        let mut v = vec![9, 5, 0, 6, 8, 2, 3, 7, 1, 4];
        for n in 0..10 {
            nth_element(&mut v, n, &mut Ord::cmp);
            assert_eq!(v[n], n);
        }
    }
    #[test]
    #[cfg(not(tarpaulin_include))]
    fn big_test() {
        // “a man has to do what a man has to do“.
        let mut v: Vec<u64> = vec![
            1069, 460, 271, 127, 766, 633, 939, 175, 928, 388, 404, 12, 949, 1104, 403, 3, 387,
            208, 440, 206, 355, 717, 671, 524, 931, 700, 90, 1018, 1139, 120, 438, 1016, 918, 685,
            777, 1091, 407, 177, 576, 31, 1155, 690, 695, 546, 570, 1134, 369, 900, 872, 985, 784,
            18, 698, 93, 214, 1159, 786, 972, 1098, 1087, 227, 466, 953, 276, 649, 76, 70, 233,
            616, 1167, 1057, 982, 693, 1158, 298, 543, 66, 1090, 1199, 61, 153, 825, 267, 289,
            1165, 260, 477, 322, 526, 277, 1166, 716, 154, 618, 263, 1006, 961, 308, 1188, 571,
            262, 1182, 410, 707, 764, 779, 674, 378, 684, 291, 491, 813, 248, 658, 1074, 704, 980,
            1110, 62, 1086, 986, 934, 981, 268, 1079, 97, 964, 320, 845, 166, 336, 608, 778, 998,
            32, 1083, 604, 351, 1176, 396, 364, 503, 861, 133, 805, 146, 735, 265, 354, 1105, 875,
            1175, 141, 224, 920, 639, 947, 1154, 528, 152, 1172, 203, 235, 740, 489, 376, 663, 897,
            1078, 598, 332, 230, 149, 846, 1144, 819, 547, 135, 871, 1170, 1116, 587, 84, 434,
            1040, 956, 1156, 7, 645, 58, 464, 155, 346, 890, 1128, 623, 893, 418, 1064, 865, 1030,
            1112, 205, 374, 703, 1003, 942, 821, 609, 832, 461, 327, 105, 1140, 877, 411, 490,
            1096, 273, 1178, 811, 313, 272, 510, 53, 377, 428, 199, 1034, 824, 755, 973, 285, 186,
            830, 523, 11, 306, 416, 1075, 607, 891, 1000, 165, 889, 927, 798, 1150, 357, 562, 1180,
            640, 130, 1163, 453, 672, 393, 282, 701, 680, 25, 926, 868, 433, 746, 1010, 807, 50,
            340, 1095, 554, 16, 963, 1067, 429, 232, 349, 129, 408, 885, 21, 783, 1198, 168, 1021,
            1119, 622, 856, 781, 48, 117, 219, 363, 1162, 898, 123, 678, 691, 456, 431, 780, 49, 6,
            63, 581, 366, 1049, 485, 373, 228, 1115, 30, 886, 391, 370, 250, 1011, 599, 1191, 399,
            380, 421, 957, 1004, 762, 809, 417, 850, 218, 100, 1179, 744, 741, 499, 106, 537, 991,
            943, 257, 673, 1122, 1127, 597, 330, 79, 610, 77, 822, 47, 94, 139, 922, 352, 603, 287,
            51, 1164, 236, 459, 211, 280, 1028, 488, 171, 713, 770, 692, 353, 994, 974, 144, 213,
            220, 87, 968, 535, 1017, 494, 1072, 343, 1126, 916, 73, 295, 1073, 768, 167, 188, 675,
            33, 540, 251, 344, 68, 1118, 896, 409, 495, 222, 395, 564, 382, 548, 109, 687, 733,
            722, 361, 452, 9, 1145, 207, 468, 185, 797, 212, 952, 1130, 615, 521, 1015, 617, 549,
            559, 884, 661, 310, 1117, 579, 98, 497, 1153, 929, 539, 852, 862, 297, 1184, 983, 368,
            445, 193, 843, 1019, 474, 1042, 585, 575, 908, 925, 536, 86, 867, 195, 574, 878, 23,
            512, 525, 274, 907, 142, 566, 621, 19, 641, 760, 593, 790, 269, 835, 2, 855, 241, 944,
            13, 80, 1012, 847, 181, 52, 667, 394, 442, 550, 125, 1157, 782, 367, 676, 853, 726,
            400, 864, 962, 1190, 728, 493, 793, 375, 455, 1168, 1001, 283, 229, 1113, 989, 1174,
            999, 1050, 469, 210, 910, 1068, 682, 1147, 683, 795, 1171, 249, 702, 1076, 178, 398,
            990, 670, 1187, 162, 1045, 202, 151, 1008, 350, 119, 930, 556, 479, 246, 114, 1194,
            860, 190, 802, 1161, 323, 914, 1136, 727, 995, 345, 478, 826, 492, 192, 1100, 482, 34,
            476, 42, 595, 196, 1143, 107, 912, 385, 27, 278, 1071, 406, 596, 948, 1103, 630, 145,
            15, 1032, 83, 538, 1046, 904, 184, 1141, 1189, 725, 1038, 666, 1058, 436, 338, 102,
            1108, 906, 405, 650, 432, 454, 136, 644, 347, 95, 335, 496, 903, 1002, 342, 881, 59,
            655, 1183, 317, 734, 138, 552, 164, 131, 82, 99, 1111, 69, 231, 1005, 711, 174, 137,
            191, 573, 938, 563, 348, 14, 359, 324, 758, 401, 72, 180, 812, 729, 1066, 509, 480,
            302, 984, 110, 664, 463, 901, 54, 844, 773, 971, 281, 176, 112, 470, 600, 37, 292,
            1080, 498, 1109, 128, 201, 1114, 1146, 1181, 458, 38, 551, 329, 688, 732, 22, 303, 358,
            720, 0, 748, 763, 321, 481, 506, 1094, 419, 1160, 637, 876, 545, 372, 776, 806, 708,
            160, 1107, 103, 751, 334, 422, 636, 628, 627, 1, 1197, 234, 831, 718, 738, 1102, 993,
            252, 955, 1053, 1142, 899, 381, 769, 475, 447, 665, 325, 560, 448, 288, 362, 965, 601,
            800, 611, 839, 414, 1123, 586, 902, 642, 1149, 654, 960, 772, 43, 567, 873, 457, 81,
            736, 577, 827, 238, 669, 158, 75, 296, 187, 204, 143, 619, 360, 446, 951, 823, 534,
            508, 796, 254, 719, 723, 978, 301, 1048, 134, 92, 65, 624, 1044, 874, 397, 444, 632,
            316, 710, 840, 10, 959, 882, 514, 987, 28, 240, 279, 555, 501, 1061, 124, 1026, 794,
            620, 635, 1007, 424, 946, 970, 808, 712, 646, 1031, 833, 435, 1093, 1077, 866, 247, 78,
            869, 1186, 225, 589, 1177, 1051, 1101, 1133, 449, 532, 173, 919, 1120, 887, 1054, 1148,
            880, 1039, 638, 467, 578, 339, 312, 384, 426, 605, 172, 472, 631, 851, 651, 89, 41,
            542, 753, 892, 924, 243, 747, 791, 441, 115, 465, 1056, 427, 1088, 1082, 64, 425, 660,
            148, 67, 656, 750, 582, 817, 1185, 430, 888, 314, 696, 96, 591, 88, 194, 905, 56, 838,
            290, 602, 1081, 121, 1060, 879, 253, 730, 940, 590, 305, 286, 113, 648, 17, 634, 486,
            1106, 706, 626, 1151, 386, 122, 439, 686, 894, 299, 502, 284, 1025, 365, 721, 1173,
            215, 913, 810, 333, 304, 239, 689, 761, 71, 189, 36, 1129, 765, 1022, 1131, 1092, 612,
            217, 522, 836, 356, 997, 553, 818, 1132, 126, 923, 625, 1037, 742, 724, 917, 789, 774,
            941, 519, 530, 588, 132, 520, 505, 46, 557, 816, 771, 517, 237, 829, 111, 91, 266, 527,
            1124, 40, 568, 55, 1196, 390, 328, 170, 20, 996, 767, 799, 392, 1063, 255, 828, 792,
            443, 420, 104, 26, 849, 1036, 1055, 1137, 1047, 1027, 101, 541, 1059, 402, 662, 311,
            969, 294, 1138, 857, 788, 183, 198, 705, 775, 118, 35, 293, 1135, 256, 870, 518, 1085,
            1084, 179, 752, 739, 572, 858, 1024, 911, 1192, 1070, 915, 584, 261, 169, 412, 383,
            1043, 834, 950, 759, 318, 1169, 244, 1052, 801, 958, 57, 694, 613, 307, 909, 657, 841,
            697, 471, 1193, 511, 837, 157, 300, 221, 988, 544, 754, 413, 156, 967, 85, 932, 653,
            745, 1065, 583, 1097, 936, 659, 652, 516, 606, 150, 699, 24, 820, 1023, 223, 29, 815,
            785, 451, 737, 731, 326, 487, 937, 379, 921, 580, 242, 945, 507, 500, 39, 259, 108,
            161, 935, 848, 5, 450, 814, 275, 45, 462, 415, 270, 558, 933, 854, 614, 533, 163, 159,
            756, 513, 337, 473, 979, 341, 976, 264, 8, 859, 389, 977, 116, 975, 1089, 757, 331,
            565, 561, 714, 569, 647, 804, 147, 216, 258, 437, 209, 681, 515, 883, 1121, 504, 966,
            319, 483, 1020, 842, 787, 709, 803, 531, 895, 629, 679, 1062, 1152, 309, 226, 668, 743,
            954, 1009, 74, 423, 1125, 749, 197, 140, 1099, 315, 4, 60, 592, 1195, 371, 863, 44,
            677, 182, 992, 245, 1041, 594, 200, 1014, 484, 1013, 1035, 715, 1033, 1029, 643, 529,
        ];
        for n in 0..(v.len() - 1) {
            nth_element(&mut v, n, &mut Ord::cmp);
            assert_eq!(v[n], n as u64);
        }
    }
}
