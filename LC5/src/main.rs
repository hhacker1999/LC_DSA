use core::str;

fn main() {
    println!(
        "Longest pString is {:?} ",
        longest_palindrome(String::from("klvxwqyzugrdoaccdafdfrvxiowkcuedfhoixzipxrkzbvpusslsgfjocvidnpsnkqdfnnzzawzsslwnvvjyoignsfbxkgrokzyusxikxumrxlzzrnbtrixxfioormoyyejashrowjqqzifacecvoruwkuessttlexvdptuvodoavsjaepvrfvbdhumtuvxufzzyowiswokioyjtzzmevttheeyjqcldllxvjraeyflthntsmipaoyjixygbtbvbnnrmlwwkeikhnnmlfspjgmcxwbjyhomfjdcnogqjviggklplpznfwjydkxzjkoskvqvnxfzdrsmooyciwulvtlmvnjbbmffureoilszlonibbcwfsjzguxqrjwypwrskhrttvnqoqisdfuifqnabzbvyzgbxfvmcomneykfmycevnrcsyqclamfxskmsxreptpxqxqidvjbuduktnwwoztvkuebfdigmjqfuolqzvjincchlmbrxpqgguwuyhrdtwqkdlqidlxzqktgzktihvlwsbysjeykiwokyqaskjjngovbagspyspeghutyoeahhgynzsyaszlirmlekpboywqdliumihwnsnwjc"))
    );
}

fn longest_palindrome(s: String) -> String {
    let char_array: Vec<char> = s.chars().collect();
    let mut ans: String = String::new();
    for i in 0..char_array.len() {
        let mut start: i32 = i as i32;
        let mut end: i32 = i as i32;
        while start >= 0
            && end < char_array.len() as i32
            && char_array[start as usize] == char_array[end as usize]
        {
            if ans.len() < ((end - start) + 1) as usize {
                ans = s[start as usize..=end as usize].to_string();
            }
            start -= 1;
            end += 1;
        }
        start = i as i32;
        end = (i + 1) as i32;
        while start >= 0
            && end < char_array.len() as i32
            && char_array[start as usize] == char_array[end as usize]
        {
            if ans.len() < ((end - start) + 1) as usize {
                ans = s[start as usize..=end as usize].to_string();
            }
            start -= 1;
            end += 1;
        }
    }
    return ans;
}

fn longest_palindrome_brute(s: String) -> String {
    let mut ans: String = String::new();
    for i in 0..s.len() {
        for j in i..s.len() {
            let slice: &str = &s[i..=j];
            let is_palin: bool = is_palindrome(slice);
            if is_palin && ans.len() < slice.len() {
                ans = slice.to_string();
            }
        }
    }
    return ans;
}

fn is_palindrome(str: &str) -> bool {
    if str.len() == 1 {
        return true;
    }
    let char_array: Vec<char> = str.chars().collect();
    let mut start: usize = 0;
    let mut end: usize = char_array.len() - 1;
    while start < end {
        if char_array[start] != char_array[end] {
            return false;
        }
        start += 1;
        end -= 1;
    }
    return true;
}
