#ifndef TRAIN_H
#define TRAIN_H

#include <iostream>
using namespace std;

const int BROWN   = 0;
const int RED     = 1;
const int BLUE    = 2;
const int YELLOW  = 3;
const int PURPLE  = 4;
const int GREEN   = 5;
const int ORANGE  = 6;


int mixTrainColors(int train1, int train2);

bool isValidTrain(int train);

#endif