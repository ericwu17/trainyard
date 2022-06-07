#include <iostream>
#include "edge.h"
#include "train.h"
using namespace std;


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

void Edge::interactTrains() {
	if (trainGoingToA == -1 || trainGoingToB == -1) {
		return;
	}
	trainGoingToA = mixTrainColors(trainGoingToA, trainGoingToB);
	trainGoingToB = mixTrainColors(trainGoingToA, trainGoingToB);
}

void Edge::dispatchTrains() {
	if (trainGoingToA != -1) {
		neighborA->addTrain(trainGoingToA, this);
		trainGoingToA = -1;
	}
	if (trainGoingToB != -1) {
		neighborB->addTrain(trainGoingToB, this);
		trainGoingToB = -1;
	}
}