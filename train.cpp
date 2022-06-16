#include "train.h"
#include "olcPixelGameEngine.h"
#include <iostream>
#include <cassert>
using namespace std;




bool isValidTrain(int train){
	return 
		train == BROWN
		|| train == RED
		|| train == BLUE
		|| train == YELLOW
		|| train == PURPLE
		|| train == GREEN
		|| train == ORANGE;
}

int mixTrainColors(int train1, int train2) {
	assert(isValidTrain(train1));
	assert(isValidTrain(train2));
	if (train1 == train2) {
		return train1;
	}
	
	if ((train1 == RED && train2 == BLUE) || (train1 == BLUE && train2 == RED)) {
		return PURPLE;
	}
	if ((train1 == YELLOW && train2 == BLUE) || (train1 == BLUE && train2 == YELLOW)) {
		return GREEN;
	}
	if ((train1 == YELLOW && train2 == RED) || (train1 == RED && train2 == YELLOW)) {
		return ORANGE;
	}

	return BROWN;
}

int mixTrainColors(int trains[], int numTrains) {
	assert(numTrains > 0);
	assert(numTrains < 5);
	if (numTrains == 1) {
		return trains[0];
	}
	if (numTrains == 2) {
		return mixTrainColors(trains[0], trains[1]);
	}
	if (numTrains == 3) {
		return mixTrainColors(mixTrainColors(trains[0], trains[1]), trains[2]);
	}
	
	return mixTrainColors(mixTrainColors(mixTrainColors(trains[0], trains[1]), trains[2]), trains[3]);
	

}

olc::Pixel resolveTrainColor(int train) {
	assert(isValidTrain(train));
	switch (train) {
		case BROWN:
			return olc::Pixel(139,69,19);
		case RED:
			return olc::Pixel(255, 0, 0);
		case BLUE:
			return olc::Pixel(0, 0, 255);
		case YELLOW:
			return olc::Pixel(255, 255, 0);
		case PURPLE:
			return olc::Pixel(148, 0, 211);
		case GREEN:
			return olc::Pixel(0, 255, 0);
		case ORANGE:
			return olc::Pixel(255, 140, 0);
	}
	return olc::Pixel(255,255,255);
}