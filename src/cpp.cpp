/**
 * @brief 程序功能：用于求解异构介质配比
 */

#pragma warning(disable : 4996);
#include <iostream>
#include <vector>
#include <algorithm>
#include <climits>
#include <float.h>
#include <string>
#include <string.h>
#include <stdlib.h>
#include <time.h>

#define GRANULARITY 2  // 粒度

using namespace std;

/*
        bitmap:位图，用于计算Footprint
*/

class BitMap {
public:
    BitMap(size_t range) {
        // 此时多开辟一个空间
        _bits.resize(range / 32 + 1);
    }
    void Set(size_t x) {
        int index = x / 32;           // 确定哪个数据（区间）
        int temp = x % 32;            // 确定哪个Bit位
        _bits[index] |= (1 << temp);  // 位操作即可
    }
    void Reset(size_t x) {
        int index = x / 32;
        int temp = x % 32;
        _bits[index] &= ~(1 << temp);  // 置0
    }
    bool Test(size_t x) {
        int index = x / 32;
        int temp = x % 32;
        if (_bits[index] & (1 << temp))
            return 1;
        else
            return 0;
    }

private:
    vector<int> _bits;
};

/*
    根据pattern，切分字符串
*/
vector<string> split(string str, string pattern) {
    string::size_type pos;
    vector<string> result;
    str += pattern;  // 扩展字符串以方便操作
    int size = str.size();
    for (int i = 0; i < size; i++) {
        pos = str.find(pattern, i);
        if (pos < size) {
            string s = str.substr(i, pos - i);
            result.push_back(s);
            i = pos + pattern.size() - 1;
        }
    }
    return result;
}
BitMap bitmap(INT_MAX);
BitMap bitmapread(INT_MAX);

static string PARV_FILE;
static string TRACE_FILE;
static int MAX_CACHE;
static double qosValue;
double rightAvg = 0.0;
double leftAvg = 0.0;

// SSD 每GB容量价格(单位：元)
static double SSD_COST = 0.628;
// HDD 每GB容量价格(单位：元)
static double HDD_COST = 0.552;
// 算法运行总时间
static long long total_time = 0.0;

/**
 * @brief 设置配置文件并运行 disksim
 *
 * @param cache_size 缓存块数量(单位:扇区)
 */
void setParvAndRun(int cache_size) {
    FILE *inputFile;
    FILE *outputFile;
    inputFile = fopen(PARV_FILE.c_str(), "r");
    outputFile = fopen("ssd-cachealloc.parv", "w+");
    char writebuffer[1024];
    system("rm -rf driver_response.txt;");

    while (fgets(writebuffer, 1024, inputFile) != NULL) {
        string strTemp = string(writebuffer);
        char cache_hit[1024];
        if (string::npos != strTemp.find("Cache size")) {
            fprintf(outputFile, "\t\tCache size = %d, \n", cache_size * 4096);
        } else {
            fprintf(outputFile, "%s", strTemp.c_str());
        }
        fflush(stdout);
    }

    fclose(outputFile);
    fclose(inputFile);
    system("rm -rf driver_response.txt");
    string shell = string("../src/disksim ");
    shell.append("ssd-cachealloc.parv ");
    shell.append("qos-cache.outv validate ");
    shell.append(TRACE_FILE.c_str());
    shell.append(" 0");
    system(shell.c_str());

    // 打印命中率
    FILE *outv = fopen("qos-cache.outv", "r");
    vector<string> v;
    while (fgets(writebuffer, 1024, outv) != NULL) {
        char *res = NULL;
        if (strstr(writebuffer, "cache hit") != nullptr) {
            res = strtok(writebuffer, " ");
            while (res != NULL) {
               string str = string(res);
                v.push_back(str);
                // printf("%s\n", res);
                res = strtok(NULL, " ");
            }
        }
        fflush(stdout);
    }
    cout << "Cache hit = " << v[4] << endl;
}

/**
 * @brief 在 driver_response.txt 中计算平均延迟
 *
 * @return double
 */
double calAvg(int &left, int &right, int &mid) {
    double average = 0.0;
    float latency = 0.0;
    int device = 0;
    int offset = 0;
    int length = 0;
    int rw = 0;

    vector<float> simutionLatency;
    FILE *outputFile = fopen("driver_response.txt", "r");
    if (outputFile != NULL) {
        int flags = 0;
        while (5 == fscanf(outputFile, "%f ,opid %d, blkno %d, bcount %d %d\n", &latency, &device, &offset, &length, &flags)) {
            simutionLatency.push_back(latency);
        }
        fclose(outputFile);
    } else {
        printf("OutputFile Open Failed!\n");
        exit(0);
    }

    sort(simutionLatency.begin(), simutionLatency.end());

    for (int i = 0; i < simutionLatency.size(); i++) {
        average += simutionLatency[i];
    }
    average = average / simutionLatency.size();
    printf("Cache size = %d MB, Average = %f ms\n", mid * 2, average);
    // printf("left = %d, right = %d, mid = %d\n", left, right, mid);
    fflush(stdout);

    if (average <= qosValue) {
        right = mid;
        rightAvg = average;
    } else {
        left = mid;
        leftAvg = average;
    }
    // printf("rightAvg = %f\n", rightAvg);
    // printf("leftAvg = %f\n", leftAvg);
    printf("\n");
    return average;
}

/**
 * @brief 介质异构配比算法主程序
 *
 * @param argc 参数个数
 * @param argv 	$0: qos
 * 			   	$1: parv配置文件
 * 				$2: trace 文件
 * 				$3: Cache 最大容量，如 1024 表示 Cache 最大为 1024 个 2M，即 2G
 * 				$4: QoS 要求
 * @return int
 */
