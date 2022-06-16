#ifndef SPRITES_H
#define SPRITES_H


#include "olcPixelGameEngine.h"

using olc::Sprite;

struct SpriteList {
public:
    SpriteList () {
        TRACKTILE_BLANK = Sprite("./images/Tracktile_blank.png");
        TRACKTILE_B = Sprite("./images/Tracktile_b.png");
        TRACKTILE_H = Sprite("./images/Tracktile_h.png");
        TRACKTILE_JB = Sprite("./images/Tracktile_jb.png");
        TRACKTILE_JS = Sprite("./images/Tracktile_js.png");
        TRACKTILE_M = Sprite("./images/Tracktile_m.png");
        TRACKTILE_S = Sprite("./images/Tracktile_s.png");
        TRACKTILE_Z = Sprite("./images/Tracktile_z.png");
        TRAINSOURCE_AND_SINK = Sprite("./images/Trainsource_and_sink.png");
        TRAINSINK_BG = Sprite("./images/Trainsink_bg.png");
        TRAINSOURCE_BG = Sprite("./images/Trainsource_bg.png");
        PLUS_SIGN = Sprite("./images/Plus_sign.png");
    };
    Sprite TRACKTILE_BLANK;
    Sprite TRACKTILE_B;
    Sprite TRACKTILE_H;
    Sprite TRACKTILE_JB;
    Sprite TRACKTILE_JS;
    Sprite TRACKTILE_M;
    Sprite TRACKTILE_S;
    Sprite TRACKTILE_Z;
    Sprite TRAINSOURCE_AND_SINK;
    Sprite TRAINSOURCE_BG;
    Sprite TRAINSINK_BG;
    Sprite PLUS_SIGN;
};





#endif