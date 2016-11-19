
struct PairRef<'a> {
    a: &'a i8,
    b: &'a i8,
}

struct PairInt{
    a: i8,
    b: i8,
}

fn main() {

    let indexes: [&str;5] = [ "ACACTGTG", "GCGCATAT","AGAGCTCT","ATCGATCG","GCATGCAT"];

    let struct1 = PairInt { a: 5 , b: 35  } ;
    let struct2 = PairInt { a: 5 , b: 35  } ;
    let struct3 = PairInt { a: 5 , b: 35  } ;
    let struct4 = PairInt { a: 5 , b: 35  } ;
    let struct5 = PairInt { a: 5 , b: 35  } ;


    let index_size: usize = indexes.len() ;
    let mut set_2: [&PairInt;5] = [&struct1, &struct2 , &struct3 , &struct4 , &struct5];
//    let mut set_2: [PairInt;5] = [struct1, struct2 , struct3 , struct4 , struct5];


    for index in indexes.iter() {
        println!("Hello, world! {}" , index);
    }

    println!("Size of set_2 {}" , set_2.len())
}
