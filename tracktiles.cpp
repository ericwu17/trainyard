#include "tracktiles.h"
#include "train.h"
#include "display.h"
#include "olcPixelGameEngine.h"
#include "sprites.h"
#include <iostream>
#include <cassert>
using namespace std;

const double TWO_PI = 6.2831852;

bool Tile::isATrackTile() const {
	return _isATrackTile;
}

void Tile::setBorder(Edge* border[]) {
	for (int i = 0; i < 4; i ++)
		this->border[i] = border[i];
};

void Tile::render(Display* display, int r, int c, SpriteList* spriteList) const {
	float width = float(spriteList->SPRITE_TRACKTILE_BLANK->width);
	display->DrawDecal(olc::vi2d(c, r) * width, spriteList->TRACKTILE_BLANK);
}


Tracktile::Tracktile() {
	_isATrackTile = true;
	for (int i = 0; i < 2; i ++) {
		activeConnection[i] = nullptr;
		passiveConnection[i] = nullptr;
	}
	for (int i = 0; i < 4; i ++) {
		border[i] = nullptr;
	}
	nTrains = 0;
};

char Tracktile::getRepr() const {
	if (nTrains > 0) {
		return 'T';  // T stands for train
	}
	return classifyConnectionType();
	
}

void Tracktile::render(Display* display, int r, int c, SpriteList* spriteList) const {
	char type = classifyConnectionType();
	float width = float(spriteList->SPRITE_TRACKTILE_BLANK->width);
	int rot;
	switch (type) {
		case '_':
			display->DrawDecal(olc::vi2d(c*width, r*width), spriteList->TRACKTILE_BLANK);
			break;
		case 's':
			if (hasConnection(0,2)) {
				display->DrawRotatedDecal(olc::vi2d(c*width + width/2, r*width + width/2), spriteList->TRACKTILE_S, 0, {width/2, width/2});
			} else {
				display->DrawRotatedDecal(olc::vi2d(c*width + width/2, r*width + width/2), spriteList->TRACKTILE_S, TWO_PI*0.25, {width/2, width/2});
			}
			break;
		case 'b':
			rot = hasConnectionUpToRotation(2, 3);
			display->DrawRotatedDecal(olc::vi2d(c*width + width/2, r*width + width/2), spriteList->TRACKTILE_B, TWO_PI*0.25*rot, {width/2, width/2});
			break;
		case 'z':
			rot = hasConnectionsUpToRotation(0, 1, 2, 3);
			display->DrawRotatedDecal(olc::vi2d(c*width + width/2, r*width + width/2), spriteList->TRACKTILE_Z, TWO_PI*0.25*rot, {width/2, width/2});
			break;
		case 'h':
			if (hasActiveConnection (0, 2)) {
				rot = 0;
			} else {
				rot = 1;
			}
			display->DrawRotatedDecal(olc::vi2d(c*width + width/2, r*width + width/2), spriteList->TRACKTILE_H, TWO_PI*0.25*rot, {width/2, width/2});
			break;
		case 'm':
			rot = hasConnectionsUpToRotation(1, 2, 2, 3);
			if (hasActiveConnection((2+rot)%4, (3+rot)%4) ) {
				display->DrawRotatedDecal(olc::vi2d(c*width + width/2, r*width + width/2), spriteList->TRACKTILE_M, TWO_PI*0.25*rot, {width/2, width/2});
			} else {
				assert (hasActiveConnection((2+rot)%4, (1+rot)%4));
				display->DrawRotatedDecal(olc::vi2d(c*width + width/2, r*width + width/2), spriteList->TRACKTILE_M_FLIPPED, TWO_PI*0.25*rot, {width/2, width/2});
			}

			break;
		case 'j':
			rot = hasConnectionsUpToRotation(0, 2, 2, 3);
			if (rot != -1) {
				if (hasActiveConnection(0, 2) || hasActiveConnection(1, 3)) {
					display->DrawRotatedDecal(olc::vi2d(c*width + width/2, r*width + width/2), spriteList->TRACKTILE_JS, TWO_PI*0.25*rot, {width/2, width/2});
				} else {
					display->DrawRotatedDecal(olc::vi2d(c*width + width/2, r*width + width/2), spriteList->TRACKTILE_JB, TWO_PI*0.25*rot, {width/2, width/2});
				}
			} else {
				rot = hasConnectionsUpToRotation(0, 2, 2, 1);
				assert(rot != -1);
				if (hasActiveConnection(0, 2) || hasActiveConnection(1, 3)) {
					display->DrawRotatedDecal(olc::vi2d(c*width + width/2, r*width + width/2), spriteList->TRACKTILE_JS_FLIPPED, TWO_PI*0.25*rot, {width/2, width/2});
				} else {
					display->DrawRotatedDecal(olc::vi2d(c*width + width/2, r*width + width/2), spriteList->TRACKTILE_JB_FLIPPED, TWO_PI*0.25*rot, {width/2, width/2});
				}
			}
	}
}

