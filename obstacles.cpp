#include "obstacles.h"
#include "edge.h"

TrainSource::TrainSource(Edge* targetEdge, int dir) {
	this->targetEdge = targetEdge;
	this->dir = dir;
	nTrains = 0;
}

void TrainSource::setTrains(int trains[], int nTrains) {
	assert(nTrains <= MAX_NUM_TRAINS_IN_STATION);
	for (int i = 0; i < nTrains; i ++) {
		this->trains[i] = trains[i];
	}
	this->nTrains = nTrains;
}

void TrainSource::dispatchTrains() {
	if (nTrains == 0) {
		return;
	}
	targetEdge->receiveTrain(this, trains[nTrains-1]);
	nTrains --;
}

char TrainSource::getRepr() const {
	return nTrains + '0';
}

TrainSink::TrainSink(Edge* sourceEdge, int dir) {
	this->sourceEdge = sourceEdge;
	this->dir = dir;
	nTrains = 0;
}
void TrainSink::setDesires(int trains[], int nTrains) {
	assert(nTrains <= MAX_NUM_TRAINS_IN_STATION);
	for (int i = 0; i < nTrains; i ++) {
		desiredTrains[i] = trains[i];
	}
	this->nTrains = nTrains;
}
void TrainSink::pullTrainsFromNeighbors() {
	// pull in a train only if there is a train which matches one of the desired trains.
	int incomingTrain = sourceEdge->softGiveTrain(this);
	for (int i = 0; i < nTrains; i ++) {
		if (incomingTrain == desiredTrains[i]) {
			sourceEdge->giveTrain(this);
			
			//delete the i-th train from the array of desired trains while keeping the rest in order
			for (int k = i; k+1 < nTrains; k ++) {
				desiredTrains[k] = desiredTrains[k+1];
			}
			nTrains --;
		}
	}
}

bool TrainSink::isSatisfied() {
	return nTrains == 0;
}

char TrainSink::getRepr() const {
	return nTrains + '0';
}
