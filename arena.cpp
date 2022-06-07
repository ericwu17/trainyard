#include "arena.h"
#include "edge.h"
#include "tracktile.h"


Arena::Arena() {
	// cout << "Setting neighbors for horizontal edges" << endl;
	for (int c = 0; c < NUM_COLS; c ++) {
		horizontalEdges[0][c].setNeighbors(nullptr, &tracktiles[0][c]);
		horizontalEdges[NUM_ROWS][c].setNeighbors(&tracktiles[NUM_ROWS-1][c], nullptr);
	}
	for (int r = 0; r < NUM_ROWS - 1; r ++) {
		for (int c = 0; c < NUM_COLS; c ++) {
			horizontalEdges[r+1][c].setNeighbors(&tracktiles[r][c], &tracktiles[r+1][c]);
		}
	}


	// cout << "Setting neighbors for vertical edges" << endl;
	for (int r = 0; r < NUM_ROWS; r ++) {
		verticalEdges[r][0].setNeighbors(nullptr, &tracktiles[r][0]);
		verticalEdges[r][NUM_COLS].setNeighbors(&tracktiles[r][NUM_COLS-1], nullptr);
	}
	for(int c = 0; c < NUM_COLS - 1; c ++) {
		for (int r = 0; r < NUM_ROWS; r ++) {
			verticalEdges[r][c+1].setNeighbors(&tracktiles[r][c], &tracktiles[r][c+1]);
		}
	}
	

	// cout << "Setting neighbors for individual tracktiles" << endl;
	for (int r = 0; r < NUM_ROWS; r ++) {
		for (int c = 0; c < NUM_COLS; c ++) {
			Edge* borderArr[4] = {&horizontalEdges[r][c], &verticalEdges[r][c+1], &horizontalEdges[r+1][c], &verticalEdges[r][c]};
			tracktiles[r][c].setBorder(borderArr);
		}
	}
}

void Arena::display() {
	for (int r = 0; r < NUM_ROWS; r ++) {
		cout << ' ';
		for (int c = 0; c < NUM_ROWS; c ++) {
			cout << horizontalEdges[r][c].getRepr() << ' ';
		}
		cout << endl;
		for (int c = 0; c < NUM_ROWS; c ++) {
			cout << verticalEdges[r][c].getRepr() << tracktiles[r][c].getRepr();
		}
		cout << verticalEdges[r][NUM_ROWS].getRepr() << endl;
	}
	cout << ' ';
	for (int c = 0; c < NUM_ROWS; c ++) {
		cout << horizontalEdges[NUM_ROWS][c].getRepr() << ' ';
	}
	cout << endl;
}