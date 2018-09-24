/// Heng Li (author of BWA) custom C hash library and usage
/// Code here: https://attractivechaos.wordpress.com/2008/09/02/implementing-generic-hash-library-in-c/
/// Bench: http://attractivechaos.awardspace.com/udb.html
/// and more bench: https://attractivechaos.wordpress.com/2008/08/28/comparison-of-hash-table-libraries/

// Note, this was pulled from latest github version from "klib"
// http://attractivechaos.github.io/klib/#Khash%3A%20generic%20hash%20table
#include "khash.h"
KHASH_MAP_INIT_INT(int, int)
KHASH_MAP_INIT_STR(str, int)

int test_int(int N, const unsigned *data)
{
	int i, ret;
	mem_start();
	khash_t(int) *h;
	unsigned k;
	h = kh_init(int);
	for (i = 0; i < N; ++i) {
		k = kh_put(int, h, data[i], &ret);
		if (!ret) kh_del(int, h, k);
		else kh_value(h, k) = i;
	}
	ret = (int)kh_size(h);
	mem_stop();
	kh_destroy(int, h);
	return ret;
}

int test_str(int N, char * const *data)
{
	int i, ret;
	mem_start();
	khash_t(str) *h;
	unsigned k;
	h = kh_init(str);
	for (i = 0; i < N; ++i) {
		k = kh_put(str, h, data[i], &ret);
		if (!ret) kh_del(str, h, k);
		else kh_value(h, k) = i;
	}
	ret = (int)kh_size(h);
	mem_stop();
	kh_destroy(str, h);
	return ret;
}
