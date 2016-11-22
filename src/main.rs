
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


fn combine(index_list: Vec<Vec<(u8,u8)>>){

    let per_lane = index_list[0].len();
    println!("{}" , per_lane );
    let to_add : Vec<(u8,u8)> = get_next_list(&index_list);
    let combined: Vec<Vec<(u8,u8)>> = vec![] ;
    for  outer in index_list.iter(){
        //println!("{}" , inde) ;
        for inner in outer.iter(){

            println!("yey: {} {}" , inner.0, inner.1) ;
        }
    }
}


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
//        println!("last {}" ,vals.0);
            next_list.push(vals)
        }
    }

//    let mut next_list:[(u8,u8)] = current_list.iter().map(|row| row[0][0]).collect();
//    println!("{} {}", size , next_list.len());
    return next_list
}

/*
    Takes a 1 d array of tuples and turns into a 2d array
    [1, 2, 3, 4] => [[1] , [2], [3], [4]]
*/
fn to_vec(bindex_pair:(u8,u8) )-> Vec<(u8,u8)> {
    let wrapped: Vec<(u8,u8)> = vec![bindex_pair];
    return wrapped;
}

fn main() {

//    let indexes: [&str;5] = [ "ACACTGTG", "GCGCATAT","AGAGCTCT","ATCGATCG","GCATGCAT"];
    let vindexes = vec![ "ACACTGTG", "GCGCATAT","AGAGCTCT","ATCGATCG","GCATGCAT"];
    let bindexes = convert_to_bin(vindexes); // returns a vector of tuples Vec[(u8,u8), (u8,u8), (u8,u8),]
    let wrapped_bindexes:Vec<Vec<(u8,u8)>> = bindexes.iter().map(|bindex| to_vec(*bindex)).collect();
    combine( wrapped_bindexes );

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
    5,6    15

    1,2,3
    1,2,4
    1,2,5
    1,2,6
    1,3,4
    1,3,5
    1,3,6
    1,4,5
    1,4,6
    1,5,6
    2,3,4
    2,3,5
    2,3,6
    2,4,5
    2,4,6
    2,5,6
    3,4,5
    3,4,6
    3,5,6
    4,5,6   20

*/
