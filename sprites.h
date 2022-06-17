#ifndef SPRITES_H
#define SPRITES_H


#include "olcPixelGameEngine.h"

using olc::Sprite;
using olc::Decal;

struct SpriteList {
public:
    SpriteList () {
        SPRITE_TRACKTILE_BLANK = new Sprite("./images/Tracktile_blank.png");
        SPRITE_TRACKTILE_B = new Sprite("./images/Tracktile_b.png");
        SPRITE_TRACKTILE_H = new Sprite("./images/Tracktile_h.png");
        SPRITE_TRACKTILE_JB = new Sprite("./images/Tracktile_jb.png");
        SPRITE_TRACKTILE_JB_FLIPPED = new Sprite("./images/Tracktile_jb_flipped.png");
        SPRITE_TRACKTILE_JS = new Sprite("./images/Tracktile_js.png");
        SPRITE_TRACKTILE_JS_FLIPPED = new Sprite("./images/Tracktile_js_flipped.png");
        SPRITE_TRACKTILE_M = new Sprite("./images/Tracktile_m.png");
        SPRITE_TRACKTILE_M_FLIPPED = new Sprite("./images/Tracktile_m_flipped.png");
        SPRITE_TRACKTILE_S = new Sprite("./images/Tracktile_s.png");
        SPRITE_TRACKTILE_Z = new Sprite("./images/Tracktile_z.png");
        SPRITE_TRAINSOURCE_AND_SINK = new Sprite("./images/Trainsource_and_sink.png");
        SPRITE_TRAINSINK_BG = new Sprite("./images/Trainsink_bg.png");
        SPRITE_TRAINSOURCE_BG = new Sprite("./images/Trainsource_bg.png");
        SPRITE_PLUS_SIGN = new Sprite("./images/Plus_sign.png");
        SPRITE_TRAINSINK_ENTRY = new Sprite("./images/Trainsink_entry.png");
        SPRITE_CIRCLE = new Sprite("./images/Circle.png");
        SPRITE_TRAIN = new Sprite("./images/Train.png");

        TRACKTILE_BLANK = new Decal(SPRITE_TRACKTILE_BLANK);
        TRACKTILE_B = new Decal(SPRITE_TRACKTILE_B);
        TRACKTILE_H = new Decal(SPRITE_TRACKTILE_H);
        TRACKTILE_JB = new Decal(SPRITE_TRACKTILE_JB);
        TRACKTILE_JB_FLIPPED = new Decal(SPRITE_TRACKTILE_JB_FLIPPED);
        TRACKTILE_JS = new Decal(SPRITE_TRACKTILE_JS);
        TRACKTILE_JS_FLIPPED = new Decal(SPRITE_TRACKTILE_JS_FLIPPED);
        TRACKTILE_M = new Decal(SPRITE_TRACKTILE_M);
        TRACKTILE_M_FLIPPED = new Decal(SPRITE_TRACKTILE_M_FLIPPED);
        TRACKTILE_S = new Decal(SPRITE_TRACKTILE_S);
        TRACKTILE_Z = new Decal(SPRITE_TRACKTILE_Z);
        TRAINSOURCE_AND_SINK = new Decal(SPRITE_TRAINSOURCE_AND_SINK);
        TRAINSINK_BG = new Decal(SPRITE_TRAINSINK_BG);
        TRAINSOURCE_BG = new Decal(SPRITE_TRAINSOURCE_BG);
        PLUS_SIGN = new Decal(SPRITE_PLUS_SIGN);
        TRAINSINK_ENTRY = new Decal(SPRITE_TRAINSINK_ENTRY);
        CIRCLE = new Decal(SPRITE_CIRCLE);
        TRAIN = new Decal(SPRITE_TRAIN);
    };
    Sprite* SPRITE_TRACKTILE_BLANK;
    Decal* TRACKTILE_BLANK;
    Sprite* SPRITE_TRACKTILE_B;
    Decal* TRACKTILE_B;
    Sprite* SPRITE_TRACKTILE_H;
    Decal* TRACKTILE_H;
    Sprite* SPRITE_TRACKTILE_JB;
    Decal* TRACKTILE_JB;
    Sprite* SPRITE_TRACKTILE_JB_FLIPPED;
    Decal* TRACKTILE_JB_FLIPPED;
    Sprite* SPRITE_TRACKTILE_JS;
    Decal* TRACKTILE_JS;
    Sprite* SPRITE_TRACKTILE_JS_FLIPPED;
    Decal* TRACKTILE_JS_FLIPPED;
    Sprite* SPRITE_TRACKTILE_M;
    Decal* TRACKTILE_M;
    Sprite* SPRITE_TRACKTILE_M_FLIPPED;
    Decal* TRACKTILE_M_FLIPPED;
    Sprite* SPRITE_TRACKTILE_S;
    Decal* TRACKTILE_S;
    Sprite* SPRITE_TRACKTILE_Z;
    Decal* TRACKTILE_Z;
    Sprite* SPRITE_TRAINSOURCE_AND_SINK;
    Decal* TRAINSOURCE_AND_SINK;
    Sprite* SPRITE_TRAINSOURCE_BG;
    Decal* TRAINSOURCE_BG;
    Sprite* SPRITE_TRAINSINK_BG;
    Decal* TRAINSINK_BG;
    Sprite* SPRITE_PLUS_SIGN;
    Decal* PLUS_SIGN;
    Sprite* SPRITE_TRAINSINK_ENTRY;
    Decal* TRAINSINK_ENTRY;
    Sprite* SPRITE_CIRCLE;
    Decal* CIRCLE;
    Sprite* SPRITE_TRAIN;
    Decal* TRAIN;
};





#endif