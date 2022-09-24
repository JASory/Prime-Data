use number_theory::NumberTheory;

// Global variables that control the output, generally speaking you only want to change either THREAD_count and/or PI_STRIDE. 
// Everything else is computed from it to produce a easily readable  table

/* 
   This particular configuration takes approximately 25hrs to compute on a six core i5-10400, smaller pi strides run faster but are more verbose 
*/
const THREAD_COUNT : u128 = 8; 
const PI_STRIDE : u128 = 8589934592; // 2^33   try to keep it a power of two
const POINTS : u128 = 64; // Number of elements per block 
const B_STRIDE : u128 = PI_STRIDE*POINTS; 
const FULL_LEN : u128 = B_STRIDE*THREAD_COUNT; 

const PRIME_INV_128: [u128; 64] = [
    
          0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab ,  0xcccccccccccccccccccccccccccccccd ,  
          0xb6db6db6db6db6db6db6db6db6db6db7 ,  0xa2e8ba2e8ba2e8ba2e8ba2e8ba2e8ba3 ,
	  0xc4ec4ec4ec4ec4ec4ec4ec4ec4ec4ec5 ,  0xf0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f1 , 
	  0xbca1af286bca1af286bca1af286bca1b ,  0x4de9bd37a6f4de9bd37a6f4de9bd37a7 ,
	  0xc234f72c234f72c234f72c234f72c235 ,  0xdef7bdef7bdef7bdef7bdef7bdef7bdf ,  
	  0xc1bacf914c1bacf914c1bacf914c1bad ,  0x18f9c18f9c18f9c18f9c18f9c18f9c19 ,
	  0xbe82fa0be82fa0be82fa0be82fa0be83 ,  0x3677d46cefa8d9df51b3bea3677d46cf ,  
	  0x13521cfb2b78c13521cfb2b78c13521d ,  0x8f2fba9386822b63cbeea4e1a08ad8f3 ,
	  0x14fbcda3ac10c9714fbcda3ac10c9715 ,  0xc2dd9ca81e9131abf0b7672a07a44c6b ,  
	  0x4f52edf8c9ea5dbf193d4bb7e327a977 ,  0x3f1f8fc7e3f1f8fc7e3f1f8fc7e3f1f9 ,
	  0xd5df984dc5abbf309b8b577e613716af ,  0x2818acb90f6bf3a9a3784a062b2e43db ,  
	  0xd1fa3f47e8fd1fa3f47e8fd1fa3f47e9 ,  0x5f02a3a0fd5c5f02a3a0fd5c5f02a3a1 ,
	  0xc32b16cfd7720f353a4c0a237c32b16d ,  0xd0c6d5bf60ee9a18dab7ec1dd3431b57 ,  
	  0xa2b10bf66e0e5aea77a04c8f8d28ac43 ,  0xc0964fda6c0964fda6c0964fda6c0965 ,
	  0xc090fdbc090fdbc090fdbc090fdbc091 ,  0xbf7efdfbf7efdfbf7efdfbf7efdfbf7f ,  
	  0xf82ee6986d6f63aa03e88cb3c9484e2b ,  0x21a291c077975b8fe21a291c077975b9 ,
	  0xa2126ad1f4f31ba03aef6ca970586723 ,  0x93c225cc74d50c06df5b0f768ce2cabd ,  
	  0x26fe4dfc9bf937f26fe4dfc9bf937f27 ,   0x685b4fe5e92c0685b4fe5e92c0685b5 ,
	  0x8bc775ca99ea03241f693a1c451ab30b ,  0x513ed9ad38b7f3bc8d07aa27db35a717 ,  
	  0x133caba736c05eb4882383b30d516325 ,   0xe4d3aa30a02dc3eed6866f8d962ae7b ,
	  0x6fbc1c498c05a84f3454dca410f8ed9d ,  0x7749b79f7f5470961d7ca632ee936f3f ,  
	  0x90948f40feac6f6b70bf015390948f41 ,   0xbb207cc0532ae21c96bdb9d3d137e0d ,
	  0x7a3607b7f5b5630e2697cc8aef46c0f7 ,  0x2f514a026d31be7bc0e8f2a76e68575b ,  
	  0xdd8f7f6d0eec7bfb687763dfdb43bb1f ,  0x766a024168e18cf81b10ea929ba144cb ,
	   0xc4c0478bbcecfee1d10c4c0478bbced ,  0x758fee6bac7f735d63fb9aeb1fdcd759 ,   
	   0x77f76e538c5167e64afaa4f437b2e0f ,  0x10fef010fef010fef010fef010fef011 ,
	  0xa020a32fefae680828cbfbeb9a020a33 ,  0xff00ff00ff00ff00ff00ff00ff00ff01 ,  
	  0xf836826ef73d52bcd624fd1470e99cb7 ,  0x3ce8354b2ea1c8cd8fb3ddbd6205b5c5 ,
	  0x8715ba188f963302d57da36ca27acdef ,  0xb25e4463cff13686ee70c03b25e4463d ,  
	  0x6c69ae01d272ca3fc5b1a6b80749cb29 ,  0xf26e5c44bfc61b2347768073c9b97113 ,
	  0xb07dd0d1b15d7cf12591e94884ce32ad ,  0xd2f87ebfcaa1c5a0f02806abc74be1fb ,  
	  0xbe25dd6d7aa646ca7ec3e8f3a7198487 ,  0xbc1d71afd8bdc03458550f8a39409d09 ,
	  
 ];
 
 
  fn format_pi(x: u32) -> String{
    " ".to_owned() + &x.to_string()
  }
 
  fn format_integer_array(slice: &[u32], stride: u128, begin: u128) -> String{

   let mut stringvec = vec![];
   
   for (idx,el) in slice.iter().enumerate(){
   // println!("Step");
     if idx%8 == 0 && idx&1 == 0{
       stringvec.push("\n".to_string());
       stringvec.push("\t".to_string());
       stringvec.push(format_pi(*el));
       stringvec.push(",".to_string());
     }
     else if idx%8 == 7 {
     stringvec.push(format_pi(*el));
       stringvec.push(",".to_string());
       stringvec.push(" // ".to_string() + &(begin+stride*(idx as u128-7)).to_string());
     }
     else if idx%8 != 7{
       stringvec.push(format_pi(*el));
       stringvec.push(",".to_string());
     }
   }
   stringvec.join("")
}
 
 /* The core prime checking function. 
 Note that this is NOT a correct prime-check since we are simply looking to count the primes and compare it 
 against primecount values which can be computed much faster. 
 This strategy is around 4 times faster than individually proving each integer that passes this test
 */
 
 fn search_check(x: u128) -> bool{
   if x&1==0{
     return false
   }
   
   for i in PRIME_INV_128{
     if i.wrapping_mul(x) < x{
       return false
     }
   }
   
   x.is_sprp(&2)
  
  }
  
  
  fn pi_block(begin:u128) -> String{

   
   let mut pi = vec![];
   let mut pseudo_string = "\n 2-SPRPs : ".to_string();
    for i in 0..POINTS{
   let start = begin+ PI_STRIDE*i; 
   let stop = begin+ PI_STRIDE*(i+1);
   let mut count = 0u32;
   for j in start..stop{
    if search_check(j){
     count+=1;
      if !j.is_sprp(&15){// use base-15 to help detect pseudoprimes this eliminates most primecount searches although it slows down the actual computation
        pseudo_string = pseudo_string.to_owned() + &j.to_string() + ",";
      }
    }
   }
   pi.push(count);
  }
  return pseudo_string.to_owned() + "\n" + &format_integer_array(&pi[..], PI_STRIDE, begin) // Format data into a string when completed
}

// Constructs one 64-element block per thread
fn pi_parallel(begin: u128) -> String {

  let mut thread_vec = vec![];
  
  for i in 0..THREAD_COUNT{
  thread_vec.push(std::thread::spawn(move || pi_block(begin+(B_STRIDE*i))) ); 
  }
  let mut stringy = String::new();
  for result in thread_vec{
    stringy = stringy + &result.join().unwrap(); 
  }
  return stringy
  }
  
  // For many threads you may want to write it out into a file instead of printing it. 
fn prime_search(){
  use std::io::*;
  let mut bound = String::new();
   println!("Input the starting point");
  std::io::stdin().read_line(&mut bound).unwrap();
  let begin = bound.trim().parse::<u128>().unwrap();
  println!("Searching from {} to {}", begin, begin + FULL_LEN);
  println!("{}", pi_parallel(begin));
}

fn main(){
prime_search()
}


 
 
 
 
 
	  
