#include <iostream>
#include "display.h"
#include "arena.h"

using namespace std;

Display::Display() {
	sAppName = "Trainyard Example";
	arena = new Arena;
}

Display::~Display() {
	delete arena;
}

bool Display::OnUserCreate() {
	// Called once at the start, so create things here
	return true;
}

bool Display::OnUserUpdate(float fElapsedTime) {
	// Clear(olc::BLACK);
	// // called once per frame
	// for (int x = 0; x < ScreenWidth(); x++)
	//     for (int y = 0; y < ScreenHeight(); y++)
	//         Draw(x, y, olc::Pixel(rand() % 255, rand() % 255, rand()% 255));
	
	// FillRect(GetMouseX(), GetMouseY(), 1, 1);
	arena->render(this);


	return true;
}