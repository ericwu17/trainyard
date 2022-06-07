#include "train.h"
#include <iostream>
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
