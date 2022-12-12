#include <iostream>
#include <vector>
using namespace std;

vector<int> maxSlidingWindow(vector<int> &nums, int k)
{
    vector<int> ans = vector<int>();
    bool wasFirst = true;
    int size = nums.size();
    int last_max = nums[0];
    // [1, 2, 4, 2, 7, 8]
    for (int i = 0; i < (size - 1); i++)
    {
        int max = nums[i];
        if (wasFirst)
        {
            for (int j = i; j < i + k; j++)
            {
                if (nums[j] > max)
                {
                    max = nums[j];
                    wasFirst = false;
                }
                else
                {
                    wasFirst = true;
                }
            }
            last_max = max;
            ans.push_back(last_max);
        }
        else
        {
            int ele = nums[i + (k - 1)];
            bool isLarge = last_max < ele;
            if (isLarge)
            {
                last_max = ele;
                ans.push_back(last_max);
            }
            else
            {
                if (last_max == max)
                {
                    wasFirst = true;
                }
                ans.push_back(last_max);
            }
        }
    }
    return ans;
}

int main() {
    //{1,3,-1,-3,5,3,6,7};
    vector<int> vect = vector<int>(8);
    vect.push_back(1);
    vect.push_back(3);
    vect.push_back(-1);
    vect.push_back(-3);
    vect.push_back(5);
    vect.push_back(3);
    vect.push_back(6);
    vect.push_back(7);
    vector<int> mw = maxSlidingWindow(vect, 3);
    for(auto i:mw) {
        cout<<i<<endl;
    }
    return 0;
}