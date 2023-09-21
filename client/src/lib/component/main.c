#include <stdio.h>

int main()
{
    int first, second, max;
    printf("请输入两个整数: ");
    scanf("%d %d", &first, &second);

    max = first;
    if (max < second) {
        max = second;
    }

    printf("第一个数为： %d\n", first);
    printf("第二个数为： %d\n", second);
    printf("大数为： %d\n", max);

    return 0;
}