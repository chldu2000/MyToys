#include <iostream>
#include "wujiang.h"
using namespace sanguo;
int main()
{
    color(4);
    cout << "滚滚长江东逝水，浪花淘尽英雄。是非成败转头空。青山依旧在，几度夕阳红。" << endl
         << "白发渔樵江渚上，惯看秋月春风。一壶浊酒喜相逢。古今多少事，都付笑谈中。" << endl;
    cout << endl;
    color(15);
    cout << "主公，欢迎回来！" << endl;
    cout << "您可对武将进行一系列操作，您帐下最多拥有1500名武将。" << endl;
    cout << endl;
    op();
    return 0;
}
