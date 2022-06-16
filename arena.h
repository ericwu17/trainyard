#ifndef ARENA_H
#define ARENA_H

#include <iostream>
#include "edge.h"
#include "tracktiles.h"
#include "display.h"
#include "sprites.h"
using namespace std;

const int NUM_ROWS = 7;
const int NUM_COLS = 7;
const int NUM_TILES = NUM_ROWS * NUM_COLS;

class Display;
class Tile;
class Edge;
class Arena {
public:
	Arena();
	~Arena();
	void display() const;
	void render(Display* display, SpriteList* spriteList) const;
	void addConnection(int row, int col, int dir1, int dir2);
	void processTick();
private:
	Tile* tiles[NUM_ROWS][NUM_COLS];
	Edge* horizontalEdges[NUM_ROWS+1][NUM_COLS];
	Edge* verticalEdges[NUM_ROWS][NUM_COLS+1];
};

#endif