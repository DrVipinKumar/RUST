fn main() {
    let data=[
                              [1,2,3],
                              [4,5,6]
                            ];
    let mut i=0;
    let mut j=0;
    for row in data {
        for value in row {
            print!("data[{}][{}]={} ",i,j, value);
            j+=1;
        }
        i+=1;
        println!();
    }
    // let mut data:[f32;4];
    // data=[10.2,20.2,30.2,40.2]; //single dimensional array
    //array is a collection of values of similiar data types
    // for (i,value) in data.iter().enumerate() {
    //     print!("data[{}]={} ",i,value);        
    //  }
    //  for value in data.iter_mut() {
    //     *value=*value+10 as f32;
    //     print!("data[{}]={} ",i,value);
    //     i+=1;
    //  }
    // for i in 0..4 {
    //     print!("data[{}]={} ",i,i);      
    // }
    // for value in data {
    //         print!("data[{}]={} ",i,value);
    //         i+=1;
    // }
}
