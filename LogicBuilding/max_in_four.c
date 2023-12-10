#include <stdio.h>

int main() {
    int num1, num2, num3, num4, max;

    printf("Enter your num1: ");
    scanf("%d", &num1);

    printf("Enter your num2: ");
    scanf("%d", &num2);

    printf("Enter your num3: ");
    scanf("%d", &num3);

    printf("Enter your num4: ");
    scanf("%d", &num4);

    max = num1;

    if (num2 > max)
    {
        max = num2;
    }

    if (num3 > max)
    {
        max = num3;
    }

    if (num4 > max)
    {
        max = num4;
    }

    printf("Maximum of four numbers is: %d\n", max);    
}