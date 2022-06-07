#include <iostream>
#include "edge.h"
using namespace std;

char Edge::getRepr() {
	if (trainGoingToA == -1 && trainGoingToB == -1) {
		return '*';
	} else {
		return 'T';
	}
};
void Edge::setNeighbors(Tracktile* a, Tracktile* b) {
	neighborA = a;
	neighborB = b;
};