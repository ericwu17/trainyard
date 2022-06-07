#include "tracktile.h"
#include "train.h"
#include <iostream>
using namespace std;


Tracktile::Tracktile() {
	for (int i = 0; i < 2; i ++) {
		activeConnection[i] = nullptr;
		passiveConnection[i] = nullptr;
	}
	for (int i = 0; i < 4; i ++) {
		border[i] = nullptr;
	}
	nTrains = 0;
};

void Tracktile::setBorder(Edge* border[]) {
	for (int i = 0; i < 4; i ++)
		this->border[i] = border[i];
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
	assert(hasConnectionsUpToRotation(0, 1, 0, 2));
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

void Tracktile::interactTrains() {
	if (nTrains < 2) {
		return;
	}

}