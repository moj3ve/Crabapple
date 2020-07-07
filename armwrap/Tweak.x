#include <dlfcn.h>

#ifndef ARMWRAP_TARGET
#error ARMWRAP_TARGET not specified!
#else
#define TARGET "/Library/MobileSubstrate/DynamicLibraries/" ARMWRAP_TARGET ".dylib"

%ctor {
	dlopen(TARGET, RTLD_NOW);
}
#endif