bool Tracktile::hasActiveConnection(int d1, int d2) const {
	return (activeConnection[0] == border[d1] && activeConnection[1] == border[d2]) 
	|| (activeConnection[1] == border[d1] && activeConnection[0] == border[d2]);
}

bool Tracktile::hasPassiveConnection(int d1, int d2) const {
	return (passiveConnection[0] == border[d1] && passiveConnection[1] == border[d2]) 
	|| (passiveConnection[1] == border[d1] && passiveConnection[0] == border[d2]);
}

bool Tracktile::hasConnection(int d1, int d2) const {
	return hasActiveConnection(d1, d2) || hasPassiveConnection(d1, d2);
}

bool Tracktile::hasConnections(int d1, int d2, int e1, int e2) const {
	return hasConnection(d1, d2) && hasConnection(e1, e2);
}

int Tracktile::hasConnectionUpToRotation(int d1, int d2) const {
	for (int i = 0; i < 4; i ++) {
		if (hasConnection((d1+i) % 4, (d2+i) % 4)) {
			return i;
		}
	}
	return -1;
}
int Tracktile::hasConnectionsUpToRotation(int d1, int d2, int e1, int e2) const {
	for (int i = 0; i < 4; i ++) {
		if (hasConnection((d1+i) % 4, (d2+i) % 4) && hasConnection((e1+i) % 4, (e2+i) % 4)) {
			return i;
		}
	}
	return -1;
}

char Tracktile::classifyConnectionType() const {
	if (activeConnection[0] == nullptr) {
		return '_';
	}
	if (passiveConnection[0] == nullptr) {
		if (hasConnectionUpToRotation(0, 2) != -1) {
			return 's';
		}
		assert(hasConnectionUpToRotation(0, 1) != -1);
		return 'b';
	}
	// now we can assume that there is both an active and passive connection
	if (hasConnectionsUpToRotation(0, 2, 1, 3) != -1) {
		return 'h';
	} else if (hasConnectionsUpToRotation(0, 1, 2, 3) != -1) {
		return 'z';
	} else if (hasConnectionsUpToRotation(0, 1, 0, 3) != -1) {
		return 'm';
	}
	assert(hasConnectionsUpToRotation(0, 1, 0, 2) != -1 || hasConnectionsUpToRotation(0, 3, 0, 2) != -1);
	return 'j';
}

void Tracktile::addConnection(int d1, int d2) {
	assert(!(d1 < 0 || d1 > 3 || d2 < 0 || d2 > 3));
	assert(d1 != d2);

	if (hasActiveConnection(d1, d2)) {
		// if the user draws a connection that is already the active connection, then we erase the passive connection
		activeConnection[0] = nullptr;
		activeConnection[1] = nullptr;
	}

	passiveConnection[0] = activeConnection[0];
	passiveConnection[1] = activeConnection[1];

	activeConnection[0] = border[d1];
	activeConnection[1] = border[d2];
}

void Tracktile::pullTrainsFromNeighbors() {
	if (activeConnection[0] != nullptr) {
		int t1 = activeConnection[0]->giveTrain(this);
		if (t1 != -1) {
			trains[nTrains] = t1;
			trainSources[nTrains] = activeConnection[0];
			trainDestinations[nTrains] = activeConnection[1];
			nTrains ++;
		}

		t1 = activeConnection[1]->giveTrain(this);
		if (t1 != -1) {
			trains[nTrains] = t1;
			trainSources[nTrains] = activeConnection[1];
			trainDestinations[nTrains] = activeConnection[0];
			nTrains ++;
		}
	}
	if (passiveConnection[0] != nullptr) {
		int t1 = passiveConnection[0]->giveTrain(this);
		if (t1 != -1) {
			trains[nTrains] = t1;
			trainSources[nTrains] = passiveConnection[0];
			trainDestinations[nTrains] = passiveConnection[1];
			nTrains ++;
		}

		t1 = passiveConnection[1]->giveTrain(this);
		if (t1 != -1) {
			trains[nTrains] = t1;
			trainSources[nTrains] = passiveConnection[1];
			trainDestinations[nTrains] = passiveConnection[0];
			nTrains ++;
		}
	}
	
}

void Tracktile::dispatchTrains() {
	for (int i = 0; i < nTrains; i ++) {
		trainDestinations[i]->receiveTrain(this, trains[i]);
	}
	nTrains = 0;
}

