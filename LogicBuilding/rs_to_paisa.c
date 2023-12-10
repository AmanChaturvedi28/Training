# include <stdio.h>
int main(){
    int rs,pa,rsp,pas;
    char c;
    printf("Enter r for rs --> paisa and p for paisa --> rs: ");
    scanf("%c",&c);

    if (c == 'r')
    {
        printf("Enter rs to convert rs --> paisa: ");
        scanf("%d",&rs);
        rsp = rs * 100;
        printf("Paisa is %d",rsp);
    }

    else if (c == 'p')
    {
        printf("Enter paisa to convert paisa --> rs: ");
        scanf("%d",&pa);
        pas = pa / 100;
        printf("rs is %d",pas);
    }
    else{
        printf("Invalid  operation");
    }
    
    
}