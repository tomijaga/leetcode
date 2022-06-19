impl Solution {
    pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        products.sort();
        
        let mut res = vec![];
        
        let (mut l, mut r) = (0, products.len() - 1);
        
        for i in 0..search_word.len(){
            let slice = &search_word[..=i];
            
            r = products.len();
            
            while(l < r){
                let mid = l + (r - l)/2;
                let prod = &products[mid][..];
                
                // println!("{:?}", (prod, slice));
                // println!("{:?}", prod.cmp(slice));
                
                if prod > slice {
                    r = mid;
                }else if &products[mid][..] < slice{
                    l = mid + 1;
                }else{
                    r = mid;
                }
            }
            
            let mut d2 = vec![];
            let mut cnt = l;
            
            while
            (
                cnt < products.len() && 
                products[cnt].starts_with(slice) && 
                cnt - l < 3
            ) 
            {
                d2.push(products[cnt].clone());
                cnt+=1;
            }
            
            res.push(d2);
        }
        
        res
    }
}