#ifndef ARENA_H
#define ARENA_H

#include <iostream>
#include "edge.h"
#include "tracktile.h"
using namespace std;

const int NUM_ROWS = 3;
const int NUM_COLS = 3;

class Arena {
public:
	Arena();
	void display() const;
	void addConnection(int row, int col, int dir1, int dir2);
	void processTick();
private:
	Tracktile tracktiles[NUM_ROWS][NUM_COLS];
	Edge horizontalEdges[NUM_ROWS+1][NUM_COLS];
	Edge verticalEdges[NUM_ROWS][NUM_COLS+1];
};

#endif