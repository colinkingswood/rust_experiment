
struct PairRef<'a> {
    a: &'a i8,
    b: &'a i8,
}

struct PairInt{
    a: i8,
    b: i8,
}

//fn convert_to_bin(index_list:Vec<&str>) -> Vec<u8>{
//    for  index in index_list.into_iter(){
//        println!("{}" , index) ;
//    }
//}

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
    let int_ac = u8::from_str_radix(&bitstr_ac, 2).unwrap();
    let int_gc = u8::from_str_radix(&bitstr_gc, 2).unwrap();

    println!("{} {}" , bitstr_ac, int_ac);
    println!("{} {}" , bitstr_gc, int_gc);

    (int_ac , int_gc)
}


fn convert_to_bin(index_list:Vec<&str>){
//    let test:Vec<(u8,u8)> = index_list.iter().map(|strindex| (5,6)).collect() ;
    let test:Vec<(u8,u8)> = index_list.iter().map(|strindex| str_to_uints(strindex)).collect() ;

    for &tup in test.iter(){
        println!("{} {}" , tup.0, tup.1) ;
    }
}


fn combine(index_list: Vec<&str>){
    for  index in &index_list{
        println!("{}" , index) ;
    }

}


fn main() {

    let indexes: [&str;5] = [ "ACACTGTG", "GCGCATAT","AGAGCTCT","ATCGATCG","GCATGCAT"];
    let vindexes = vec![ "ACACTGTG", "GCGCATAT","AGAGCTCT","ATCGATCG","GCATGCAT"];

    let struct1 = PairInt { a: 5 , b: 35  } ;
    let struct2 = PairInt { a: 5 , b: 35  } ;
    let struct3 = PairInt { a: 5 , b: 35  } ;
    let struct4 = PairInt { a: 5 , b: 35  } ;
    let struct5 = PairInt { a: 5 , b: 35  } ;


    let index_size: usize = indexes.len() ;
    let mut set_2: [&PairInt;5] = [&struct1, &struct2 , &struct3 , &struct4 , &struct5];
//    let mut set_2: [PairInt;5] = [struct1, struct2 , struct3 , struct4 , struct5];

    let bindexes = convert_to_bin(vindexes);
//    println!("{}" ,bindexes[0]);

    for index in indexes.iter() {
        println!("Hello, world! {}" , index);
    }

    println!("Size of set_2 {}" , set_2.len())
}
