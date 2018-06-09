#include<stdio.h>
#define add(a,b) a+b
char s[100];
char user[100]="WangHaoyu";
int version=3;
struct tire{
	int color;
	int price;
};
struct car{
	int price,height;
	struct tire a[4];
	//char number[10];
	char* owner;
};
int sum(int x,int y){
	return x+y;
}
char* getcar(struct car*  x){
	int p=x->price,i;
	//char s[50];
	//printf("1");
	for (i=0;i<4;i++){
		printf("%d\n",i);
		p+=x->a[i].price;
	}
	//sprintfs(s,"The number of %s's car is %s,which values %d dollars.",x.owner,x.number,p);
	puts(x->owner);
	sprintf(s,"The price of %s's car is %d dollars.",x->owner,p);
	puts(s);
	return s;
}
/*
int main(){
	struct car x;
	int i;
	x.price=200;
	for (i=0;i<4;i++){
		x.a[i].price=123;
		x.a[i].color=1;
	}
	printf("%d\n",add(2,4));
	puts(getcar(&x));
	//puts(user);
	//printf("%s",getcar(&x));
	return 0;
}
*/