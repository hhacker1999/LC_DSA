#include <iostream>
#include <unordered_map>
using namespace std;

int lengthOfLongestSubstring(string s)
{
    int length = s.length();
    int max_len = 0;
    int max_len_ptr = 0;
    unordered_map<char, int> string_map = unordered_map<char, int>();
    for (int i = 0; i < length; i++)
    {

        const char at = s.at(i);

        if (string_map.find(at) == string_map.end())
        {
            string_map.insert(pair<char, int>(at, i));
            max_len_ptr += 1;
        }
        else
        {
            if (max_len_ptr > max_len) {
                max_len_ptr = max_len;
                max_len_ptr = 0;
                string_map.clear();
            }

        }
    }
    if (max_len == 0) max_len = length;
    return max_len;
}

int main()
{
    lengthOfLongestSubstring(string("asfsad"));
    return 0;
}