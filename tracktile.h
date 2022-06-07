#ifndef TRACKTILE_H
#define TRACKTILE_H

#include <iostream>
#include "edge.h"
using namespace std;

const int UP = 0;
const int RIGHT = 1;
const int DOWN = 2;
const int LEFT = 3;

class Edge;

class Tracktile {
public:
	Tracktile();
	char getRepr() const;

	void addTrain(int train, Edge* incomingSource);
		// This function will be called by each Edge when the Edge is dispatching trains.
	void interactTrains();
	void dispatchTrains();

	void setBorder(Edge* border[]);
private:
	Edge* activeConnection[2];
	Edge* passiveConnection[2];
	Edge* border[4];
	
	int trains[4];
	Edge* trainDestinations[4];
	int nTrains;

};

#endif