#include <iostream>
#include "tracktile.h"
#include "edge.h"
#include "arena.h"

using namespace std;


int main() {
	Arena a;
	while(true) {
		a.display();
		cin.ignore();
		a.processTick();
	}
}