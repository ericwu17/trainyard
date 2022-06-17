#ifndef EDGE_H
#define EDGE_H

#include <iostream>
#include "tracktiles.h"
#include "display.h"
using namespace std;

class Tile;

class Edge {
public:
	Edge() : neighborA(nullptr), neighborB(nullptr), trainGoingToA(-1), trainGoingToB(-1) {};
	char getRepr() const;
	void render(Display* display, int r, int c, bool isVertical, SpriteList* spriteList) const;

	void receiveTrain(Tile* source, int train);
		// This function will be called by each Tracktile when the Tracktile is dispatching trains.
	void interactTrains();
	int giveTrain(Tile *recipient);
		// This function will be called be each Tracktile when the tracktile is receiving trains.
	int softGiveTrain(Tile *recipient) const;
	bool crashIfTrainsInEdge();

	void setNeighbors(Tile* a, Tile* b);

private :
	Tile* neighborA;
	Tile* neighborB;
	int trainGoingToA;
	int trainGoingToB;
};

#endif