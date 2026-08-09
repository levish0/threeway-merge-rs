#include "xdiff.h"

enum {
	THREEWAY_ALGORITHM_MYERS = 0,
	THREEWAY_ALGORITHM_MINIMAL = 1,
	THREEWAY_ALGORITHM_PATIENCE = 2,
	THREEWAY_ALGORITHM_HISTOGRAM = 3
};

int threeway_merge_run(const char *base, long base_size,
		const char *ours, long ours_size,
		const char *theirs, long theirs_size,
		int algorithm, int marker_size, int level, int favor, int style,
		const char *base_label, const char *ours_label,
		const char *theirs_label, char **result_ptr, long *result_size)
{
	mmfile_t base_file, ours_file, theirs_file;
	mmbuffer_t result = { NULL, 0 };
	xmparam_t params = { 0 };
	int status;

	if (!result_ptr || !result_size || base_size < 0 || ours_size < 0 ||
	    theirs_size < 0 || (!base && base_size) || (!ours && ours_size) ||
	    (!theirs && theirs_size))
		return -2;

	*result_ptr = NULL;
	*result_size = 0;

	switch (algorithm) {
	case THREEWAY_ALGORITHM_MYERS:
		break;
	case THREEWAY_ALGORITHM_MINIMAL:
		params.xpp.flags = XDF_NEED_MINIMAL;
		break;
	case THREEWAY_ALGORITHM_PATIENCE:
		params.xpp.flags = XDF_PATIENCE_DIFF;
		break;
	case THREEWAY_ALGORITHM_HISTOGRAM:
		params.xpp.flags = XDF_HISTOGRAM_DIFF;
		break;
	default:
		return -2;
	}

	if (level < XDL_MERGE_MINIMAL || level > XDL_MERGE_ZEALOUS_ALNUM ||
	    favor < 0 || favor > XDL_MERGE_FAVOR_UNION ||
	    style < 0 || style > XDL_MERGE_ZEALOUS_DIFF3)
		return -2;

	base_file.ptr = (char *)base;
	base_file.size = base_size;
	ours_file.ptr = (char *)ours;
	ours_file.size = ours_size;
	theirs_file.ptr = (char *)theirs;
	theirs_file.size = theirs_size;

	params.marker_size = marker_size;
	params.level = level;
	params.favor = favor;
	params.style = style;
	params.ancestor = base_label;
	params.file1 = ours_label;
	params.file2 = theirs_label;

	status = xdl_merge(&base_file, &ours_file, &theirs_file,
			   &params, &result);
	*result_ptr = result.ptr;
	*result_size = result.size;
	return status;
}

void threeway_merge_free(void *ptr)
{
	xdl_free(ptr);
}
