#ifndef DISPLAY_H
#define DISPLAY_H

#include <iostream>
#include "arena.h"


#include "olcPixelGameEngine.h"

using namespace std;

class Arena;

class Display : public olc::PixelGameEngine {
	public:
		Display();
		~Display();

	public:
		bool OnUserCreate() override;

		bool OnUserUpdate(float fElapsedTime) override;
	private:
		Arena* arena;
};


#endif
