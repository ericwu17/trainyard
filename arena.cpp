#include "arena.h"
#include "edge.h"
#include "tracktiles.h"
#include "display.h"
#include "olcPixelGameEngine.h"
#include "sprites.h"
#include <cassert>

using namespace std;

Arena::Arena() {
	for (int c = 0; c < NUM_COLS; c ++)
		for (int r = 0; r < NUM_ROWS; r ++)
			tiles[r][c] = new Tracktile();
	
	for (int r = 0; r < NUM_ROWS+1; r ++)
		for (int c = 0; c < NUM_COLS; c ++)
			horizontalEdges[r][c] = new Edge();
	
	for (int r = 0; r < NUM_ROWS; r ++)
		for (int c = 0; c < NUM_COLS+1; c ++)
			verticalEdges[r][c] = new Edge();
	


	delete tiles[0][3];
	tiles[0][3] = new TrainSource(3);
	delete tiles[5][2];
	bool canRTrainArr[] = {true, true, false, true};
	tiles[5][2] = new TrainSink(canRTrainArr);


	// cout << "Setting neighbors for horizontal edges" << endl;
	for (int c = 0; c < NUM_COLS; c ++) {
		horizontalEdges[0][c]->setNeighbors(nullptr, tiles[0][c]);
		horizontalEdges[NUM_ROWS][c]->setNeighbors(tiles[NUM_ROWS-1][c], nullptr);
	}
	for (int r = 0; r < NUM_ROWS - 1; r ++) {
		for (int c = 0; c < NUM_COLS; c ++) {
			horizontalEdges[r+1][c]->setNeighbors(tiles[r][c], tiles[r+1][c]);
		}
	}


	// cout << "Setting neighbors for vertical edges" << endl;
	for (int r = 0; r < NUM_ROWS; r ++) {
		verticalEdges[r][0]->setNeighbors(nullptr, tiles[r][0]);
		verticalEdges[r][NUM_COLS]->setNeighbors(tiles[r][NUM_COLS-1], nullptr);
	}
	for(int c = 0; c < NUM_COLS - 1; c ++) {
		for (int r = 0; r < NUM_ROWS; r ++) {
			verticalEdges[r][c+1]->setNeighbors(tiles[r][c], tiles[r][c+1]);
		}
	}
	

	// cout << "Setting neighbors for individual tracktiles" << endl;
	for (int r = 0; r < NUM_ROWS; r ++) {
		for (int c = 0; c < NUM_COLS; c ++) {
			Edge* borderArr[4] = {horizontalEdges[r][c], verticalEdges[r][c+1], horizontalEdges[r+1][c], verticalEdges[r][c]};
			tiles[r][c]->setBorder(borderArr);
		}
	}
	// the below is a situation where two trains need to merge into one
	// tracktiles[0][0].addConnection(1,2);
	// tracktiles[0][1].addConnection(2,3);
	// tracktiles[0][1].addConnection(2,1);
	// tracktiles[0][2].addConnection(2,3);
	// tracktiles[1][2].addConnection(2,0);
	// tracktiles[1][1].addConnection(2,0);
	// horizontalEdges[1][0].receiveTrain(&tracktiles[1][0], 1);
	// horizontalEdges[1][2].receiveTrain(&tracktiles[1][2], 2);



	// the below is a situation where a train is in an infinite loop
	// tracktiles[0][0].addConnection(1,2);
	// tracktiles[0][1].addConnection(3,2);
	// tracktiles[1][0].addConnection(1,0);
	// tracktiles[1][1].addConnection(3,0);
	// horizontalEdges[1][0].receiveTrain(&tracktiles[0][0], 1);

	// the below is a situation where two trains need to merge into one while also mixing with a third
	// tracktiles[0][0].addConnection(1,2);
	// tracktiles[0][1].addConnection(2,3);
	// tracktiles[0][1].addConnection(2,1);
	// tracktiles[0][2].addConnection(2,3);
	// tracktiles[1][2].addConnection(2,0);
	// tracktiles[1][1].addConnection(2,0);
	// horizontalEdges[1][0].receiveTrain(&tracktiles[1][0], 1);
	// horizontalEdges[1][2].receiveTrain(&tracktiles[1][2], 2);
	// horizontalEdges[2][1].receiveTrain(&tracktiles[2][1], 3);

	// the below tests active/passive track switching
	// tiles[0][0]->addConnection(1,2);
	// tiles[0][1]->addConnection(2,3);
	// tiles[0][1]->addConnection(2,1);
	// tiles[0][2]->addConnection(2,3);
	// tiles[1][2]->addConnection(2,0);
	// tiles[1][1]->addConnection(2,0);
	// horizontalEdges[2][1].receiveTrain(tiles[2][1], 3);
	// horizontalEdges[1][1].receiveTrain(tiles[1][1], 2);

	// tiles[0][0]->addConnection(3,2);
	// tiles[0][0]->addConnection(0,1);
	// tiles[0][1]->addConnection(0,1);
	// tiles[1][0]->addConnection(0,3);
	// tiles[1][0]->addConnection(1,2);

	// tiles [6][6]->addConnection(0,2);
	// tiles [6][6]->addConnection(1,3);
	// tiles [6][5]->addConnection(1,3);
	// tiles [6][5]->addConnection(0,2);
	// tiles [5][6]->addConnection(2,3);
	// tiles [5][6]->addConnection(1,2);
	// tiles [4][6]->addConnection(1,2);
	// tiles [4][6]->addConnection(2,3);
	// tiles [3][6]->addConnection(3,0);
	// tiles [3][6]->addConnection(1,0);

	// tiles [0][6]->addConnection(3,0);
	// tiles [0][6]->addConnection(1,3);
	// tiles [1][6]->addConnection(1,3);
	// tiles [1][6]->addConnection(2,3);

	// tiles[0][2]->addConnection(2,1);
	// tiles[1][2]->addConnection(2,0);
	// tiles[2][2]->addConnection(2,0);
	// tiles[3][2]->addConnection(2,0);
	// tiles[4][2]->addConnection(2,0);
	int trainArr[] = {1, 2, 3};
	tiles[0][3]->setTrains(trainArr, 3);
	int trainArr2[] = {2, 3, 4};
	tiles[5][2]->setDesires(trainArr2, 3);

}

