#include <iostream>
#include <filesystem>
#include "olcPixelGameEngine.h"
#include "display.h"

using namespace std;

using std::filesystem::current_path;


int main(int argc, char const *argv[]) {
	cout << "Current working directory: " << current_path() << endl;
	Display demo;
	if (demo.Construct(672, 672, 1, 1))
		demo.Start();

	return 0;
}