bool Tracktile::indicesOfTrainCollidingAlong(Edge* e1, Edge* e2, int & index1, int &index2) const {
	// two trains are colliding alone e1, e2 if one has source: e1, destination: e2 
	// and the other has source: e2, destination: e1
	// if there are no trains colliding along (e1, e2), then we leave index1 and index2 unchanged.
	// return true if there are trains colliding, false otherwise.
	for (int i1 = 0; i1 < nTrains; i1 ++) {
		for (int i2 = 0; i2 < nTrains; i2 ++) {
			if (trainSources[i1] == e1 && trainDestinations[i1] == e2 && trainSources[i2] == e2 && trainDestinations[i2] == e1) {
				index1 = i1;
				index2 = i2;
				return true;
			}
		}
	}
	return false;
}

void Tracktile::interactTrains() {
	char connType = classifyConnectionType();
	assert (connType != ' ');
	if (nTrains < 2) {
		if (nTrains == 1 && (connType == 'j' || connType == 'm')) {
			switchActiveAndPassive();
		}
		
		return;
	}
	if (connType == 'h' || connType == 's' || connType == 'b') {
		// simply mix all the trains in these connection types
		int newTrainColor = mixTrainColors(trains, nTrains);
		for (int i = 0; i < nTrains; i ++) {
			trains[i]  = newTrainColor;
		}
		return;
	}
	int i1, i2;
	if (connType == 'z') {
		// first do mixing on Active Connection

		if (indicesOfTrainCollidingAlong(activeConnection[0], activeConnection[1], i1, i2)) {
			trains[i1] = trains[i2] = mixTrainColors(trains[i1], trains[i2]);
		}
		// then do mixing on Active Connection
		if (indicesOfTrainCollidingAlong(passiveConnection[0], passiveConnection[1], i1, i2)) {
			trains[i1] = trains[i2] = mixTrainColors(trains[i1], trains[i2]);
		}
		return;
	}
	assert(connType == 'j' || connType == 'm');
	// For j and m type connections, we need to worry about switching between active and passive track.
	bool willSwitchTrack = nTrains % 2 == 1;

	if (indicesOfTrainCollidingAlong(activeConnection[0], activeConnection[1], i1, i2)) {
		trains[i1] = trains[i2] = mixTrainColors(trains[i1], trains[i2]);
	}
	// In this case we don't check for trains colliding along the passive connection, because we know it cannot exist on j and m type connections.
	
	// see if two trains have the same destination
	// if they do, merge them
	for (int i1 = 0; i1 < nTrains; i1 ++) {
		for (int i2 = 0; i2 < nTrains; i2 ++) {
			if(i1 == i2) {
				continue;
			}
			if (trainDestinations[i1] == trainDestinations[i2]) {
				trains[i1] = mixTrainColors(trains[i1], trains[i2]);
				
				// remove the i2 index of train arrays
				int temp = trains[i2];
				trains[i2] = trains[nTrains - 1];
				trains[nTrains - 1] = temp;
				Edge* tempEdge = trainSources[i2];
				trainSources[i2] = trainSources[nTrains-1];
				trainSources[nTrains-1] = tempEdge;
				tempEdge = trainDestinations[i2];
				trainDestinations[i2] = trainDestinations[nTrains-1];
				trainDestinations[nTrains-1] = tempEdge;
				nTrains --;
			}
			

		}
	}


	// switch track connections;
	if (willSwitchTrack) {
		switchActiveAndPassive();
	}

}

void Tracktile::switchActiveAndPassive() {
	assert (activeConnection[0] != nullptr && passiveConnection[0] != nullptr);

	for (int i = 0; i < 2; i ++) {
		Edge* temp = activeConnection[i];
		activeConnection[i] = passiveConnection[i];
		passiveConnection[i] = temp;
	}
}





TrainSource::TrainSource(int dir) {
	assert(0 <= dir && dir <= 3);
	targetEdge = border[dir];
	this->dir = dir;
	nTrains = 0;
}

