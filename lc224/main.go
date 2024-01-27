package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	// fmt.Println(getOperand(true, 3, "2+6+4"))
	// fmt.Println(getOperand(false, 3, "2+6+4"))
	// fmt.Println(simpleEval("2-6+4"))
	// fmt.Println(simpleEval("-2*2"))
	// fmt.Println(simpleEval("2*6-4"))
	fmt.Println(calculate("-(-20-3)"))
}

//(1+(4+5+2)-3)+(6+8)

func calculate(s string) int {
	var cPattern string
	for _, v := range s {
		if v != ' ' {
			cPattern += string(v)
		}

	}
	for strings.Contains(cPattern, "(") &&
		strings.Contains(cPattern, ")") {
		left := -1
		right := -1
		for index, v := range cPattern {
			if v == '(' {
				left = index
			}
			if v == ')' {
				right = index
				break
			}
		}
		ptr := cPattern[left+1 : right]
		res := simpleEval(ptr)
		resStr := strconv.Itoa(res)
		cPattern = cPattern[:left] + resStr + cPattern[right+1:]

	}

	return simpleEval(cPattern)
}

// operators
const PLUS = '+'
const MINUS = '-'
const MUL = '*'
const DIV = '/'
const OF = 'o'

func simpleEval(pattern string) int {
	var result int
	// Find multiply and of
	currentPattern := pattern

	for strings.Contains(currentPattern, "/") {
		for i := 1; i < len(currentPattern)-1; i++ {
			if currentPattern[i] == DIV {
				lo, rl := getOperand(true, i, currentPattern)
				ro, rr := getOperand(false, i, currentPattern)
				result := lo / ro
				currentPattern = currentPattern[:i-rl] + strconv.Itoa(
					result,
				) + currentPattern[i+rr+1:]
			}

		}
	}

	for strings.Contains(currentPattern, "*") {
		for i := 1; i < len(currentPattern)-1; i++ {
			if currentPattern[i] == MUL {
				lo, rl := getOperand(true, i, currentPattern)
				ro, rr := getOperand(false, i, currentPattern)
				result := lo * ro
				currentPattern = currentPattern[:i-rl] + strconv.Itoa(
					result,
				) + currentPattern[i+rr+1:]
			}
		}
	}

	for strings.Contains(currentPattern, "+") || strings.Contains(currentPattern, "-") {
		count := 0
		for _, v := range currentPattern {
			if v == MINUS {
				count += 1
			}
		}
		if count == 1 && currentPattern[0] == MINUS && !strings.Contains(currentPattern, "+") &&
			!strings.Contains(currentPattern, "*") && !strings.Contains(currentPattern, "/") {
			break
		}
		for i, v := range currentPattern {
			if v == PLUS {
				fmt.Println(currentPattern)
				lo, _ := getOperand(true, i, currentPattern)
				ro, rr := getOperand(false, i, currentPattern)
				result := lo + ro
				fmt.Println(result)
				currentPattern = strconv.Itoa(
					result,
				) + currentPattern[i+rr+1:]
				break
			}
			if v == MINUS && i != 0 {
				lo, _ := getOperand(true, i, currentPattern)
				ro, rr := getOperand(false, i, currentPattern)
				result := lo - ro
				currentPattern = strconv.Itoa(
					result,
				) + currentPattern[i+rr+1:]
				fmt.Println(currentPattern)
				break
			}
		}
	}

	// for strings.Contains(currentPattern, "+") {
	// 	fmt.Println(currentPattern)
	// 	for i := 1; i < len(currentPattern)-1; i++ {
	// 		if currentPattern[i] == PLUS {
	// 			lo, rl := getOperand(true, i, currentPattern)
	// 			ro, rr := getOperand(false, i, currentPattern)
	// 			result := lo + ro
	// 			currentPattern = currentPattern[:i-rl] + strconv.Itoa(
	// 				result,
	// 			) + currentPattern[i+rr+1:]
	// 		}
	// 	}
	// }
	//
	// for strings.Contains(currentPattern, "-") {
	// 	fmt.Println(currentPattern)
	// 	count := 0
	// 	for _, v := range currentPattern {
	// 		if v == MINUS {
	// 			count += 1
	// 		}
	// 	}
	// 	if count == 1 && currentPattern[0] == MINUS && !strings.Contains(currentPattern, "+") &&
	// 		!strings.Contains(currentPattern, "*") && !strings.Contains(currentPattern, "/") {
	// 		break
	// 	}
	// 	for i := 1; i < len(currentPattern)-1; i++ {
	// 		if currentPattern[i] == MINUS {
	// 			lo, rl := getOperand(true, i, currentPattern)
	// 			ro, rr := getOperand(false, i, currentPattern)
	// 			result := lo - ro
	// 			currentPattern = currentPattern[:i-rl] + strconv.Itoa(
	// 				result,
	// 			) + currentPattern[i+rr+1:]
	// 			fmt.Println(currentPattern)
	// 		}
	// 	}
	//
	// }
	foo, err := strconv.Atoi(currentPattern)
	if err != nil {
		return result
	}
	result = foo

	return result
}

func getOperand(left bool, index int, pattern string) (int, int) {
	var operand int
	var r int

	if left {
		for i := index - 1; i >= 0; i-- {
			if pattern[i] == PLUS {
				break
			}
			if pattern[i] == MINUS && i != 0 {
				break
			}
			foo := pattern[i:index]
			oper, err := strconv.Atoi(foo)
			if err != nil {
				break
			}
			r = r + 1
			operand = oper
		}
	} else {
		if index == len(pattern)-2 {
			foo := pattern[index+1:]
			oper, err := strconv.Atoi(foo)
			if err != nil {
				fmt.Println(err)
			} else {
				operand = oper
			}
		}
		r = 1
		for i := index + 2; i < len(pattern); i++ {
			var oper int
			var err error
			if i+1 == len(pattern) {
				foo := pattern[index+1:]
				oper, err = strconv.Atoi(foo)
			} else {
				foo := pattern[index+1 : i]
				oper, err = strconv.Atoi(foo)
			}
			if err != nil {
				break
			}
			operand = oper
			r = r + 1
		}
	}
	foo := strconv.Itoa(operand)
	return operand, len(foo)
}
