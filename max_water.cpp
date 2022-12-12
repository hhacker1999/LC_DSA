int min(int one, int two) {
    if (one < two) {
        return one;
    }
    return two;
}

// Normies Solution
int maxArea(vector<int> &height)
{
    int max_water = -1;
    int size = height.size();
    for (int i = 0; i < size - 1 ; i++) {
        for (int j = i; j < size; j++) {
            int minimum = min(height[i], height[j]);
            int current_water = minimum * (j - i);
            if(current_water > max_water) {
                max_water = current_water;
            }
        }
    }
    return max_water;
}

// Chad Solution 
int maxArea2(vector<int> &height)
{
    int max_water = -1;
    int size = height.size();
    int end = size - 1;
    int start = 0;
    while (start < size - 1) {
        int minimum = min(height[start], height[end]);
        int current_water = minimum * (end - start);
        if (current_water > max_water) {
            max_water = current_water;
        }
        if (minimum == height[start]) {
            start +=1;
        }
        else {
            end -= 1;
        }
    }
    return max_water;
}