// temp (pls delete me)
void TrainSource::render(Display* display, int r, int c, SpriteList* spriteList) const {
	float rotation = 0;
	float width = float(spriteList->SPRITE_TRACKTILE_BLANK->width);
	switch (dir) {
		case 0:
			rotation = 0.5*TWO_PI;
			break;
		case 1:
			rotation = 0.75*TWO_PI;
			break;
		case 2:
			rotation = 0*TWO_PI;
			break;
		case 3:
			rotation = 0.25*TWO_PI;
			break;
	}
	display->DrawRotatedDecal(olc::vi2d(c*width + width/2, r*width + width/2), spriteList->TRAINSOURCE_BG, rotation, {width/2, width/2});
	display->DrawDecal(olc::vi2d(c, r) * width, spriteList->TRAINSOURCE_AND_SINK);

	float plus_sign_width = float(spriteList->SPRITE_PLUS_SIGN->width);

	float scale;
	int num_cols;
	if(nTrains <= 1) {
		scale = 1;
		num_cols = 1;
	} else if (nTrains <= 4) {
		scale = 0.5;
		num_cols = 2;
	} else {
		assert(nTrains <= 9);
		scale = 0.33;
		num_cols = 3;
	}

	for (int i = 0; i < nTrains; i ++) {
		int currCol = i%num_cols;
		int currRow = i/num_cols;

		double xPos = c*width + (width - plus_sign_width)/2 + currCol*(plus_sign_width*scale);
		double yPos = r*width + (width - plus_sign_width)/2 + currRow*(plus_sign_width*scale);
		display->DrawDecal(olc::vi2d(xPos, yPos), spriteList->PLUS_SIGN, {scale, scale}, resolveTrainColor(trains[i]));
	}
}

void TrainSource::setTrains(int trains[], int nTrains) {
	assert(nTrains <= MAX_NUM_TRAINS_IN_STATION);
	for (int i = 0; i < nTrains; i ++) {
		this->trains[i] = trains[i];
	}
	this->nTrains = nTrains;
}

void TrainSource::setBorder(Edge* border[]) {
	targetEdge = border[dir];
	for (int i = 0; i < 4; i ++)
		this->border[i] = border[i];
}

void TrainSource::dispatchTrains() {
	if (nTrains == 0) {
		return;
	}
	targetEdge->receiveTrain(this, trains[nTrains-1]);
	nTrains --;
}

char TrainSource::getRepr() const {
	return nTrains + '0';
}

TrainSink::TrainSink(bool canReceiveTrain[]) {
	for (int i = 0; i < 4; i ++) {
		this->canReceiveTrain[i] = canReceiveTrain[i];
	}
	nTrains = 0;
}

void TrainSink::setBorder(Edge* border[]) {
	for (int i = 0; i < 4; i ++)
		this->border[i] = border[i];
}
void TrainSink::setDesires(int trains[], int nTrains) {
	assert(nTrains <= MAX_NUM_TRAINS_IN_STATION);
	for (int i = 0; i < nTrains; i ++) {
		desiredTrains[i] = trains[i];
	}
	this->nTrains = nTrains;
}
void TrainSink::pullTrainsFromNeighbors() {
	// pull in a train only if there is a train which matches one of the desired trains.
	for (int dir = 0; dir < 4; dir ++) {
		if (canReceiveTrain[dir]) {
			int incomingTrain = border[dir]->softGiveTrain(this);
			for (int i = 0; i < nTrains; i ++) {
				if (incomingTrain == desiredTrains[i]) {
					border[dir]->giveTrain(this);
					
					//delete the i-th train from the array of desired trains while keeping the rest in order
					for (int k = i; k+1 < nTrains; k ++) {
						desiredTrains[k] = desiredTrains[k+1];
					}
					nTrains --;
				}
			}
		}
	}
}

bool TrainSink::isSatisfied() {
	return nTrains == 0;
}

char TrainSink::getRepr() const {
	return nTrains + '0';
}

void TrainSink::render(Display* display, int r, int c, SpriteList* spriteList) const {
	float width = float(spriteList->SPRITE_TRACKTILE_BLANK->width);
	display->DrawDecal(olc::vi2d(c, r) * width, spriteList->TRACKTILE_BLANK);

	for (int i = 0; i < 4; i ++) {
		if (canReceiveTrain[i]) {
			display->DrawRotatedDecal(olc::vi2d(c*width + width/2, r*width + width/2), spriteList->TRAINSINK_ENTRY, i*0.25*TWO_PI, {width/2, width/2});
		}
	}
	display->DrawDecal(olc::vi2d(c, r) * width, spriteList->TRAINSOURCE_AND_SINK);


	float circle_width = float(spriteList->SPRITE_CIRCLE->width);

	float scale;
	int num_cols;
	if(nTrains <= 1) {
		scale = 1;
		num_cols = 1;
	} else if (nTrains <= 4) {
		scale = 0.5;
		num_cols = 2;
	} else {
		assert(nTrains <= 9);
		scale = 0.33;
		num_cols = 3;
	}

	for (int i = 0; i < nTrains; i ++) {
		int currCol = i%num_cols;
		int currRow = i/num_cols;

		double xPos = c*width + (width - circle_width)/2 + currCol*(circle_width*scale);
		double yPos = r*width + (width - circle_width)/2 + currRow*(circle_width*scale);
		display->DrawDecal(olc::vi2d(xPos, yPos), spriteList->CIRCLE, {scale, scale}, resolveTrainColor(desiredTrains[i]));
	}

}