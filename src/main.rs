
/* take the string representation and convert it to a tuple of u8*/
fn str_to_uints(strindex:&str)->(u8, u8){

    let mut bitstr_ac = String::from("");
    let mut bitstr_gc = String::from("");

    for c in strindex.chars() {
        if c == 'A' || c == 'C' {
            bitstr_ac.push('1') ;
        }
        else{
            bitstr_ac.push('0') ;
        }

        if c == 'G' || c == 'C' {
            bitstr_gc.push('1') ;
        }
        else{
            bitstr_gc.push('0') ;
        }
    }

    // convert sting to u8
    let int_ac = u8::from_str_radix(&bitstr_ac, 2).unwrap();
    let int_gc = u8::from_str_radix(&bitstr_gc, 2).unwrap();

    println!("{} {} {} {} {}" , strindex, bitstr_ac, int_ac, bitstr_gc, int_gc);

    (int_ac , int_gc)
}


fn convert_to_bin(index_list:Vec<&str>)->Vec<(u8,u8)> {
    let binvec: Vec<(u8,u8)> = index_list.iter().map(|strindex| str_to_uints(strindex)).collect() ;
    return binvec
}

////fn convert_to_bin2(index_list:&[&str])->Vec<(u8,u8)> {
//fn convert_to_bin2<'a>(index_list:&[&str])->&'static [(u8,u8)] {
//    //let size: u64 = index_list.len() as u64;
//    //println!("..{}" , size);
//    //let binvec2: [(u8,u8)] = index_list.iter().map(|strindex| str_to_uints(strindex)).collect() ;
//
//    let binvec: Vec<(u8,u8)> = &index_list.iter().map(|strindex| str_to_uints(strindex)).collect() ;
//    return &binvec[..]
//}


/*
Pass in  the following:

[                    outer
    [(u8,u8) , (u8, u8) , ],       inner  (second round)
    [(u8,u8) ,  ],                 inner  (first round)
    [(u8,u8) ,  ],

]
*/
fn combine(index_list: Vec<Vec<(u8,u8)>>, initial:&[(u8,u8)], mut start_pos:usize)->Vec<Vec<(u8,u8)>>{
//    fn combine(index_list: Vec<Vec<(u8,u8)>>, initial:&Vec<(u8,u8)>, mut start_pos:usize)->Vec<Vec<(u8,u8)>>{

/*
    [
      [(1,2)] ,
      [(3,5)] ,
    ]

*/

//    let to_add = ;
    let per_lane = index_list[0].len();
    println!("{}" , per_lane );
    println!("sp:{}" , start_pos );

//    let to_add : Vec<(u8,u8)> = get_next_list(&index_list);
    let mut combined: Vec<Vec<(u8,u8)>> = vec![] ;
//    let mut combined: Vec<Vec<(u8,u8)>> = Vec::with_capacity() ;

    for  outer in index_list.iter(){
        //println!("{}" , inde) ;
        for next_index in initial.iter(){

//            println!("yey: {} {}" , inner.0, inner.1) ;
            let mut can_combine = true ;
            for inner in outer.iter(){
//                println!("--{} {}", next_index.0, next_index.1);
                let ac: u8 = next_index.0 ^ inner.0;
                let gc: u8 = next_index.1 ^ inner.1;
                let mis_b: u8 = ac | gc ;
                if mis_b.count_ones() < 3{
                    can_combine = false;
                }
            }

            if can_combine == true {
                let mut new = outer.clone();
                new.push(next_index.clone());
                combined.push(new);
            }

        }
        start_pos += 1;
    }
    println!("'here'");
//    print_matrix(combined);

    return combined;
}

fn print_matrix(matrix:Vec<Vec<(u8,u8)>>){

    for outer in matrix.iter(){
        for inner in outer.iter(){
            print!("({} {}) ", inner.0, inner.1);
        }
        println!("");
    }
}

//fn combine2(index_slice:&[(u8,u8)]){
//    let to_add = get_next_list2(index_slice);
//    println!(".{}", index_slice.len());
//}
//

/*
    takes a 2d array and returns the last column,
    so it can be combined again
    1,2,3         3
    2,3,4    =>   4
    3,4,5         5

*/
fn get_next_list(current_list:&Vec<Vec<(u8,u8)>>)->Vec<(u8,u8)>{
//    let size: usize = current_list.len();
    let mut next_list: Vec<(u8,u8)> = vec![] ;
    for (i, row) in current_list.iter().enumerate(){
        if i != 0 {
            let vals: (u8,u8) = row[0] ;
            next_list.push(vals)
        }
    }
    return next_list
}



