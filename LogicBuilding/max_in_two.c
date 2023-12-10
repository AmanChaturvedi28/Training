#include <stdio.h>
int main()
{
   int num1, num2, max;

    printf("Enter your num1: ");
    scanf("%d", &num1);

    printf("Enter your num2: ");
    scanf("%d", &num2);
    
    max = num1;
    if (num2 > max)
    {
        max = num2;
    }
    
    printf("Maximum of two numbers is: %d\n", max);
}