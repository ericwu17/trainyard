#ifndef TRACKTILES_H
#define TRACKTILES_H

#include <iostream>
#include "edge.h"
#include "display.h"
#include "sprites.h"
using namespace std;

const int UP = 0;
const int RIGHT = 1;
const int DOWN = 2;
const int LEFT = 3;

const int MAX_NUM_TRAINS_IN_STATION = 9;

class Edge;
class Display;

class Tile {
public:
	Tile() {};
	virtual char getRepr() const {
		return '&';
	};
	virtual void render(Display* display, int r, int c, SpriteList* spriteList) const;
	virtual void pullTrainsFromNeighbors() {};
	virtual void interactTrains() {};
	virtual void dispatchTrains() {};


	virtual void setBorder(Edge* border[]);
	virtual void addConnection(int d1, int d2) {};
	virtual void setTrains(int trains[], int nTrains) {};
	virtual void setDesires(int trains[], int nTrains) {};
	
	bool isATrackTile() const;
protected:
	bool _isATrackTile;
	Edge* border[4];
};


class Tracktile : public Tile{
public:
	Tracktile();
	char getRepr() const;

	void pullTrainsFromNeighbors();
	void interactTrains();
	void dispatchTrains();

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
	
	int trains[4];
	Edge* trainDestinations[4];
	Edge* trainSources[4];
	int nTrains;
};

class TrainSource : public Tile{
public:
	TrainSource() {};
	TrainSource(int dir);
	void setTrains(int trains[], int nTrains);
	void setBorder(Edge* border[]);
	void dispatchTrains();
	char getRepr() const;
	void render(Display* display, int r, int c, SpriteList* spriteList) const;
private:
	Edge* targetEdge;
	int dir;
	int nTrains;
	int trains[MAX_NUM_TRAINS_IN_STATION];
};

class TrainSink : public Tile {
public:
	TrainSink() {};
	TrainSink(int dir);
	void setBorder(Edge* border[]);
	void setDesires(int trains[], int nTrains);
	void pullTrainsFromNeighbors();
	bool isSatisfied();
	char getRepr() const;
	int getX() const;
	int getY() const;
private:
	Edge* sourceEdge;
	int dir;
	int nTrains;
	int desiredTrains[MAX_NUM_TRAINS_IN_STATION];
};



#endif