fn test(current_list:&[(u8,u8)])->usize{
    let blah = [1,2,3 ,4,5];
    return blah.len()
}

//////->[(u8,u8)]
//fn get_next_list2(current_list:&[(u8,u8)])->u8{
//////    let size: usize = current_list.len();
////    let mut next_list: Vec<(u8,u8)> = vec![] ;
//////    let mut next_list: &[(u8,u8)]  = [0;current_list.len() - 1];
////    for (i, row) in current_list.iter().enumerate(){
////        println!("..{}", row.1) ;
////        if i != 0 {
////            let vals: (u8,u8) = *row ;
////            next_list.push(vals)
////        }
////    }
//////    return &next_list.as_slice()
//}
////

/*
    Takes a 1 d array of tuples and turns into a 2d array
    [1, 2, 3, 4] => [[1] , [2], [3], [4]]
*/
fn to_vec(bindex_pair:(u8,u8) )-> Vec<(u8,u8)> {
    let wrapped: Vec<(u8,u8)> = vec![bindex_pair];
    return wrapped;
}

//fn capacity_needed(length, depth){
//
//}

fn main() {

    let vindexes = vec!["ACACTGTG",
                        "GCGCATAT",
                        "AGAGCTCT",
                        "ATCGATCG",
                        "GCATGCAT",
                        "CGCGATAT",
                        "ATATCGGC",
                        "GTGTCACA",
                        "CCGGAATT",
                        "CGGCCTTT",
                        "ACACTGTG",
                        "GCTTTTAT",
                        "AGCGGTCT",
                        "TCGATACG",
                        "ATGGCCAT",
                        "GATATCGC",
                        "ATACCGGC",
                        "GGATCACA",
                        "GGGTTATT",
                        "CCAAAAAT",
                        "ACATGCTG",
                        "GCTTATTT",
                        "CGGTAGCT",
                        "TCGATTTG",
                        "AATTTAAT",
                        //"GATATCCC",
                        //"ATAGGGGG",
                        //"AAATTTTT",
                        //"CCGTTTTT",
                        "CGGGCAAT",
                        ];
    let bindexes = convert_to_bin(vindexes); // returns a vector of tuples Vec[(u8,u8), (u8,u8), (u8,u8),]
    let initial = bindexes.clone();

    let wrapped_bindexes:Vec<Vec<(u8,u8)>> = bindexes.iter().map(|bindex| to_vec(*bindex)).collect();
    let combined1 = combine( wrapped_bindexes, &initial[1..], 1 );
    let combined2 = combine(combined1, &initial[2..], 2 );
    let combined3 = combine(combined2, &initial[3..], 3 );
    let combined4 = combine(combined3, &initial[4..], 4 );
    let combined5 = combine(combined4, &initial[5..], 5 );
//    let combined6 = combine(combined5, &initial[5..], 5 );
//    let combined7 = combine(combined6, &initial[5..], 5 );

//    print_matrix(combined5);
    println!("{}" , combined5.len());
}

/*
    1
    2
    3
    4
    5
    6    6

    1,2
    1,3
    1,4
    1,5
    1,6
    2,3
    2,4
    2,5
    2,6
    3,4
    3,5
    3,6
    4,5
    4,6
    5,6    15  5+4+3+2+1

    1,2,3
    1,2,4
    1,2,5
    1,2,6
    1,3,4
    1,3,5
    1,3,6
    1,4,5
    1,4,6
    1,5,6   4+3+2+1
    2,3,4
    2,3,5
    2,3,6
    2,4,5
    2,4,6
    2,5,6   3+2+1
    3,4,5
    3,4,6
    3,5,6   2+1
    4,5,6   20  5+4+3+2+1 + 4+3+2+1 + 3+2+1 + 2+1 + 1

    1,2,3,4
    1,2,3,5
    1,2,3,6  3
    1,2,4,5
    1,2,4,6  2
    1,2,5,6  1
    1,3,4,5  2
    1,3,4,6
    1,3,5,6  1
    1,4,5,6  1
    2,3,4,5  2
    2,3,4,6
    2,3,5,6  1
    2,4,5,6  1
    3,4,5,6  1     15

*/
