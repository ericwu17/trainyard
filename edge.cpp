#include <iostream>
#include <cassert>
#include "edge.h"
#include "train.h"
#include "sprites.h"
using namespace std;

const double TWO_PI = 6.2831852;


char Edge::getRepr() const {
	if (trainGoingToA == -1 && trainGoingToB == -1) {
		return '*';
	} else {
		if (trainGoingToA != -1 && trainGoingToB != -1) {
			return 'T';
		}
		if (trainGoingToA != -1) {
			return '0' + trainGoingToA;
		} 
		return '0' + trainGoingToB;
	}
};

void Edge::render(Display* display, int r, int c, bool isVertical, SpriteList* spriteList) const {
	float train_height = float(spriteList->SPRITE_TRAIN->height);
	float train_width = float(spriteList->SPRITE_TRAIN->width);
	float gridSize = float(spriteList->SPRITE_TRACKTILE_BLANK->width);
	float xPos, yPos;
	if (isVertical) {
		xPos = gridSize * c;
		yPos = gridSize * r + gridSize * 0.5;
		if (trainGoingToA != -1) {
			display->DrawRotatedDecal(olc::vi2d(xPos, yPos), spriteList->TRAIN, 0.75*TWO_PI, {train_width/2, train_height/2}, {1,1}, resolveTrainColor(trainGoingToA));
		}
		if (trainGoingToB != -1) {
			display->DrawRotatedDecal(olc::vi2d(xPos, yPos), spriteList->TRAIN, 0.25*TWO_PI, {train_width/2, train_height/2}, {1,1}, resolveTrainColor(trainGoingToB));
		}
	} else {
		xPos = gridSize * c + gridSize * 0.5;
		yPos = gridSize * r;
		if (trainGoingToA != -1) {
			display->DrawRotatedDecal(olc::vi2d(xPos, yPos), spriteList->TRAIN,0*TWO_PI, {train_width/2, train_height/2}, {1,1}, resolveTrainColor(trainGoingToA));
		}
		if (trainGoingToB != -1) {
			display->DrawRotatedDecal(olc::vi2d(xPos, yPos), spriteList->TRAIN, 0.5*TWO_PI, {train_width/2, train_height/2}, {1,1}, resolveTrainColor(trainGoingToB));
		}
	}
};

void Edge::setNeighbors(Tile* a, Tile* b) {
	neighborA = a;
	neighborB = b;
};

void Edge::receiveTrain(Tile* source, int train) {
	assert(source == neighborA || source == neighborB);
	assert(isValidTrain(train));
	if (source == neighborA) {
		assert(trainGoingToB == -1);
		trainGoingToB = train;
	} else {
		assert(trainGoingToA == -1);
		trainGoingToA = train;
	}
}

void Edge::interactTrains() {
	if (trainGoingToA == -1 || trainGoingToB == -1) {
		return;
	}
	trainGoingToB = trainGoingToA = mixTrainColors(trainGoingToA, trainGoingToB);
}

int Edge::giveTrain(Tile *recipient) {
	assert(recipient == neighborA || recipient == neighborB);
	if (recipient == neighborA) {
		int toRet = trainGoingToA;
		trainGoingToA = -1;
		return toRet;
	} else {
		int toRet = trainGoingToB;
		trainGoingToB = -1;
		return toRet;
	}
}

int Edge::softGiveTrain(Tile *recipient) const {
	// same as giveTrain, but only returns the train without actually removing the train.
	// this function is used when a TrainSink needs to test whether it can pull in a train from the edge.
	assert(recipient == neighborA || recipient == neighborB);
	if (recipient == neighborA) {
		return trainGoingToA;
	} else {
		return trainGoingToB;
	}
}

bool Edge::crashIfTrainsInEdge() {
	if (trainGoingToA == -1 && trainGoingToB == -1) {
		return false;
	}
	cout << "A train has crashed" << endl;
	return true;
}
