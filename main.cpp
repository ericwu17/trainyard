#include <iostream>

using namespace std;

class Tracktile;
class Edge;

const int UP = 0;
const int RIGHT = 1;
const int DOWN = 2;
const int LEFT = 3;

const int NUM_ROWS = 3;
const int NUM_COLS = 3;

class Tracktile {
public:
	Tracktile();

	void addTrain(int train, Edge* incomingSource);
		// This function will be called by each Edge when the Edge is dispatching trains.
	void interactTrains();
	void dispatchTrains();

	void setBorder(Edge* border[]) {
		for (int i = 0; i < 4; i ++)
			this->border[i] = border[i];
	}
private:
	Edge* activeConnection[2];
	Edge* passiveConnection[2];
	Edge* border[4];
	
	int trains[4];
	Edge* trainDestinations[4];
	int nTrains;

};

Tracktile::Tracktile() {
	for (int i = 0; i < 2; i ++) {
		activeConnection[i] = nullptr;
		passiveConnection[i] = nullptr;
	}
	for (int i = 0; i < 4; i ++) {
		border[i] = nullptr;
	}
	nTrains = 0;
}



class Edge {
public:
	Edge() : neighborA(nullptr), neighborB(nullptr), trainGoingToA(-1), trainGoingToB(-1) {};
	void receiveTrain(Tracktile* source, int train);
	// This function will be called by each Tracktile when the Tracktile is dispatching trains.
	void interactTrains();
	void dispatchTrains();

	void setNeighbors(Tracktile* a, Tracktile* b) {
		neighborA = a;
		neighborB = b;
	};
private :
	Tracktile* neighborA;
	Tracktile* neighborB;
	int trainGoingToA;
	int trainGoingToB;
};

class Arena {
public:
	Arena();
private:
	Tracktile tracktiles[NUM_ROWS][NUM_COLS];
	Edge horizontalEdges [NUM_ROWS+1][NUM_COLS];
	Edge verticalEdges [NUM_ROWS][NUM_COLS+1];
};

Arena::Arena() {
	cout << "Setting neighbors for horizontal edges" << endl;
	for (int c = 0; c < NUM_COLS; c ++) {
		horizontalEdges[0][c].setNeighbors(nullptr, &tracktiles[0][c]);
		horizontalEdges[NUM_ROWS][c].setNeighbors(&tracktiles[NUM_ROWS-1][c], nullptr);
	}
	for (int r = 0; r < NUM_ROWS - 1; r ++) {
		for (int c = 0; c < NUM_COLS; c ++) {
			horizontalEdges[r+1][c].setNeighbors(&tracktiles[r][c], &tracktiles[r+1][c]);
		}
	}


	cout << "Setting neighbors for vertical edges" << endl;
	for (int r = 0; r < NUM_ROWS; r ++) {
		verticalEdges[r][0].setNeighbors(nullptr, &tracktiles[r][0]);
		verticalEdges[r][NUM_COLS].setNeighbors(&tracktiles[r][NUM_COLS-1], nullptr);
	}
	for(int c = 0; c < NUM_COLS - 1; c ++) {
		for (int r = 0; r < NUM_ROWS; r ++) {
			verticalEdges[r][c+1].setNeighbors(&tracktiles[r][c], &tracktiles[r][c+1]);
		}
	}
	

	cout << "Setting neighbors for individual tracktiles" << endl;
	for (int r = 0; r < NUM_ROWS; r ++) {
		for (int c = 0; c < NUM_COLS; c ++) {
			Edge* borderArr[4] = {&horizontalEdges[r][c], &verticalEdges[r][c+1], &horizontalEdges[r+1][c], &verticalEdges[r][c]};
			tracktiles[r][c].setBorder(borderArr);
		}
	}
}



int main() {
	Arena a;
	cout << "Hello World" << endl;
}