# include <stdio.h>
int main(){
    int prod = 1,n;
    printf("Enter the number to calculate product: ");
    scanf("%d",&n);

    for (int i = 1; i <= n; i++)
    {
        prod = prod * i;
    }
    

    printf("product is: %d",prod);
}