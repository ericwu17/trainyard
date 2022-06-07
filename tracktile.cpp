#include "tracktile.h"
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
	if (passiveConnection[0] != nullptr && activeConnection[0] != nullptr) {
		return 'C';  // C stands for connection, and a captial C implies both are there.
	}
	if (activeConnection[0] != nullptr) {
		return 'c';  // c stands for connection
	}
	return '_';
	
}

bool Tracktile::hasActiveConnection(int d1, int d2) const {
	return (activeConnection[0] == border[d1] && activeConnection[1] == border[d2]) 
	|| (activeConnection[1] == border[d1] && activeConnection[0] == border[d2]);
}

bool Tracktile::hasPassiveConnection(int d1, int d2) const {
	return (passiveConnection[0] == border[d1] && passiveConnection[1] == border[d2]) 
	|| (passiveConnection[1] == border[d1] && passiveConnection[0] == border[d2]);
}

void Tracktile::addConnection(int d1, int d2) {
	if (d1 < 0 || d1 > 3 || d2 < 0 || d2 > 3) {
		cout << "invalid direction!" << endl;
		exit(1);
	}
	if (d1 == d2) {
		cout << "cannot have a self-connection" << endl;
		exit(1);
	}

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