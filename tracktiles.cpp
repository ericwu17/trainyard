#include "tracktiles.h"
#include "train.h"
#include "display.h"
#include "olcPixelGameEngine.h"
#include "sprites.h"
#include <iostream>
#include <cassert>
using namespace std;

bool Tile::isATrackTile() const {
	return _isATrackTile;
}

void Tile::setBorder(Edge* border[]) {
	for (int i = 0; i < 4; i ++)
		this->border[i] = border[i];
};

void Tile::render(Display* display, int r, int c, SpriteList* spriteList) const {
	display->DrawSprite(olc::vi2d(r, c) * 96, &(spriteList->TRACKTILE_BLANK));
}


Tracktile::Tracktile() {
	_isATrackTile = true;
	for (int i = 0; i < 2; i ++) {
		activeConnection[i] = nullptr;
		passiveConnection[i] = nullptr;
	}
	for (int i = 0; i < 4; i ++) {
		border[i] = nullptr;
	}
	nTrains = 0;
};

char Tracktile::getRepr() const {
	if (nTrains > 0) {
		return 'T';  // T stands for train
	}
	return classifyConnectionType();
	
}

bool Tracktile::hasActiveConnection(int d1, int d2) const {
	return (activeConnection[0] == border[d1] && activeConnection[1] == border[d2]) 
	|| (activeConnection[1] == border[d1] && activeConnection[0] == border[d2]);
}

bool Tracktile::hasPassiveConnection(int d1, int d2) const {
	return (passiveConnection[0] == border[d1] && passiveConnection[1] == border[d2]) 
	|| (passiveConnection[1] == border[d1] && passiveConnection[0] == border[d2]);
}

bool Tracktile::hasConnection(int d1, int d2) const {
	return hasActiveConnection(d1, d2) || hasPassiveConnection(d1, d2);
}

bool Tracktile::hasConnections(int d1, int d2, int e1, int e2) const {
	return hasConnection(d1, d2) && hasConnection(e1, e2);
}

bool Tracktile::hasConnectionUpToRotation(int d1, int d2) const {
	for (int i = 0; i < 4; i ++) {
		if (hasConnection((d1+i) % 4, (d2+i) % 4)) {
			return true;
		}
	}
	return false;
}
bool Tracktile::hasConnectionsUpToRotation(int d1, int d2, int e1, int e2) const {
	for (int i = 0; i < 4; i ++) {
		if (hasConnection((d1+i) % 4, (d2+i) % 4) && hasConnection((e1+i) % 4, (e2+i) % 4)) {
			return true;
		}
	}
	return false;
}

char Tracktile::classifyConnectionType() const {
	if (activeConnection[0] == nullptr) {
		return '_';
	}
	if (passiveConnection[0] == nullptr) {
		if (hasConnectionUpToRotation(0, 2)) {
			return 's';
		}
		assert(hasConnectionUpToRotation(0, 1));
		return 'b';
	}
	// now we can assume that there is both an active and passive connection
	if (hasConnectionsUpToRotation(0, 2, 1, 3)) {
		return 'h';
	} else if (hasConnectionsUpToRotation(0, 1, 2, 3)) {
		return 'z';
	} else if (hasConnectionsUpToRotation(0, 1, 0, 3)) {
		return 'm';
	}
	assert(hasConnectionsUpToRotation(0, 1, 0, 2) || hasConnectionsUpToRotation(0, 3, 0, 2));
	return 'j';
}

void Tracktile::addConnection(int d1, int d2) {
	assert(!(d1 < 0 || d1 > 3 || d2 < 0 || d2 > 3));
	assert(d1 != d2);

	if (hasActiveConnection(d1, d2)) {
		// if the user draws a connection that is already the active connection, then we erase the passive connection
		activeConnection[0] = nullptr;
		activeConnection[1] = nullptr;
	}

	passiveConnection[0] = activeConnection[0];
	passiveConnection[1] = activeConnection[1];

	activeConnection[0] = border[d1];
	activeConnection[1] = border[d2];
}

void Tracktile::pullTrainsFromNeighbors() {
	if (activeConnection[0] != nullptr) {
		int t1 = activeConnection[0]->giveTrain(this);
		if (t1 != -1) {
			trains[nTrains] = t1;
			trainSources[nTrains] = activeConnection[0];
			trainDestinations[nTrains] = activeConnection[1];
			nTrains ++;
		}

		t1 = activeConnection[1]->giveTrain(this);
		if (t1 != -1) {
			trains[nTrains] = t1;
			trainSources[nTrains] = activeConnection[1];
			trainDestinations[nTrains] = activeConnection[0];
			nTrains ++;
		}
	}
	if (passiveConnection[0] != nullptr) {
		int t1 = passiveConnection[0]->giveTrain(this);
		if (t1 != -1) {
			trains[nTrains] = t1;
			trainSources[nTrains] = passiveConnection[0];
			trainDestinations[nTrains] = passiveConnection[1];
			nTrains ++;
		}

		t1 = passiveConnection[1]->giveTrain(this);
		if (t1 != -1) {
			trains[nTrains] = t1;
			trainSources[nTrains] = passiveConnection[1];
			trainDestinations[nTrains] = passiveConnection[0];
			nTrains ++;
		}
	}
	
}

