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

	void pullTrainsFromNeighbors();
	void interactTrains();
	void dispatchTrains();

	void setBorder(Edge* border[]);

	void addConnection(int d1, int d2);
	void switchActiveAndPassive();
	bool hasActiveConnection(int d1, int d2) const;
	bool hasPassiveConnection(int d1, int d2) const;
	bool hasConnection(int d1, int d2) const;
	bool hasConnections(int d1, int d2, int e1, int e2) const;
	bool hasConnectionUpToRotation(int d1, int d2) const;
	bool hasConnectionsUpToRotation(int d1, int d2, int e1, int e2) const;
	bool indicesOfTrainCollidingAlong(Edge* e1, Edge* e2, int & index1, int &index2) const;
	char classifyConnectionType() const;
private:
	Edge* activeConnection[2];
	Edge* passiveConnection[2];
	Edge* border[4];
	
	int trains[4];
	Edge* trainDestinations[4];
	Edge* trainSources[4];
	int nTrains;

};

#endif