#include <iostream>
#include "display.h"
#include "arena.h"
#include "sprites.h"

using namespace std;



Display::Display() {
	sAppName = "Trainyard Example";
	arena = new Arena;
	mousePos[0] = -1;
	mousePos[1] = -1;
	mouseDir = -1;
	prevMousePos[0] = -1;
	prevMousePos[1] = -1;
	prevMouseDir = -1;
}

Display::~Display() {
	delete arena;
	delete SPRITE_TRACKTILE_BLANK;
}

bool Display::OnUserCreate() {
	// Called once at the start, so create things here
	spriteList = new SpriteList;
	return true;
}

bool Display::OnUserUpdate(float fElapsedTime) {
	const float gridSize = float(spriteList->SPRITE_TRACKTILE_BLANK->width);
	float mouseX =  float(GetMouseX());
	float mouseY =  float(GetMouseY());
	mousePos[0] = mouseX/gridSize;
	mousePos[1] = mouseY/gridSize;
	
	int distToUp = int(mouseY) % int(gridSize);
	int distToLeft = int(mouseX) % int(gridSize);
	int distToDown = gridSize - distToUp;
	int distToRight = gridSize - distToLeft;
	int minDir = 0;
	int minDist = distToUp;
	if (distToRight < minDist) {
		minDir = 1;
		minDist = distToRight;
	}
	if (distToDown < minDist) {
		minDir = 2;
		minDist = distToDown;
	}
	if (distToLeft < minDist) {
		minDir = 3;
		minDist = distToLeft;
	}
	if (minDist < gridSize * 0.2){
		mouseDir = minDir;
	} else {
		mouseDir = -1;
	}

	// cout << prevMousePos[0] << ' ' << prevMousePos[1] << ' ' << mousePos[0] << ' ' << mousePos[1] << ' ' << mouseDir << endl;
	if (GetMouse(0).bHeld) {
		if (prevMousePos[0] == mousePos[0] && prevMousePos[1] == mousePos[1]) {
			if (prevMouseDir != mouseDir && mouseDir != -1 && prevMouseDir != -1) {
				arena->addConnection(mousePos[1], mousePos[0], prevMouseDir, mouseDir);
			}
		}
		if (mouseDir != -1) {
			prevMouseDir = mouseDir;
		}
		prevMousePos[0] = mousePos[0];
		prevMousePos[1] = mousePos[1];
	} else {
		prevMousePos[0] = -1;
		prevMousePos[1] = -1;
	}

	
	arena->render(this, spriteList);
	if (GetKey(olc::Key::N).bPressed) {
		arena->processTick();
	}


	return true;
}