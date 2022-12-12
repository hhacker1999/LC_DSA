vector<vector<string>> valid_anagram(vector<string> str)
{
    vector<int> vec1 = vector<int>(26, 0);
    vector<int> vec2 = vector<int>(26, 0);
    vector<vector<string>> ans = vector<vector<string>>();
    unordered_map<int, string> map;
    int size = str1.length();
    for (int i = 0; i < size - 1; i++)
    {
        auto itr = map.find(i);
        if (itr == map.find())
        {

            vector<string> curr_list = vector<string>();
            curr_list.push_back(str[j]);
            for (int j = i + 1; j < size; j++)
            {
                bool isana = isValid(str[i], str[j]);
                if (isana)
                {
                    curr_list.push_back(str[j])
                        map.insert(pair<int, string>(j, str[j]));
                }
            }
            ans.push_back(curr_list);
        }
    }

    return ans;
}