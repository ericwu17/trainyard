#include <iostream>
#include "edge.h"
using namespace std;

bool isValidTrain(int train){
	// -1 is the null train
	// valid trains are trains for 0 to 6:
	// 0: brown
	// 1: red     2: blue    3: yellow
	// 4: purple  5: green   6: orange
	return 0 <= train && train <= 6;
}

char Edge::getRepr() const {
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

void Edge::receiveTrain(Tracktile* source, int train) {
	assert(source == neighborA || source == neighborB);
	assert(isValidTrain(train));
	if (source == neighborA) {
		trainGoingToB = train;
	} else {
		trainGoingToA = train;
	}
}