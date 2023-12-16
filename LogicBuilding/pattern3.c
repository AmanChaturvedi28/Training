#include <stdio.h>

int main() {
    int n=4;
    for(int i=1;i<=n;i++){
        for(int j=i;j<=2*i-1;j++){
            printf("%d",j);
        }
        for(int j=2*(i-1);j>=i;j--){
            printf("%d",j);
        }
    }
    return 0;
}
