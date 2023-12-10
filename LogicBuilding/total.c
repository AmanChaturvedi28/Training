# include <stdio.h>
int main(){
    int tot = 0,n;

    printf("Enter number to calculate total: ");
    scanf("%d",&n);

    for(int i ; i<=n ; i++){
        tot = tot + i;
    }
    printf("Total is: %d",tot);
}