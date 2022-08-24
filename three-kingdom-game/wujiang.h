#ifndef WUJIANG_H
#define WUJIANG_H
#include <string>
#include <iomanip>
#include <conio.h>
#include <algorithm>
#include <windows.h>
using namespace std;
namespace sanguo
{
    // int num_wj=0;       //记录武将数量
    class wujiang //武将
    {
    public:
        string name;
        int age;
        int force_ti;
        int force_zhi;
        int force_wu;
        int live; //用来判断武将信息是否有效，添加武将时给该值赋0，删除武将时给该值赋1
        bool sys; //用来判断该武将是否为初始武将，初始武将（true）不可删除
        int order;
    };
    void op();
    void color(int a);
}
#endif