int main(int argc, char *argv[]) {
    PARV_FILE = argv[1];
    TRACE_FILE = string(argv[2]);
    MAX_CACHE = stoi(argv[3]);
    qosValue = stof(argv[4]);

    // double qosValue = (double)to_float(argv[4]);

    // SSD容量：左闭区间，右闭区间，二分查找，查找满足QoS时的最优

    int left = 1;
    int right = MAX_CACHE;  // 读取最大的Cache指标
    int isExist = -1;       // 最优值落在哪个指针上

    double average = 0.0;
    time_t start_time = time(NULL);
    while (left + 1 < right) {
        int mid = (right + left) / 2;
        setParvAndRun(mid);
        // system("grep 'IOdriver Response time average' qos-cache.outv");
        average = calAvg(left, right, mid);
    }

    // 如果 right 一直没有变化，需要单独跑一遍 right 指针
    if (right == MAX_CACHE) {
        setParvAndRun(MAX_CACHE);
        average = calAvg(MAX_CACHE, MAX_CACHE, MAX_CACHE);
        rightAvg = average;
    }

    // 如果 left 一直没有变化，需要单独跑一下 left 指针
    if (left == 1) {
        setParvAndRun(1);
        average = calAvg(left, left, left);
        leftAvg = average;
    }

    // 二分结束，最优值可能落在 right 或 left 指针上
    if (rightAvg < leftAvg) {
        average = rightAvg;
        isExist = 1;  // 最优值落在 right 指针
    } else {
        average = leftAvg;
        isExist = 0;  // 最优值落在 left 指针
    }

    FILE *inputFile = fopen(TRACE_FILE.c_str(), "r");
    long long footprint = 0;
    long long volume = 0;
    long long offset = 0;  // 偏移和length均是以块为单位
    long long length = 0;  // 偏移和length均是以块为单位
    long long maxAddress = 0;
    long long minAddress = INT_MAX;
    string rw = "Init";

    char writebuffer[1024];

    // HDD的容量配比，通过使用bitmap计算footprint即可。
    while (fgets(writebuffer, 1024, inputFile) != NULL) {
        long long real = 0;
        string str = string(writebuffer);
        vector<string> ret = split(str, " ");
        // 时间戳
        rw = ret[0];
        offset = stol(ret[2]);
        length = stol(ret[3]);

        maxAddress = max(maxAddress, offset);
        minAddress = min(minAddress, offset);

        if (rw == "W") {
            real = 0;
            long long blkno = offset;
            long long len = length;
            for (long long i = blkno; i < blkno + len; i++) {
                if (bitmap.Test(i) == 0) {
                    real++;
                }
                bitmap.Set(i);
            }
            footprint += real;
        } else {
            real = 0;
            long long blkno = offset;
            long long len = length;
            for (long long i = blkno; i < blkno + len; i++) {
                if (bitmapread.Test(i) == 0) {
                    real++;
                }
                bitmapread.Set(i);
            }
        }
        volume += length;
    }
    // 下面两个容量单位是 B
    long long hddCap = (long long)footprint * 512 / 0.85;
    long long ssdCap = (long long)(left + isExist) * 2 * 1024 * 1024;
    // printf("ssd: %d\n\n", left);

    /*
     * 不能满足QoS的场景，特殊判断
     */

    time_t end_time = time(NULL);
    total_time = (end_time - start_time);
    if (average > qosValue) {
        printf("Unable to meet QoS.\n");
    } else {
        printf(
            "Able to meet QoS: %f ms\n"
            "average latency: %f ms\n"
            "SSD Cap: %lld B (%d MB), Cost:%.2f￥\n"
            "HDD Cap: %lld B (%d MB), Cost:%.2f￥\n"
            "Runtime: %lld s\n",
            qosValue,
            average,
            ssdCap,
            (left + isExist) * 2,
            (ssdCap * SSD_COST) / (1024 * 1024),
            hddCap,
            hddCap / (1024 * 1024),
            (hddCap * HDD_COST) / (1024 * 1024),
            total_time);
    }

    return 0;
}

/*
  字符串转化为float类型
*/

// float to_float(string s) {
//     int i = 0, n = 0;
//     int point_index = s.find('.');
//     float result = 0, under_0 = 0;  // under_0存储小数部分
//     if (s.find('.') != -1)          // 字符串里有小数点
//     {
//         if ((point_index == 0) || (point_index == s.size() - 1))  // 小数点位置合理，不能在字符串第1位,且不能在最后一位
//         {
//             return 0;
//         }
//         for (i = 0; i <= point_index - 1; i++)  // 计算整数部分
//         {
//             if (s[i] >= '0' && s[i] <= '9') {
//                 result = result * 10 + s[i] - 48;
//             }
//         }
//         for (i = s.size() - 1; i >= point_index - 1; i--)  // 计算小数部分
//         {
//             if (s[i] >= '0' && s[i] <= '9') {
//                 if (i == point_index - 1) {
//                     under_0 = under_0 * 0.1 + 0;  // i=小数点前一位，under_0+0
//                 } else {
//                     under_0 = under_0 * 0.1 + s[i] - 48;
//                 }
//             }
//         }
//         result = result + under_0;  // 把整数部分和小数部分相加
//     } else                          // 字符串只有整数部分
//     {
//         for (i = 0; i <= s.size() - 1; i++) {
//             if (s[i] >= '0' && s[i] <= '9') {
//                 result = result * 10 + s[i] - 48;
//             }
//         }
//     }
//     return result;
// }