void Tracktile::dispatchTrains() {
	for (int i = 0; i < nTrains; i ++) {
		trainDestinations[i]->receiveTrain(this, trains[i]);
	}
	nTrains = 0;
}

bool Tracktile::indicesOfTrainCollidingAlong(Edge* e1, Edge* e2, int & index1, int &index2) const {
	// two trains are colliding alone e1, e2 if one has source: e1, destination: e2 
	// and the other has source: e2, destination: e1
	// if there are no trains colliding along (e1, e2), then we leave index1 and index2 unchanged.
	// return true if there are trains colliding, false otherwise.
	for (int i1 = 0; i1 < nTrains; i1 ++) {
		for (int i2 = 0; i2 < nTrains; i2 ++) {
			if (trainSources[i1] == e1 && trainDestinations[i1] == e2 && trainSources[i2] == e2 && trainDestinations[i2] == e1) {
				index1 = i1;
				index2 = i2;
				return true;
			}
		}
	}
	return false;
}

void Tracktile::interactTrains() {
	char connType = classifyConnectionType();
	assert (connType != ' ');
	if (nTrains < 2) {
		if (nTrains == 1 && (connType == 'j' || connType == 'm')) {
			switchActiveAndPassive();
		}
		
		return;
	}
	if (connType == 'h' || connType == 's' || connType == 'b') {
		// simply mix all the trains in these connection types
		int newTrainColor = mixTrainColors(trains, nTrains);
		for (int i = 0; i < nTrains; i ++) {
			trains[i]  = newTrainColor;
		}
		return;
	}
	int i1, i2;
	if (connType == 'z') {
		// first do mixing on Active Connection

		if (indicesOfTrainCollidingAlong(activeConnection[0], activeConnection[1], i1, i2)) {
			trains[i1] = trains[i2] = mixTrainColors(trains[i1], trains[i2]);
		}
		// then do mixing on Active Connection
		if (indicesOfTrainCollidingAlong(passiveConnection[0], passiveConnection[1], i1, i2)) {
			trains[i1] = trains[i2] = mixTrainColors(trains[i1], trains[i2]);
		}
		return;
	}
	assert(connType == 'j' || connType == 'm');
	// For j and m type connections, we need to worry about switching between active and passive track.
	bool willSwitchTrack = nTrains % 2 == 1;

	if (indicesOfTrainCollidingAlong(activeConnection[0], activeConnection[1], i1, i2)) {
		trains[i1] = trains[i2] = mixTrainColors(trains[i1], trains[i2]);
	}
	// In this case we don't check for trains colliding along the passive connection, because we know it cannot exist on j and m type connections.
	
	// see if two trains have the same destination
	// if they do, merge them
	for (int i1 = 0; i1 < nTrains; i1 ++) {
		for (int i2 = 0; i2 < nTrains; i2 ++) {
			if(i1 == i2) {
				continue;
			}
			if (trainDestinations[i1] == trainDestinations[i2]) {
				trains[i1] = mixTrainColors(trains[i1], trains[i2]);
				
				// remove the i2 index of train arrays
				int temp = trains[i2];
				trains[i2] = trains[nTrains - 1];
				trains[nTrains - 1] = temp;
				Edge* tempEdge = trainSources[i2];
				trainSources[i2] = trainSources[nTrains-1];
				trainSources[nTrains-1] = tempEdge;
				tempEdge = trainDestinations[i2];
				trainDestinations[i2] = trainDestinations[nTrains-1];
				trainDestinations[nTrains-1] = tempEdge;
				nTrains --;
			}
			

		}
	}


	// switch track connections;
	if (willSwitchTrack) {
		switchActiveAndPassive();
	}

}

void Tracktile::switchActiveAndPassive() {
	assert (activeConnection[0] != nullptr && passiveConnection[0] != nullptr);

	for (int i = 0; i < 2; i ++) {
		Edge* temp = activeConnection[i];
		activeConnection[i] = passiveConnection[i];
		passiveConnection[i] = temp;
	}
}





TrainSource::TrainSource(int dir) {
	assert(0 <= dir && dir <= 3);
	targetEdge = border[dir];
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

void TrainSource::setBorder(Edge* border[]) {
	targetEdge = border[dir];
	for (int i = 0; i < 4; i ++)
		this->border[i] = border[i];
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

TrainSink::TrainSink(int dir) {
	assert(0 <= dir && dir <= 3);
	sourceEdge = border[dir];
	this->dir = dir;
	nTrains = 0;
}

void TrainSink::setBorder(Edge* border[]) {
	sourceEdge = border[dir];
	for (int i = 0; i < 4; i ++)
		this->border[i] = border[i];
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
