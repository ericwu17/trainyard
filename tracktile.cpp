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

char Tracktile::getRepr() {
	if (nTrains > 0) {
		return 'T';  // T stands for train
	}
	if (activeConnection[0] != nullptr) {
		return 'C';  // C stands for connection
	}
	return '_';
	
}