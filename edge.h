#ifndef EDGE_H
#define EDGE_H

#include <iostream>
#include "tracktile.h"
using namespace std;

class Tracktile;

class Edge {
public:
	Edge() : neighborA(nullptr), neighborB(nullptr), trainGoingToA(-1), trainGoingToB(-1) {};
	void receiveTrain(Tracktile* source, int train);
	// This function will be called by each Tracktile when the Tracktile is dispatching trains.
	void interactTrains();
	void dispatchTrains();

	void setNeighbors(Tracktile* a, Tracktile* b);

	char display(); 
private :
	Tracktile* neighborA;
	Tracktile* neighborB;
	int trainGoingToA;
	int trainGoingToB;
};

#endif