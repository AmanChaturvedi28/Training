# Question: Program that reads a number input that should be atleast 4 digit long then apply P&C and find prime numbers from it. Highlight the prime numbers in pascal triangle which is from user input.

## Code: 



### Explanation:
generateDigitPermutations(input):
Takes an integer input and generates all unique permutations of its digits.
Returns an array of unique permutations.

isPrime(num):
Checks if a given number is a prime number.
Returns true if the number is prime, otherwise false.

generatePascalsTriangle(rows, maxPrime):
Generates Pascal's Triangle up to a specified number of rows or until a maximum prime number is reached.
Returns a 2D array representing the triangle.

Event Listener:
Listens for a click event on an HTML element with the ID "submitButton."

Button Click Handler:
Reads an input number from an HTML input element ("inputNumber").
Validates the input, ensuring it's a valid number and at least 1000.
Finds the maximum prime number less than or equal to the input.
Displays the maximum prime number on the page.
Generates Pascal's Triangle up to the maximum prime number.
Identifies prime numbers within the triangle based on certain conditions.
Calculates an error ratio based on prime numbers in the triangle.
Displays prime numbers, error ratio, and primes not included in the triangle.

isSingleDigitPrime(num):
Checks if a number is a prime number and has only a single digit.


#### Author: Aman Chaturvedi
