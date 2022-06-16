#include <iostream>
#include "olcPixelGameEngine.h"
#include "display.h"

using namespace std;



int main(int argc, char const *argv[]) {
	Display demo;
	if (demo.Construct(672, 672, 1, 1))
		demo.Start();

	return 0;
}