Arena::~Arena() {
	for (int r = 0; r < NUM_ROWS; r ++)
		for (int c = 0; c < NUM_COLS; c ++)
			delete tiles[r][c];
	
	for (int r = 0; r < NUM_ROWS+1; r ++)
		for (int c = 0; c < NUM_COLS; c ++)
			delete horizontalEdges[r][c];
	
	for (int r = 0; r < NUM_ROWS; r ++)
		for (int c = 0; c < NUM_COLS+1; c ++)
			delete verticalEdges[r][c];
}

void Arena::display() const {
	for (int r = 0; r < NUM_ROWS; r ++) {
		cout << ' ';
		for (int c = 0; c < NUM_ROWS; c ++) {
			cout << horizontalEdges[r][c]->getRepr() << ' ';
		}
		cout << endl;
		for (int c = 0; c < NUM_ROWS; c ++) {
			cout << verticalEdges[r][c]->getRepr() << tiles[r][c]->getRepr();
		}
		cout << verticalEdges[r][NUM_ROWS]->getRepr() << endl;
	}
	cout << ' ';
	for (int c = 0; c < NUM_ROWS; c ++) {
		cout << horizontalEdges[NUM_ROWS][c]->getRepr() << ' ';
	}
	cout << endl;
}

void Arena::render(Display* display, SpriteList* spriteList) const {
	for (int r = 0; r < NUM_ROWS; r ++) {
		for (int c = 0; c < NUM_COLS; c ++) {
			tiles[r][c]->render(display, r, c, spriteList);
		}
	}

	for (int r = 0; r < NUM_ROWS+1; r ++)
		for (int c = 0; c < NUM_COLS; c ++)
			horizontalEdges[r][c]->render(display, r, c, false, spriteList);
	
	for (int r = 0; r < NUM_ROWS; r ++)
		for (int c = 0; c < NUM_COLS+1; c ++)
			verticalEdges[r][c]->render(display, r, c, true, spriteList);
}

void Arena::addConnection(int row, int col, int dir1, int dir2) {
	if (row < 0 || row >= NUM_ROWS) {
		cout << "row out of range" << endl;
		exit(1);
	}
	if (col < 0 || col >= NUM_COLS) {
		cout << "col out of range" << endl;
		exit(1);
	}
	tiles[row][col]->addConnection(dir1, dir2);
}

void Arena::processTick() {

	for(int c = 0; c < NUM_COLS+1; c ++) {
		for (int r = 0; r < NUM_ROWS; r ++) {
			verticalEdges[r][c]->interactTrains();
		}
	}
	for(int r = 0; r < NUM_ROWS+1; r ++) {
		for (int c = 0; c < NUM_COLS; c ++) {
			horizontalEdges[r][c]->interactTrains();
		}
	}
	
	for (int r = 0; r < NUM_ROWS; r ++) {
		for (int c = 0; c < NUM_COLS; c ++) {
			tiles[r][c]->pullTrainsFromNeighbors();
		}
	}

	for(int c = 0; c < NUM_COLS+1; c ++) {
		for (int r = 0; r < NUM_ROWS; r ++) {
			if(verticalEdges[r][c]->crashIfTrainsInEdge()) {
				assert(false);
			}
		}
	}
	for(int r = 0; r < NUM_ROWS+1; r ++) {
		for (int c = 0; c < NUM_COLS; c ++) {
			if(horizontalEdges[r][c]->crashIfTrainsInEdge()) {
				assert(false);
			}
		}
	}


	for (int r = 0; r < NUM_ROWS; r ++) {
		for (int c = 0; c < NUM_COLS; c ++) {
			tiles[r][c]->interactTrains();
		}
	}
	for (int r = 0; r < NUM_ROWS; r ++) {
		for (int c = 0; c < NUM_COLS; c ++) {
			tiles[r][c]->dispatchTrains();
		}
	}
}