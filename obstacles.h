#ifndef OBSTACLES_H
#define OBSTACLES_H

#include <iostream>
#include "tracktile.h"

using namespace std;

const int MAX_NUM_TRAINS_IN_STATION = 9;

class Boulder : public Tile{
public:
	Boulder() {};
	char getRepr() {
		return 'X';
	};
};

class TrainSource : public Tile{
public:
	TrainSource() {};
	TrainSource(int dir);
	void setTrains(int trains[], int nTrains);
	void setBorder(Edge* border[]);
	void dispatchTrains();
	char getRepr() const;
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