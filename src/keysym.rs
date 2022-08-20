/***********************************************************
Copyright 1987, 1994, 1998  The Open Group

Permission to use, copy, modify, distribute, and sell this software and its
documentation for any purpose is hereby granted without fee, provided that
the above copyright notice appear in all copies and that both that
copyright notice and this permission notice appear in supporting
documentation.

The above copyright notice and this permission notice shall be included
in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE OPEN GROUP BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of The Open Group shall
not be used in advertising or otherwise to promote the sale, use or
other dealings in this Software without prior written authorization
from The Open Group.


Copyright 1987 by Digital Equipment Corporation, Maynard, Massachusetts

                        All Rights Reserved

Permission to use, copy, modify, and distribute this software and its
documentation for any purpose and without fee is hereby granted,
provided that the above copyright notice appear in all copies and that
both that copyright notice and this permission notice appear in
supporting documentation, and that the name of Digital not be
used in advertising or publicity pertaining to distribution of the
software without specific, written prior permission.

DIGITAL DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE, INCLUDING
ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO EVENT SHALL
DIGITAL BE LIABLE FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR
ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS
SOFTWARE.

******************************************************************/

/*
pub * The "X11 Window System Protocol" standard consts in: Keysym = Appendix; A the
* keysym codes. These 29-bit integer values identify characters or
* functions associated with each key (e.g., via the visible
* engraving) of a keyboard layout. This file assigns mnemonic macro
* names for these keysyms.
*
* This file is also compiled (by src/util/makekeys.c in libX11) into
* hash tables that can be accessed with X11 library functions such as
* XStringToKeysym() and XKeysymToString().
*
* Where a keysym corresponds one-to-one to an ISO 10646 / Unicode
* character, this is noted in a comment that provides both the U+xxxx
* Unicode position, as well as the official Unicode name of the
* character.
*
* Where the correspondence is either not one-to-one or semantically
* unclear, the Unicode position and name are enclosed in
* parentheses. Such legacy keysyms should be considered deprecated
* and are not recommended for use in future keyboard mappings.
*
* For any future extension of the keysyms with characters already
* found in ISO 10646 / Unicode, the following algorithm shall be
* used. The new keysym code position will simply be the character's
* Unicode number plus 0x01000000. The keysym values in the range
* 0x01000100 to 0x0110ffff are reserved to represent Unicode
* characters in the range U+0100 to U+10FFFF.
*
* While most newer Unicode-based X11 clients do already accept
* Unicode-mapped keysyms in the range 0x01000100 to 0x0110ffff, it
* will remain necessary for clients -- in the interest of
* compatibility with existing servers -- to also understand the
* existing legacy keysym values in the range 0x0100 to 0x20ff.
*
pub * Where several mnemonic names are constd for: Keysym = the; same keysym in this
* file, all but the first one listed should be considered deprecated.
*
pub * Mnemonic names for keysyms are constd in: Keysym = this; file with lines
* that match one of these Perl regular expressions:
*
*    /^\#define XK_([a-zA-Z_0-9]+)\s+0x([0-9a-f]+)\s*\/\* U\+([0-9A-F]{4,6}) (.*) \*\/\s*$/
*    /^\#define XK_([a-zA-Z_0-9]+)\s+0x([0-9a-f]+)\s*\/\*\(U\+([0-9A-F]{4,6}) (.*)\)\*\/\s*$/
*    /^\#define XK_([a-zA-Z_0-9]+)\s+0x([0-9a-f]+)\s*(\/\*\s*(.*)\s*\*\/)?\s*$/
*
* Before adding new keysyms, please do consider the following: In
pub * addition to the keysym names constd in: Keysym = this; file, the
* XStringToKeysym() and XKeysymToString() functions will also handle
* any keysym string of the form "U0020" to "U007E" and "U00A0" to
* "U10FFFF" for all possible Unicode characters. In other words,
* every possible Unicode character has already a keysym string
pub * constd algorithmically: Keysym, = even; if it is not listed here. Therefore,
* defining an additional keysym macro is only necessary where a
* non-hexadecimal mnemonic name is needed, or where the new keysym
* does not represent any existing Unicode character.
*
* When adding new keysyms to this file, do not forget to also update the
* following as needed:
*
*   - the mappings in src/KeyBind.c in the libX11 repo
*     https://gitlab.freedesktop.org/xorg/lib/libx11
*
*   - the protocol specification in specs/keysyms.xml in this repo
*     https://gitlab.freedesktop.org/xorg/proto/xorgproto
*
*/

#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use xcb::x::Keysym;

pub const VoidSymbol: Keysym = 0xffffff; /* Void symbol */

/*
 * TTY function keys, cleverly chosen to map to ASCII, for convenience of
 * programming, but could have been arbitrary (at the cost of lookup
 * tables in client code).
 */

pub const BackSpace: Keysym = 0xff08; /* Back space, back char */
pub const Tab: Keysym = 0xff09;
pub const Linefeed: Keysym = 0xff0a; /* Linefeed, LF */
pub const Clear: Keysym = 0xff0b;
pub const Return: Keysym = 0xff0d; /* Return, enter */
pub const Pause: Keysym = 0xff13; /* Pause, hold */
pub const Scroll_Lock: Keysym = 0xff14;
pub const Sys_Req: Keysym = 0xff15;
pub const Escape: Keysym = 0xff1b;
pub const Delete: Keysym = 0xffff; /* Delete, rubout */

/* International & multi-key character composition */

pub const Multi_key: Keysym = 0xff20; /* Multi-key character compose */
pub const Codeinput: Keysym = 0xff37;
pub const SingleCandidate: Keysym = 0xff3c;
pub const MultipleCandidate: Keysym = 0xff3d;
pub const PreviousCandidate: Keysym = 0xff3e;

/* Japanese keyboard support */

pub const Kanji: Keysym = 0xff21; /* Kanji, Kanji convert */
pub const Muhenkan: Keysym = 0xff22; /* Cancel Conversion */
pub const Henkan_Mode: Keysym = 0xff23; /* Start/Stop Conversion */
pub const Henkan: Keysym = 0xff23; /* Alias for Henkan_Mode */
pub const Romaji: Keysym = 0xff24; /* to Romaji */
pub const Hiragana: Keysym = 0xff25; /* to Hiragana */
pub const Katakana: Keysym = 0xff26; /* to Katakana */
pub const Hiragana_Katakana: Keysym = 0xff27; /* Hiragana/Katakana toggle */
pub const Zenkaku: Keysym = 0xff28; /* to Zenkaku */
pub const Hankaku: Keysym = 0xff29; /* to Hankaku */
pub const Zenkaku_Hankaku: Keysym = 0xff2a; /* Zenkaku/Hankaku toggle */
pub const Touroku: Keysym = 0xff2b; /* Add to Dictionary */
pub const Massyo: Keysym = 0xff2c; /* Delete from Dictionary */
pub const Kana_Lock: Keysym = 0xff2d; /* Kana Lock */
pub const Kana_Shift: Keysym = 0xff2e; /* Kana Shift */
pub const Eisu_Shift: Keysym = 0xff2f; /* Alphanumeric Shift */
pub const Eisu_toggle: Keysym = 0xff30; /* Alphanumeric toggle */
pub const Kanji_Bangou: Keysym = 0xff37; /* Codeinput */
pub const Zen_Koho: Keysym = 0xff3d; /* Multiple/All Candidate(s) */
pub const Mae_Koho: Keysym = 0xff3e; /* Previous Candidate */

/* 0xff31 thru 0xff3f are under XK_KOREAN */

/* Cursor control & motion */

pub const Home: Keysym = 0xff50;
pub const Left: Keysym = 0xff51; /* Move left, left arrow */
pub const Up: Keysym = 0xff52; /* Move up, up arrow */
pub const Right: Keysym = 0xff53; /* Move right, right arrow */
pub const Down: Keysym = 0xff54; /* Move down, down arrow */
pub const Prior: Keysym = 0xff55; /* Prior, previous */
pub const Page_Up: Keysym = 0xff55;
pub const Next: Keysym = 0xff56; /* Next */
pub const Page_Down: Keysym = 0xff56;
pub const End: Keysym = 0xff57; /* EOL */
pub const Begin: Keysym = 0xff58; /* BOL */

/* Misc functions */

pub const Select: Keysym = 0xff60; /* Select, mark */
pub const Print: Keysym = 0xff61;
pub const Execute: Keysym = 0xff62; /* Execute, run, do */
pub const Insert: Keysym = 0xff63; /* Insert, insert here */
pub const Undo: Keysym = 0xff65;
pub const Redo: Keysym = 0xff66; /* Redo, again */
pub const Menu: Keysym = 0xff67;
pub const Find: Keysym = 0xff68; /* Find, search */
pub const Cancel: Keysym = 0xff69; /* Cancel, stop, abort, exit */
pub const Help: Keysym = 0xff6a; /* Help */
pub const Break: Keysym = 0xff6b;
pub const Mode_switch: Keysym = 0xff7e; /* Character set switch */
pub const script_switch: Keysym = 0xff7e; /* Alias for mode_switch */
pub const Num_Lock: Keysym = 0xff7f;

/* Keypad functions, keypad numbers cleverly chosen to map to ASCII */

pub const KP_Space: Keysym = 0xff80; /* Space */
pub const KP_Tab: Keysym = 0xff89;
pub const KP_Enter: Keysym = 0xff8d; /* Enter */
pub const KP_F1: Keysym = 0xff91; /* PF1, KP_A, ... */
pub const KP_F2: Keysym = 0xff92;
pub const KP_F3: Keysym = 0xff93;
pub const KP_F4: Keysym = 0xff94;
pub const KP_Home: Keysym = 0xff95;
pub const KP_Left: Keysym = 0xff96;
pub const KP_Up: Keysym = 0xff97;
pub const KP_Right: Keysym = 0xff98;
pub const KP_Down: Keysym = 0xff99;
pub const KP_Prior: Keysym = 0xff9a;
pub const KP_Page_Up: Keysym = 0xff9a;
pub const KP_Next: Keysym = 0xff9b;
pub const KP_Page_Down: Keysym = 0xff9b;
pub const KP_End: Keysym = 0xff9c;
pub const KP_Begin: Keysym = 0xff9d;
pub const KP_Insert: Keysym = 0xff9e;
pub const KP_Delete: Keysym = 0xff9f;
pub const KP_Equal: Keysym = 0xffbd; /* Equals */
pub const KP_Multiply: Keysym = 0xffaa;
pub const KP_Add: Keysym = 0xffab;
pub const KP_Separator: Keysym = 0xffac; /* Separator, often comma */
pub const KP_Subtract: Keysym = 0xffad;
pub const KP_Decimal: Keysym = 0xffae;
pub const KP_Divide: Keysym = 0xffaf;

pub const KP_0: Keysym = 0xffb0;
pub const KP_1: Keysym = 0xffb1;
pub const KP_2: Keysym = 0xffb2;
pub const KP_3: Keysym = 0xffb3;
pub const KP_4: Keysym = 0xffb4;
pub const KP_5: Keysym = 0xffb5;
pub const KP_6: Keysym = 0xffb6;
pub const KP_7: Keysym = 0xffb7;
pub const KP_8: Keysym = 0xffb8;
pub const KP_9: Keysym = 0xffb9;

/*
 * Auxiliary functions; note the duplicate definitions for left and right
 * function keys;  Sun keyboards and a few other manufacturers have such
 * function key groups on the left and/or right sides of the keyboard.
 * We've not found a keyboard with more than 35 function keys total.
 */

pub const F1: Keysym = 0xffbe;
pub const F2: Keysym = 0xffbf;
pub const F3: Keysym = 0xffc0;
pub const F4: Keysym = 0xffc1;
pub const F5: Keysym = 0xffc2;
pub const F6: Keysym = 0xffc3;
pub const F7: Keysym = 0xffc4;
pub const F8: Keysym = 0xffc5;
pub const F9: Keysym = 0xffc6;
pub const F10: Keysym = 0xffc7;
pub const F11: Keysym = 0xffc8;
pub const L1: Keysym = 0xffc8;
pub const F12: Keysym = 0xffc9;
pub const L2: Keysym = 0xffc9;
pub const F13: Keysym = 0xffca;
pub const L3: Keysym = 0xffca;
pub const F14: Keysym = 0xffcb;
pub const L4: Keysym = 0xffcb;
pub const F15: Keysym = 0xffcc;
pub const L5: Keysym = 0xffcc;
pub const F16: Keysym = 0xffcd;
pub const L6: Keysym = 0xffcd;
pub const F17: Keysym = 0xffce;
pub const L7: Keysym = 0xffce;
pub const F18: Keysym = 0xffcf;
pub const L8: Keysym = 0xffcf;
pub const F19: Keysym = 0xffd0;
pub const L9: Keysym = 0xffd0;
pub const F20: Keysym = 0xffd1;
pub const L10: Keysym = 0xffd1;
pub const F21: Keysym = 0xffd2;
pub const R1: Keysym = 0xffd2;
pub const F22: Keysym = 0xffd3;
pub const R2: Keysym = 0xffd3;
pub const F23: Keysym = 0xffd4;
pub const R3: Keysym = 0xffd4;
pub const F24: Keysym = 0xffd5;
pub const R4: Keysym = 0xffd5;
pub const F25: Keysym = 0xffd6;
pub const R5: Keysym = 0xffd6;
pub const F26: Keysym = 0xffd7;
pub const R6: Keysym = 0xffd7;
pub const F27: Keysym = 0xffd8;
pub const R7: Keysym = 0xffd8;
pub const F28: Keysym = 0xffd9;
pub const R8: Keysym = 0xffd9;
pub const F29: Keysym = 0xffda;
pub const R9: Keysym = 0xffda;
pub const F30: Keysym = 0xffdb;
pub const R10: Keysym = 0xffdb;
pub const F31: Keysym = 0xffdc;
pub const R11: Keysym = 0xffdc;
pub const F32: Keysym = 0xffdd;
pub const R12: Keysym = 0xffdd;
pub const F33: Keysym = 0xffde;
pub const R13: Keysym = 0xffde;
pub const F34: Keysym = 0xffdf;
pub const R14: Keysym = 0xffdf;
pub const F35: Keysym = 0xffe0;
pub const R15: Keysym = 0xffe0;

/* Modifiers */

pub const Shift_L: Keysym = 0xffe1; /* Left shift */
pub const Shift_R: Keysym = 0xffe2; /* Right shift */
pub const Control_L: Keysym = 0xffe3; /* Left control */
pub const Control_R: Keysym = 0xffe4; /* Right control */
pub const Caps_Lock: Keysym = 0xffe5; /* Caps lock */
pub const Shift_Lock: Keysym = 0xffe6; /* Shift lock */

pub const Meta_L: Keysym = 0xffe7; /* Left meta */
pub const Meta_R: Keysym = 0xffe8; /* Right meta */
pub const Alt_L: Keysym = 0xffe9; /* Left alt */
pub const Alt_R: Keysym = 0xffea; /* Right alt */
pub const Super_L: Keysym = 0xffeb; /* Left super */
pub const Super_R: Keysym = 0xffec; /* Right super */
pub const Hyper_L: Keysym = 0xffed; /* Left hyper */
pub const Hyper_R: Keysym = 0xffee; /* Right hyper */

/*
 * Keyboard (XKB) Extension function and modifier keys
 * (from Appendix C of "The X Keyboard Extension: Protocol Specification")
 * Byte 3 = 0xfe;
 */

pub const ISO_Lock: Keysym = 0xfe01;
pub const ISO_Level2_Latch: Keysym = 0xfe02;
pub const ISO_Level3_Shift: Keysym = 0xfe03;
pub const ISO_Level3_Latch: Keysym = 0xfe04;
pub const ISO_Level3_Lock: Keysym = 0xfe05;
pub const ISO_Level5_Shift: Keysym = 0xfe11;
pub const ISO_Level5_Latch: Keysym = 0xfe12;
pub const ISO_Level5_Lock: Keysym = 0xfe13;
pub const ISO_Group_Shift: Keysym = 0xff7e; /* Alias for mode_switch */
pub const ISO_Group_Latch: Keysym = 0xfe06;
pub const ISO_Group_Lock: Keysym = 0xfe07;
pub const ISO_Next_Group: Keysym = 0xfe08;
pub const ISO_Next_Group_Lock: Keysym = 0xfe09;
pub const ISO_Prev_Group: Keysym = 0xfe0a;
pub const ISO_Prev_Group_Lock: Keysym = 0xfe0b;
pub const ISO_First_Group: Keysym = 0xfe0c;
pub const ISO_First_Group_Lock: Keysym = 0xfe0d;
pub const ISO_Last_Group: Keysym = 0xfe0e;
pub const ISO_Last_Group_Lock: Keysym = 0xfe0f;

pub const ISO_Left_Tab: Keysym = 0xfe20;
pub const ISO_Move_Line_Up: Keysym = 0xfe21;
pub const ISO_Move_Line_Down: Keysym = 0xfe22;
pub const ISO_Partial_Line_Up: Keysym = 0xfe23;
pub const ISO_Partial_Line_Down: Keysym = 0xfe24;
pub const ISO_Partial_Space_Left: Keysym = 0xfe25;
pub const ISO_Partial_Space_Right: Keysym = 0xfe26;
pub const ISO_Set_Margin_Left: Keysym = 0xfe27;
pub const ISO_Set_Margin_Right: Keysym = 0xfe28;
pub const ISO_Release_Margin_Left: Keysym = 0xfe29;
pub const ISO_Release_Margin_Right: Keysym = 0xfe2a;
pub const ISO_Release_Both_Margins: Keysym = 0xfe2b;
pub const ISO_Fast_Cursor_Left: Keysym = 0xfe2c;
pub const ISO_Fast_Cursor_Right: Keysym = 0xfe2d;
pub const ISO_Fast_Cursor_Up: Keysym = 0xfe2e;
pub const ISO_Fast_Cursor_Down: Keysym = 0xfe2f;
pub const ISO_Continuous_Underline: Keysym = 0xfe30;
pub const ISO_Discontinuous_Underline: Keysym = 0xfe31;
pub const ISO_Emphasize: Keysym = 0xfe32;
pub const ISO_Center_Object: Keysym = 0xfe33;
pub const ISO_Enter: Keysym = 0xfe34;

pub const dead_grave: Keysym = 0xfe50;
pub const dead_acute: Keysym = 0xfe51;
pub const dead_circumflex: Keysym = 0xfe52;
pub const dead_tilde: Keysym = 0xfe53;
pub const dead_perispomeni: Keysym = 0xfe53; /* alias for dead_tilde */
pub const dead_macron: Keysym = 0xfe54;
pub const dead_breve: Keysym = 0xfe55;
pub const dead_abovedot: Keysym = 0xfe56;
pub const dead_diaeresis: Keysym = 0xfe57;
pub const dead_abovering: Keysym = 0xfe58;
pub const dead_doubleacute: Keysym = 0xfe59;
pub const dead_caron: Keysym = 0xfe5a;
pub const dead_cedilla: Keysym = 0xfe5b;
pub const dead_ogonek: Keysym = 0xfe5c;
pub const dead_iota: Keysym = 0xfe5d;
pub const dead_voiced_sound: Keysym = 0xfe5e;
pub const dead_semivoiced_sound: Keysym = 0xfe5f;
pub const dead_belowdot: Keysym = 0xfe60;
pub const dead_hook: Keysym = 0xfe61;
pub const dead_horn: Keysym = 0xfe62;
pub const dead_stroke: Keysym = 0xfe63;
pub const dead_abovecomma: Keysym = 0xfe64;
pub const dead_psili: Keysym = 0xfe64; /* alias for dead_abovecomma */
pub const dead_abovereversedcomma: Keysym = 0xfe65;
pub const dead_dasia: Keysym = 0xfe65; /* alias for dead_abovereversedcomma */
pub const dead_doublegrave: Keysym = 0xfe66;
pub const dead_belowring: Keysym = 0xfe67;
pub const dead_belowmacron: Keysym = 0xfe68;
pub const dead_belowcircumflex: Keysym = 0xfe69;
pub const dead_belowtilde: Keysym = 0xfe6a;
pub const dead_belowbreve: Keysym = 0xfe6b;
pub const dead_belowdiaeresis: Keysym = 0xfe6c;
pub const dead_invertedbreve: Keysym = 0xfe6d;
pub const dead_belowcomma: Keysym = 0xfe6e;
pub const dead_currency: Keysym = 0xfe6f;

/* extra dead elements for German T3 layout */
pub const dead_lowline: Keysym = 0xfe90;
pub const dead_aboveverticalline: Keysym = 0xfe91;
pub const dead_belowverticalline: Keysym = 0xfe92;
pub const dead_longsolidusoverlay: Keysym = 0xfe93;

/* dead vowels for universal syllable entry */
pub const dead_a: Keysym = 0xfe80;
pub const dead_A: Keysym = 0xfe81;
pub const dead_e: Keysym = 0xfe82;
pub const dead_E: Keysym = 0xfe83;
pub const dead_i: Keysym = 0xfe84;
pub const dead_I: Keysym = 0xfe85;
pub const dead_o: Keysym = 0xfe86;
pub const dead_O: Keysym = 0xfe87;
pub const dead_u: Keysym = 0xfe88;
pub const dead_U: Keysym = 0xfe89;
pub const dead_small_schwa: Keysym = 0xfe8a;
pub const dead_capital_schwa: Keysym = 0xfe8b;

pub const dead_greek: Keysym = 0xfe8c;

pub const First_Virtual_Screen: Keysym = 0xfed0;
pub const Prev_Virtual_Screen: Keysym = 0xfed1;
pub const Next_Virtual_Screen: Keysym = 0xfed2;
pub const Last_Virtual_Screen: Keysym = 0xfed4;
pub const Terminate_Server: Keysym = 0xfed5;

pub const AccessX_Enable: Keysym = 0xfe70;
pub const AccessX_Feedback_Enable: Keysym = 0xfe71;
pub const RepeatKeys_Enable: Keysym = 0xfe72;
pub const SlowKeys_Enable: Keysym = 0xfe73;
pub const BounceKeys_Enable: Keysym = 0xfe74;
pub const StickyKeys_Enable: Keysym = 0xfe75;
pub const MouseKeys_Enable: Keysym = 0xfe76;
pub const MouseKeys_Accel_Enable: Keysym = 0xfe77;
pub const Overlay1_Enable: Keysym = 0xfe78;
pub const Overlay2_Enable: Keysym = 0xfe79;
pub const AudibleBell_Enable: Keysym = 0xfe7a;

pub const Pointer_Left: Keysym = 0xfee0;
pub const Pointer_Right: Keysym = 0xfee1;
pub const Pointer_Up: Keysym = 0xfee2;
pub const Pointer_Down: Keysym = 0xfee3;
pub const Pointer_UpLeft: Keysym = 0xfee4;
pub const Pointer_UpRight: Keysym = 0xfee5;
pub const Pointer_DownLeft: Keysym = 0xfee6;
pub const Pointer_DownRight: Keysym = 0xfee7;
pub const Pointer_Button_Dflt: Keysym = 0xfee8;
pub const Pointer_Button1: Keysym = 0xfee9;
pub const Pointer_Button2: Keysym = 0xfeea;
pub const Pointer_Button3: Keysym = 0xfeeb;
pub const Pointer_Button4: Keysym = 0xfeec;
pub const Pointer_Button5: Keysym = 0xfeed;
pub const Pointer_DblClick_Dflt: Keysym = 0xfeee;
pub const Pointer_DblClick1: Keysym = 0xfeef;
pub const Pointer_DblClick2: Keysym = 0xfef0;
pub const Pointer_DblClick3: Keysym = 0xfef1;
pub const Pointer_DblClick4: Keysym = 0xfef2;
pub const Pointer_DblClick5: Keysym = 0xfef3;
pub const Pointer_Drag_Dflt: Keysym = 0xfef4;
pub const Pointer_Drag1: Keysym = 0xfef5;
pub const Pointer_Drag2: Keysym = 0xfef6;
pub const Pointer_Drag3: Keysym = 0xfef7;
pub const Pointer_Drag4: Keysym = 0xfef8;
pub const Pointer_Drag5: Keysym = 0xfefd;

pub const Pointer_EnableKeys: Keysym = 0xfef9;
pub const Pointer_Accelerate: Keysym = 0xfefa;
pub const Pointer_DfltBtnNext: Keysym = 0xfefb;
pub const Pointer_DfltBtnPrev: Keysym = 0xfefc;

/* Single-Stroke Multiple-Character N-Graph Keysyms For The X Input Method */

pub const ch: Keysym = 0xfea0;
pub const Ch: Keysym = 0xfea1;
pub const CH: Keysym = 0xfea2;
pub const c_h: Keysym = 0xfea3;
pub const C_h: Keysym = 0xfea4;
pub const C_H: Keysym = 0xfea5;

/*
 * 3270 Terminal Keys
 * Byte 3 = 0xfd;
 */

pub const KEY_3270_Duplicate: Keysym = 0xfd01;
pub const KEY_3270_FieldMark: Keysym = 0xfd02;
pub const KEY_3270_Right2: Keysym = 0xfd03;
pub const KEY_3270_Left2: Keysym = 0xfd04;
pub const KEY_3270_BackTab: Keysym = 0xfd05;
pub const KEY_3270_EraseEOF: Keysym = 0xfd06;
pub const KEY_3270_EraseInput: Keysym = 0xfd07;
pub const KEY_3270_Reset: Keysym = 0xfd08;
pub const KEY_3270_Quit: Keysym = 0xfd09;
pub const KEY_3270_PA1: Keysym = 0xfd0a;
pub const KEY_3270_PA2: Keysym = 0xfd0b;
pub const KEY_3270_PA3: Keysym = 0xfd0c;
pub const KEY_3270_Test: Keysym = 0xfd0d;
pub const KEY_3270_Attn: Keysym = 0xfd0e;
pub const KEY_3270_CursorBlink: Keysym = 0xfd0f;
pub const KEY_3270_AltCursor: Keysym = 0xfd10;
pub const KEY_3270_KeyClick: Keysym = 0xfd11;
pub const KEY_3270_Jump: Keysym = 0xfd12;
pub const KEY_3270_Ident: Keysym = 0xfd13;
pub const KEY_3270_Rule: Keysym = 0xfd14;
pub const KEY_3270_Copy: Keysym = 0xfd15;
pub const KEY_3270_Play: Keysym = 0xfd16;
pub const KEY_3270_Setup: Keysym = 0xfd17;
pub const KEY_3270_Record: Keysym = 0xfd18;
pub const KEY_3270_ChangeScreen: Keysym = 0xfd19;
pub const KEY_3270_DeleteWord: Keysym = 0xfd1a;
pub const KEY_3270_ExSelect: Keysym = 0xfd1b;
pub const KEY_3270_CursorSelect: Keysym = 0xfd1c;
pub const KEY_3270_PrintScreen: Keysym = 0xfd1d;
pub const KEY_3270_Enter: Keysym = 0xfd1e;

/*
 * Latin 1
 * (ISO/IEC 8859-1 = Unicode; U+0020..U+00FF)
 * Byte 3 = 0;
 */
pub const space: Keysym = 0x0020; /* U+0020 SPACE */
pub const exclam: Keysym = 0x0021; /* U+0021 EXCLAMATION MARK */
pub const quotedbl: Keysym = 0x0022; /* U+0022 QUOTATION MARK */
pub const numbersign: Keysym = 0x0023; /* U+0023 NUMBER SIGN */
pub const dollar: Keysym = 0x0024; /* U+0024 DOLLAR SIGN */
pub const percent: Keysym = 0x0025; /* U+0025 PERCENT SIGN */
pub const ampersand: Keysym = 0x0026; /* U+0026 AMPERSAND */
pub const apostrophe: Keysym = 0x0027; /* U+0027 APOSTROPHE */
pub const quoteright: Keysym = 0x0027; /* deprecated */
pub const parenleft: Keysym = 0x0028; /* U+0028 LEFT PARENTHESIS */
pub const parenright: Keysym = 0x0029; /* U+0029 RIGHT PARENTHESIS */
pub const asterisk: Keysym = 0x002a; /* U+002A ASTERISK */
pub const plus: Keysym = 0x002b; /* U+002B PLUS SIGN */
pub const comma: Keysym = 0x002c; /* U+002C COMMA */
pub const minus: Keysym = 0x002d; /* U+002D HYPHEN-MINUS */
pub const period: Keysym = 0x002e; /* U+002E FULL STOP */
pub const slash: Keysym = 0x002f; /* U+002F SOLIDUS */
pub const KEY_0: Keysym = 0x0030; /* U+0030 DIGIT ZERO */
pub const KEY_1: Keysym = 0x0031; /* U+0031 DIGIT ONE */
pub const KEY_2: Keysym = 0x0032; /* U+0032 DIGIT TWO */
pub const KEY_3: Keysym = 0x0033; /* U+0033 DIGIT THREE */
pub const KEY_4: Keysym = 0x0034; /* U+0034 DIGIT FOUR */
pub const KEY_5: Keysym = 0x0035; /* U+0035 DIGIT FIVE */
pub const KEY_6: Keysym = 0x0036; /* U+0036 DIGIT SIX */
pub const KEY_7: Keysym = 0x0037; /* U+0037 DIGIT SEVEN */
pub const KEY_8: Keysym = 0x0038; /* U+0038 DIGIT EIGHT */
pub const KEY_9: Keysym = 0x0039; /* U+0039 DIGIT NINE */
pub const colon: Keysym = 0x003a; /* U+003A COLON */
pub const semicolon: Keysym = 0x003b; /* U+003B SEMICOLON */
pub const less: Keysym = 0x003c; /* U+003C LESS-THAN SIGN */
pub const equal: Keysym = 0x003d; /* U+003D EQUALS SIGN */
pub const greater: Keysym = 0x003e; /* U+003E GREATER-THAN SIGN */
pub const question: Keysym = 0x003f; /* U+003F QUESTION MARK */
pub const at: Keysym = 0x0040; /* U+0040 COMMERCIAL AT */
pub const A: Keysym = 0x0041; /* U+0041 LATIN CAPITAL LETTER A */
pub const B: Keysym = 0x0042; /* U+0042 LATIN CAPITAL LETTER B */
pub const C: Keysym = 0x0043; /* U+0043 LATIN CAPITAL LETTER C */
pub const D: Keysym = 0x0044; /* U+0044 LATIN CAPITAL LETTER D */
pub const E: Keysym = 0x0045; /* U+0045 LATIN CAPITAL LETTER E */
pub const F: Keysym = 0x0046; /* U+0046 LATIN CAPITAL LETTER F */
pub const G: Keysym = 0x0047; /* U+0047 LATIN CAPITAL LETTER G */
pub const H: Keysym = 0x0048; /* U+0048 LATIN CAPITAL LETTER H */
pub const I: Keysym = 0x0049; /* U+0049 LATIN CAPITAL LETTER I */
pub const J: Keysym = 0x004a; /* U+004A LATIN CAPITAL LETTER J */
pub const K: Keysym = 0x004b; /* U+004B LATIN CAPITAL LETTER K */
pub const L: Keysym = 0x004c; /* U+004C LATIN CAPITAL LETTER L */
pub const M: Keysym = 0x004d; /* U+004D LATIN CAPITAL LETTER M */
pub const N: Keysym = 0x004e; /* U+004E LATIN CAPITAL LETTER N */
pub const O: Keysym = 0x004f; /* U+004F LATIN CAPITAL LETTER O */
pub const P: Keysym = 0x0050; /* U+0050 LATIN CAPITAL LETTER P */
pub const Q: Keysym = 0x0051; /* U+0051 LATIN CAPITAL LETTER Q */
pub const R: Keysym = 0x0052; /* U+0052 LATIN CAPITAL LETTER R */
pub const S: Keysym = 0x0053; /* U+0053 LATIN CAPITAL LETTER S */
pub const T: Keysym = 0x0054; /* U+0054 LATIN CAPITAL LETTER T */
pub const U: Keysym = 0x0055; /* U+0055 LATIN CAPITAL LETTER U */
pub const V: Keysym = 0x0056; /* U+0056 LATIN CAPITAL LETTER V */
pub const W: Keysym = 0x0057; /* U+0057 LATIN CAPITAL LETTER W */
pub const X: Keysym = 0x0058; /* U+0058 LATIN CAPITAL LETTER X */
pub const Y: Keysym = 0x0059; /* U+0059 LATIN CAPITAL LETTER Y */
pub const Z: Keysym = 0x005a; /* U+005A LATIN CAPITAL LETTER Z */
pub const bracketleft: Keysym = 0x005b; /* U+005B LEFT SQUARE BRACKET */
pub const backslash: Keysym = 0x005c; /* U+005C REVERSE SOLIDUS */
pub const bracketright: Keysym = 0x005d; /* U+005D RIGHT SQUARE BRACKET */
pub const asciicircum: Keysym = 0x005e; /* U+005E CIRCUMFLEX ACCENT */
pub const underscore: Keysym = 0x005f; /* U+005F LOW LINE */
pub const grave: Keysym = 0x0060; /* U+0060 GRAVE ACCENT */
pub const quoteleft: Keysym = 0x0060; /* deprecated */
pub const a: Keysym = 0x0061; /* U+0061 LATIN SMALL LETTER A */
pub const b: Keysym = 0x0062; /* U+0062 LATIN SMALL LETTER B */
pub const c: Keysym = 0x0063; /* U+0063 LATIN SMALL LETTER C */
pub const d: Keysym = 0x0064; /* U+0064 LATIN SMALL LETTER D */
pub const e: Keysym = 0x0065; /* U+0065 LATIN SMALL LETTER E */
pub const f: Keysym = 0x0066; /* U+0066 LATIN SMALL LETTER F */
pub const g: Keysym = 0x0067; /* U+0067 LATIN SMALL LETTER G */
pub const h: Keysym = 0x0068; /* U+0068 LATIN SMALL LETTER H */
pub const i: Keysym = 0x0069; /* U+0069 LATIN SMALL LETTER I */
pub const j: Keysym = 0x006a; /* U+006A LATIN SMALL LETTER J */
pub const k: Keysym = 0x006b; /* U+006B LATIN SMALL LETTER K */
pub const l: Keysym = 0x006c; /* U+006C LATIN SMALL LETTER L */
pub const m: Keysym = 0x006d; /* U+006D LATIN SMALL LETTER M */
pub const n: Keysym = 0x006e; /* U+006E LATIN SMALL LETTER N */
pub const o: Keysym = 0x006f; /* U+006F LATIN SMALL LETTER O */
pub const p: Keysym = 0x0070; /* U+0070 LATIN SMALL LETTER P */
pub const q: Keysym = 0x0071; /* U+0071 LATIN SMALL LETTER Q */
pub const r: Keysym = 0x0072; /* U+0072 LATIN SMALL LETTER R */
pub const s: Keysym = 0x0073; /* U+0073 LATIN SMALL LETTER S */
pub const t: Keysym = 0x0074; /* U+0074 LATIN SMALL LETTER T */
pub const u: Keysym = 0x0075; /* U+0075 LATIN SMALL LETTER U */
pub const v: Keysym = 0x0076; /* U+0076 LATIN SMALL LETTER V */
pub const w: Keysym = 0x0077; /* U+0077 LATIN SMALL LETTER W */
pub const x: Keysym = 0x0078; /* U+0078 LATIN SMALL LETTER X */
pub const y: Keysym = 0x0079; /* U+0079 LATIN SMALL LETTER Y */
pub const z: Keysym = 0x007a; /* U+007A LATIN SMALL LETTER Z */
pub const braceleft: Keysym = 0x007b; /* U+007B LEFT CURLY BRACKET */
pub const bar: Keysym = 0x007c; /* U+007C VERTICAL LINE */
pub const braceright: Keysym = 0x007d; /* U+007D RIGHT CURLY BRACKET */
pub const asciitilde: Keysym = 0x007e; /* U+007E TILDE */

pub const nobreakspace: Keysym = 0x00a0; /* U+00A0 NO-BREAK SPACE */
pub const exclamdown: Keysym = 0x00a1; /* U+00A1 INVERTED EXCLAMATION MARK */
pub const cent: Keysym = 0x00a2; /* U+00A2 CENT SIGN */
pub const sterling: Keysym = 0x00a3; /* U+00A3 POUND SIGN */
pub const currency: Keysym = 0x00a4; /* U+00A4 CURRENCY SIGN */
pub const yen: Keysym = 0x00a5; /* U+00A5 YEN SIGN */
pub const brokenbar: Keysym = 0x00a6; /* U+00A6 BROKEN BAR */
pub const section: Keysym = 0x00a7; /* U+00A7 SECTION SIGN */
pub const diaeresis: Keysym = 0x00a8; /* U+00A8 DIAERESIS */
pub const copyright: Keysym = 0x00a9; /* U+00A9 COPYRIGHT SIGN */
pub const ordfeminine: Keysym = 0x00aa; /* U+00AA FEMININE ORDINAL INDICATOR */
pub const guillemotleft: Keysym = 0x00ab; /* U+00AB LEFT-POINTING DOUBLE ANGLE QUOTATION MARK */
pub const notsign: Keysym = 0x00ac; /* U+00AC NOT SIGN */
pub const hyphen: Keysym = 0x00ad; /* U+00AD SOFT HYPHEN */
pub const registered: Keysym = 0x00ae; /* U+00AE REGISTERED SIGN */
pub const macron: Keysym = 0x00af; /* U+00AF MACRON */
pub const degree: Keysym = 0x00b0; /* U+00B0 DEGREE SIGN */
pub const plusminus: Keysym = 0x00b1; /* U+00B1 PLUS-MINUS SIGN */
pub const twosuperior: Keysym = 0x00b2; /* U+00B2 SUPERSCRIPT TWO */
pub const threesuperior: Keysym = 0x00b3; /* U+00B3 SUPERSCRIPT THREE */
pub const acute: Keysym = 0x00b4; /* U+00B4 ACUTE ACCENT */
pub const mu: Keysym = 0x00b5; /* U+00B5 MICRO SIGN */
pub const paragraph: Keysym = 0x00b6; /* U+00B6 PILCROW SIGN */
pub const periodcentered: Keysym = 0x00b7; /* U+00B7 MIDDLE DOT */
pub const cedilla: Keysym = 0x00b8; /* U+00B8 CEDILLA */
pub const onesuperior: Keysym = 0x00b9; /* U+00B9 SUPERSCRIPT ONE */
pub const masculine: Keysym = 0x00ba; /* U+00BA MASCULINE ORDINAL INDICATOR */
pub const guillemotright: Keysym = 0x00bb; /* U+00BB RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK */
pub const onequarter: Keysym = 0x00bc; /* U+00BC VULGAR FRACTION ONE QUARTER */
pub const onehalf: Keysym = 0x00bd; /* U+00BD VULGAR FRACTION ONE HALF */
pub const threequarters: Keysym = 0x00be; /* U+00BE VULGAR FRACTION THREE QUARTERS */
pub const questiondown: Keysym = 0x00bf; /* U+00BF INVERTED QUESTION MARK */
pub const Agrave: Keysym = 0x00c0; /* U+00C0 LATIN CAPITAL LETTER A WITH GRAVE */
pub const Aacute: Keysym = 0x00c1; /* U+00C1 LATIN CAPITAL LETTER A WITH ACUTE */
pub const Acircumflex: Keysym = 0x00c2; /* U+00C2 LATIN CAPITAL LETTER A WITH CIRCUMFLEX */
pub const Atilde: Keysym = 0x00c3; /* U+00C3 LATIN CAPITAL LETTER A WITH TILDE */
pub const Adiaeresis: Keysym = 0x00c4; /* U+00C4 LATIN CAPITAL LETTER A WITH DIAERESIS */
pub const Aring: Keysym = 0x00c5; /* U+00C5 LATIN CAPITAL LETTER A WITH RING ABOVE */
pub const AE: Keysym = 0x00c6; /* U+00C6 LATIN CAPITAL LETTER AE */
pub const Ccedilla: Keysym = 0x00c7; /* U+00C7 LATIN CAPITAL LETTER C WITH CEDILLA */
pub const Egrave: Keysym = 0x00c8; /* U+00C8 LATIN CAPITAL LETTER E WITH GRAVE */
pub const Eacute: Keysym = 0x00c9; /* U+00C9 LATIN CAPITAL LETTER E WITH ACUTE */
pub const Ecircumflex: Keysym = 0x00ca; /* U+00CA LATIN CAPITAL LETTER E WITH CIRCUMFLEX */
pub const Ediaeresis: Keysym = 0x00cb; /* U+00CB LATIN CAPITAL LETTER E WITH DIAERESIS */
pub const Igrave: Keysym = 0x00cc; /* U+00CC LATIN CAPITAL LETTER I WITH GRAVE */
pub const Iacute: Keysym = 0x00cd; /* U+00CD LATIN CAPITAL LETTER I WITH ACUTE */
pub const Icircumflex: Keysym = 0x00ce; /* U+00CE LATIN CAPITAL LETTER I WITH CIRCUMFLEX */
pub const Idiaeresis: Keysym = 0x00cf; /* U+00CF LATIN CAPITAL LETTER I WITH DIAERESIS */
pub const ETH: Keysym = 0x00d0; /* U+00D0 LATIN CAPITAL LETTER ETH */
pub const Eth: Keysym = 0x00d0; /* deprecated */
pub const Ntilde: Keysym = 0x00d1; /* U+00D1 LATIN CAPITAL LETTER N WITH TILDE */
pub const Ograve: Keysym = 0x00d2; /* U+00D2 LATIN CAPITAL LETTER O WITH GRAVE */
pub const Oacute: Keysym = 0x00d3; /* U+00D3 LATIN CAPITAL LETTER O WITH ACUTE */
pub const Ocircumflex: Keysym = 0x00d4; /* U+00D4 LATIN CAPITAL LETTER O WITH CIRCUMFLEX */
pub const Otilde: Keysym = 0x00d5; /* U+00D5 LATIN CAPITAL LETTER O WITH TILDE */
pub const Odiaeresis: Keysym = 0x00d6; /* U+00D6 LATIN CAPITAL LETTER O WITH DIAERESIS */
pub const multiply: Keysym = 0x00d7; /* U+00D7 MULTIPLICATION SIGN */
pub const Oslash: Keysym = 0x00d8; /* U+00D8 LATIN CAPITAL LETTER O WITH STROKE */
pub const Ooblique: Keysym = 0x00d8; /* U+00D8 LATIN CAPITAL LETTER O WITH STROKE */
pub const Ugrave: Keysym = 0x00d9; /* U+00D9 LATIN CAPITAL LETTER U WITH GRAVE */
pub const Uacute: Keysym = 0x00da; /* U+00DA LATIN CAPITAL LETTER U WITH ACUTE */
pub const Ucircumflex: Keysym = 0x00db; /* U+00DB LATIN CAPITAL LETTER U WITH CIRCUMFLEX */
pub const Udiaeresis: Keysym = 0x00dc; /* U+00DC LATIN CAPITAL LETTER U WITH DIAERESIS */
pub const Yacute: Keysym = 0x00dd; /* U+00DD LATIN CAPITAL LETTER Y WITH ACUTE */
pub const THORN: Keysym = 0x00de; /* U+00DE LATIN CAPITAL LETTER THORN */
pub const Thorn: Keysym = 0x00de; /* deprecated */
pub const ssharp: Keysym = 0x00df; /* U+00DF LATIN SMALL LETTER SHARP S */
pub const agrave: Keysym = 0x00e0; /* U+00E0 LATIN SMALL LETTER A WITH GRAVE */
pub const aacute: Keysym = 0x00e1; /* U+00E1 LATIN SMALL LETTER A WITH ACUTE */
pub const acircumflex: Keysym = 0x00e2; /* U+00E2 LATIN SMALL LETTER A WITH CIRCUMFLEX */
pub const atilde: Keysym = 0x00e3; /* U+00E3 LATIN SMALL LETTER A WITH TILDE */
pub const adiaeresis: Keysym = 0x00e4; /* U+00E4 LATIN SMALL LETTER A WITH DIAERESIS */
pub const aring: Keysym = 0x00e5; /* U+00E5 LATIN SMALL LETTER A WITH RING ABOVE */
pub const ae: Keysym = 0x00e6; /* U+00E6 LATIN SMALL LETTER AE */
pub const ccedilla: Keysym = 0x00e7; /* U+00E7 LATIN SMALL LETTER C WITH CEDILLA */
pub const egrave: Keysym = 0x00e8; /* U+00E8 LATIN SMALL LETTER E WITH GRAVE */
pub const eacute: Keysym = 0x00e9; /* U+00E9 LATIN SMALL LETTER E WITH ACUTE */
pub const ecircumflex: Keysym = 0x00ea; /* U+00EA LATIN SMALL LETTER E WITH CIRCUMFLEX */
pub const ediaeresis: Keysym = 0x00eb; /* U+00EB LATIN SMALL LETTER E WITH DIAERESIS */
pub const igrave: Keysym = 0x00ec; /* U+00EC LATIN SMALL LETTER I WITH GRAVE */
pub const iacute: Keysym = 0x00ed; /* U+00ED LATIN SMALL LETTER I WITH ACUTE */
pub const icircumflex: Keysym = 0x00ee; /* U+00EE LATIN SMALL LETTER I WITH CIRCUMFLEX */
pub const idiaeresis: Keysym = 0x00ef; /* U+00EF LATIN SMALL LETTER I WITH DIAERESIS */
pub const eth: Keysym = 0x00f0; /* U+00F0 LATIN SMALL LETTER ETH */
pub const ntilde: Keysym = 0x00f1; /* U+00F1 LATIN SMALL LETTER N WITH TILDE */
pub const ograve: Keysym = 0x00f2; /* U+00F2 LATIN SMALL LETTER O WITH GRAVE */
pub const oacute: Keysym = 0x00f3; /* U+00F3 LATIN SMALL LETTER O WITH ACUTE */
pub const ocircumflex: Keysym = 0x00f4; /* U+00F4 LATIN SMALL LETTER O WITH CIRCUMFLEX */
pub const otilde: Keysym = 0x00f5; /* U+00F5 LATIN SMALL LETTER O WITH TILDE */
pub const odiaeresis: Keysym = 0x00f6; /* U+00F6 LATIN SMALL LETTER O WITH DIAERESIS */
pub const division: Keysym = 0x00f7; /* U+00F7 DIVISION SIGN */
pub const oslash: Keysym = 0x00f8; /* U+00F8 LATIN SMALL LETTER O WITH STROKE */
pub const ooblique: Keysym = 0x00f8; /* U+00F8 LATIN SMALL LETTER O WITH STROKE */
pub const ugrave: Keysym = 0x00f9; /* U+00F9 LATIN SMALL LETTER U WITH GRAVE */
pub const uacute: Keysym = 0x00fa; /* U+00FA LATIN SMALL LETTER U WITH ACUTE */
pub const ucircumflex: Keysym = 0x00fb; /* U+00FB LATIN SMALL LETTER U WITH CIRCUMFLEX */
pub const udiaeresis: Keysym = 0x00fc; /* U+00FC LATIN SMALL LETTER U WITH DIAERESIS */
pub const yacute: Keysym = 0x00fd; /* U+00FD LATIN SMALL LETTER Y WITH ACUTE */
pub const thorn: Keysym = 0x00fe; /* U+00FE LATIN SMALL LETTER THORN */
pub const ydiaeresis: Keysym = 0x00ff; /* U+00FF LATIN SMALL LETTER Y WITH DIAERESIS */

/*
 * Latin 2
 * Byte 3 = 1;
 */

pub const Aogonek: Keysym = 0x01a1; /* U+0104 LATIN CAPITAL LETTER A WITH OGONEK */
pub const breve: Keysym = 0x01a2; /* U+02D8 BREVE */
pub const Lstroke: Keysym = 0x01a3; /* U+0141 LATIN CAPITAL LETTER L WITH STROKE */
pub const Lcaron: Keysym = 0x01a5; /* U+013D LATIN CAPITAL LETTER L WITH CARON */
pub const Sacute: Keysym = 0x01a6; /* U+015A LATIN CAPITAL LETTER S WITH ACUTE */
pub const Scaron: Keysym = 0x01a9; /* U+0160 LATIN CAPITAL LETTER S WITH CARON */
pub const Scedilla: Keysym = 0x01aa; /* U+015E LATIN CAPITAL LETTER S WITH CEDILLA */
pub const Tcaron: Keysym = 0x01ab; /* U+0164 LATIN CAPITAL LETTER T WITH CARON */
pub const Zacute: Keysym = 0x01ac; /* U+0179 LATIN CAPITAL LETTER Z WITH ACUTE */
pub const Zcaron: Keysym = 0x01ae; /* U+017D LATIN CAPITAL LETTER Z WITH CARON */
pub const Zabovedot: Keysym = 0x01af; /* U+017B LATIN CAPITAL LETTER Z WITH DOT ABOVE */
pub const aogonek: Keysym = 0x01b1; /* U+0105 LATIN SMALL LETTER A WITH OGONEK */
pub const ogonek: Keysym = 0x01b2; /* U+02DB OGONEK */
pub const lstroke: Keysym = 0x01b3; /* U+0142 LATIN SMALL LETTER L WITH STROKE */
pub const lcaron: Keysym = 0x01b5; /* U+013E LATIN SMALL LETTER L WITH CARON */
pub const sacute: Keysym = 0x01b6; /* U+015B LATIN SMALL LETTER S WITH ACUTE */
pub const caron: Keysym = 0x01b7; /* U+02C7 CARON */
pub const scaron: Keysym = 0x01b9; /* U+0161 LATIN SMALL LETTER S WITH CARON */
pub const scedilla: Keysym = 0x01ba; /* U+015F LATIN SMALL LETTER S WITH CEDILLA */
pub const tcaron: Keysym = 0x01bb; /* U+0165 LATIN SMALL LETTER T WITH CARON */
pub const zacute: Keysym = 0x01bc; /* U+017A LATIN SMALL LETTER Z WITH ACUTE */
pub const doubleacute: Keysym = 0x01bd; /* U+02DD DOUBLE ACUTE ACCENT */
pub const zcaron: Keysym = 0x01be; /* U+017E LATIN SMALL LETTER Z WITH CARON */
pub const zabovedot: Keysym = 0x01bf; /* U+017C LATIN SMALL LETTER Z WITH DOT ABOVE */
pub const Racute: Keysym = 0x01c0; /* U+0154 LATIN CAPITAL LETTER R WITH ACUTE */
pub const Abreve: Keysym = 0x01c3; /* U+0102 LATIN CAPITAL LETTER A WITH BREVE */
pub const Lacute: Keysym = 0x01c5; /* U+0139 LATIN CAPITAL LETTER L WITH ACUTE */
pub const Cacute: Keysym = 0x01c6; /* U+0106 LATIN CAPITAL LETTER C WITH ACUTE */
pub const Ccaron: Keysym = 0x01c8; /* U+010C LATIN CAPITAL LETTER C WITH CARON */
pub const Eogonek: Keysym = 0x01ca; /* U+0118 LATIN CAPITAL LETTER E WITH OGONEK */
pub const Ecaron: Keysym = 0x01cc; /* U+011A LATIN CAPITAL LETTER E WITH CARON */
pub const Dcaron: Keysym = 0x01cf; /* U+010E LATIN CAPITAL LETTER D WITH CARON */
pub const Dstroke: Keysym = 0x01d0; /* U+0110 LATIN CAPITAL LETTER D WITH STROKE */
pub const Nacute: Keysym = 0x01d1; /* U+0143 LATIN CAPITAL LETTER N WITH ACUTE */
pub const Ncaron: Keysym = 0x01d2; /* U+0147 LATIN CAPITAL LETTER N WITH CARON */
pub const Odoubleacute: Keysym = 0x01d5; /* U+0150 LATIN CAPITAL LETTER O WITH DOUBLE ACUTE */
pub const Rcaron: Keysym = 0x01d8; /* U+0158 LATIN CAPITAL LETTER R WITH CARON */
pub const Uring: Keysym = 0x01d9; /* U+016E LATIN CAPITAL LETTER U WITH RING ABOVE */
pub const Udoubleacute: Keysym = 0x01db; /* U+0170 LATIN CAPITAL LETTER U WITH DOUBLE ACUTE */
pub const Tcedilla: Keysym = 0x01de; /* U+0162 LATIN CAPITAL LETTER T WITH CEDILLA */
pub const racute: Keysym = 0x01e0; /* U+0155 LATIN SMALL LETTER R WITH ACUTE */
pub const abreve: Keysym = 0x01e3; /* U+0103 LATIN SMALL LETTER A WITH BREVE */
pub const lacute: Keysym = 0x01e5; /* U+013A LATIN SMALL LETTER L WITH ACUTE */
pub const cacute: Keysym = 0x01e6; /* U+0107 LATIN SMALL LETTER C WITH ACUTE */
pub const ccaron: Keysym = 0x01e8; /* U+010D LATIN SMALL LETTER C WITH CARON */
pub const eogonek: Keysym = 0x01ea; /* U+0119 LATIN SMALL LETTER E WITH OGONEK */
pub const ecaron: Keysym = 0x01ec; /* U+011B LATIN SMALL LETTER E WITH CARON */
pub const dcaron: Keysym = 0x01ef; /* U+010F LATIN SMALL LETTER D WITH CARON */
pub const dstroke: Keysym = 0x01f0; /* U+0111 LATIN SMALL LETTER D WITH STROKE */
pub const nacute: Keysym = 0x01f1; /* U+0144 LATIN SMALL LETTER N WITH ACUTE */
pub const ncaron: Keysym = 0x01f2; /* U+0148 LATIN SMALL LETTER N WITH CARON */
pub const odoubleacute: Keysym = 0x01f5; /* U+0151 LATIN SMALL LETTER O WITH DOUBLE ACUTE */
pub const rcaron: Keysym = 0x01f8; /* U+0159 LATIN SMALL LETTER R WITH CARON */
pub const uring: Keysym = 0x01f9; /* U+016F LATIN SMALL LETTER U WITH RING ABOVE */
pub const udoubleacute: Keysym = 0x01fb; /* U+0171 LATIN SMALL LETTER U WITH DOUBLE ACUTE */
pub const tcedilla: Keysym = 0x01fe; /* U+0163 LATIN SMALL LETTER T WITH CEDILLA */
pub const abovedot: Keysym = 0x01ff; /* U+02D9 DOT ABOVE */

/*
 * Latin 3
 * Byte 3 = 2;
 */

pub const Hstroke: Keysym = 0x02a1; /* U+0126 LATIN CAPITAL LETTER H WITH STROKE */
pub const Hcircumflex: Keysym = 0x02a6; /* U+0124 LATIN CAPITAL LETTER H WITH CIRCUMFLEX */
pub const Iabovedot: Keysym = 0x02a9; /* U+0130 LATIN CAPITAL LETTER I WITH DOT ABOVE */
pub const Gbreve: Keysym = 0x02ab; /* U+011E LATIN CAPITAL LETTER G WITH BREVE */
pub const Jcircumflex: Keysym = 0x02ac; /* U+0134 LATIN CAPITAL LETTER J WITH CIRCUMFLEX */
pub const hstroke: Keysym = 0x02b1; /* U+0127 LATIN SMALL LETTER H WITH STROKE */
pub const hcircumflex: Keysym = 0x02b6; /* U+0125 LATIN SMALL LETTER H WITH CIRCUMFLEX */
pub const idotless: Keysym = 0x02b9; /* U+0131 LATIN SMALL LETTER DOTLESS I */
pub const gbreve: Keysym = 0x02bb; /* U+011F LATIN SMALL LETTER G WITH BREVE */
pub const jcircumflex: Keysym = 0x02bc; /* U+0135 LATIN SMALL LETTER J WITH CIRCUMFLEX */
pub const Cabovedot: Keysym = 0x02c5; /* U+010A LATIN CAPITAL LETTER C WITH DOT ABOVE */
pub const Ccircumflex: Keysym = 0x02c6; /* U+0108 LATIN CAPITAL LETTER C WITH CIRCUMFLEX */
pub const Gabovedot: Keysym = 0x02d5; /* U+0120 LATIN CAPITAL LETTER G WITH DOT ABOVE */
pub const Gcircumflex: Keysym = 0x02d8; /* U+011C LATIN CAPITAL LETTER G WITH CIRCUMFLEX */
pub const Ubreve: Keysym = 0x02dd; /* U+016C LATIN CAPITAL LETTER U WITH BREVE */
pub const Scircumflex: Keysym = 0x02de; /* U+015C LATIN CAPITAL LETTER S WITH CIRCUMFLEX */
pub const cabovedot: Keysym = 0x02e5; /* U+010B LATIN SMALL LETTER C WITH DOT ABOVE */
pub const ccircumflex: Keysym = 0x02e6; /* U+0109 LATIN SMALL LETTER C WITH CIRCUMFLEX */
pub const gabovedot: Keysym = 0x02f5; /* U+0121 LATIN SMALL LETTER G WITH DOT ABOVE */
pub const gcircumflex: Keysym = 0x02f8; /* U+011D LATIN SMALL LETTER G WITH CIRCUMFLEX */
pub const ubreve: Keysym = 0x02fd; /* U+016D LATIN SMALL LETTER U WITH BREVE */
pub const scircumflex: Keysym = 0x02fe; /* U+015D LATIN SMALL LETTER S WITH CIRCUMFLEX */

/*
 * Latin 4
 * Byte 3 = 3;
 */

pub const kra: Keysym = 0x03a2; /* U+0138 LATIN SMALL LETTER KRA */
pub const kappa: Keysym = 0x03a2; /* deprecated */
pub const Rcedilla: Keysym = 0x03a3; /* U+0156 LATIN CAPITAL LETTER R WITH CEDILLA */
pub const Itilde: Keysym = 0x03a5; /* U+0128 LATIN CAPITAL LETTER I WITH TILDE */
pub const Lcedilla: Keysym = 0x03a6; /* U+013B LATIN CAPITAL LETTER L WITH CEDILLA */
pub const Emacron: Keysym = 0x03aa; /* U+0112 LATIN CAPITAL LETTER E WITH MACRON */
pub const Gcedilla: Keysym = 0x03ab; /* U+0122 LATIN CAPITAL LETTER G WITH CEDILLA */
pub const Tslash: Keysym = 0x03ac; /* U+0166 LATIN CAPITAL LETTER T WITH STROKE */
pub const rcedilla: Keysym = 0x03b3; /* U+0157 LATIN SMALL LETTER R WITH CEDILLA */
pub const itilde: Keysym = 0x03b5; /* U+0129 LATIN SMALL LETTER I WITH TILDE */
pub const lcedilla: Keysym = 0x03b6; /* U+013C LATIN SMALL LETTER L WITH CEDILLA */
pub const emacron: Keysym = 0x03ba; /* U+0113 LATIN SMALL LETTER E WITH MACRON */
pub const gcedilla: Keysym = 0x03bb; /* U+0123 LATIN SMALL LETTER G WITH CEDILLA */
pub const tslash: Keysym = 0x03bc; /* U+0167 LATIN SMALL LETTER T WITH STROKE */
pub const ENG: Keysym = 0x03bd; /* U+014A LATIN CAPITAL LETTER ENG */
pub const eng: Keysym = 0x03bf; /* U+014B LATIN SMALL LETTER ENG */
pub const Amacron: Keysym = 0x03c0; /* U+0100 LATIN CAPITAL LETTER A WITH MACRON */
pub const Iogonek: Keysym = 0x03c7; /* U+012E LATIN CAPITAL LETTER I WITH OGONEK */
pub const Eabovedot: Keysym = 0x03cc; /* U+0116 LATIN CAPITAL LETTER E WITH DOT ABOVE */
pub const Imacron: Keysym = 0x03cf; /* U+012A LATIN CAPITAL LETTER I WITH MACRON */
pub const Ncedilla: Keysym = 0x03d1; /* U+0145 LATIN CAPITAL LETTER N WITH CEDILLA */
pub const Omacron: Keysym = 0x03d2; /* U+014C LATIN CAPITAL LETTER O WITH MACRON */
pub const Kcedilla: Keysym = 0x03d3; /* U+0136 LATIN CAPITAL LETTER K WITH CEDILLA */
pub const Uogonek: Keysym = 0x03d9; /* U+0172 LATIN CAPITAL LETTER U WITH OGONEK */
pub const Utilde: Keysym = 0x03dd; /* U+0168 LATIN CAPITAL LETTER U WITH TILDE */
pub const Umacron: Keysym = 0x03de; /* U+016A LATIN CAPITAL LETTER U WITH MACRON */
pub const amacron: Keysym = 0x03e0; /* U+0101 LATIN SMALL LETTER A WITH MACRON */
pub const iogonek: Keysym = 0x03e7; /* U+012F LATIN SMALL LETTER I WITH OGONEK */
pub const eabovedot: Keysym = 0x03ec; /* U+0117 LATIN SMALL LETTER E WITH DOT ABOVE */
pub const imacron: Keysym = 0x03ef; /* U+012B LATIN SMALL LETTER I WITH MACRON */
pub const ncedilla: Keysym = 0x03f1; /* U+0146 LATIN SMALL LETTER N WITH CEDILLA */
pub const omacron: Keysym = 0x03f2; /* U+014D LATIN SMALL LETTER O WITH MACRON */
pub const kcedilla: Keysym = 0x03f3; /* U+0137 LATIN SMALL LETTER K WITH CEDILLA */
pub const uogonek: Keysym = 0x03f9; /* U+0173 LATIN SMALL LETTER U WITH OGONEK */
pub const utilde: Keysym = 0x03fd; /* U+0169 LATIN SMALL LETTER U WITH TILDE */
pub const umacron: Keysym = 0x03fe; /* U+016B LATIN SMALL LETTER U WITH MACRON */

/*
 * Latin 8
 */
pub const Wcircumflex: Keysym = 0x1000174; /* U+0174 LATIN CAPITAL LETTER W WITH CIRCUMFLEX */
pub const wcircumflex: Keysym = 0x1000175; /* U+0175 LATIN SMALL LETTER W WITH CIRCUMFLEX */
pub const Ycircumflex: Keysym = 0x1000176; /* U+0176 LATIN CAPITAL LETTER Y WITH CIRCUMFLEX */
pub const ycircumflex: Keysym = 0x1000177; /* U+0177 LATIN SMALL LETTER Y WITH CIRCUMFLEX */
pub const Babovedot: Keysym = 0x1001e02; /* U+1E02 LATIN CAPITAL LETTER B WITH DOT ABOVE */
pub const babovedot: Keysym = 0x1001e03; /* U+1E03 LATIN SMALL LETTER B WITH DOT ABOVE */
pub const Dabovedot: Keysym = 0x1001e0a; /* U+1E0A LATIN CAPITAL LETTER D WITH DOT ABOVE */
pub const dabovedot: Keysym = 0x1001e0b; /* U+1E0B LATIN SMALL LETTER D WITH DOT ABOVE */
pub const Fabovedot: Keysym = 0x1001e1e; /* U+1E1E LATIN CAPITAL LETTER F WITH DOT ABOVE */
pub const fabovedot: Keysym = 0x1001e1f; /* U+1E1F LATIN SMALL LETTER F WITH DOT ABOVE */
pub const Mabovedot: Keysym = 0x1001e40; /* U+1E40 LATIN CAPITAL LETTER M WITH DOT ABOVE */
pub const mabovedot: Keysym = 0x1001e41; /* U+1E41 LATIN SMALL LETTER M WITH DOT ABOVE */
pub const Pabovedot: Keysym = 0x1001e56; /* U+1E56 LATIN CAPITAL LETTER P WITH DOT ABOVE */
pub const pabovedot: Keysym = 0x1001e57; /* U+1E57 LATIN SMALL LETTER P WITH DOT ABOVE */
pub const Sabovedot: Keysym = 0x1001e60; /* U+1E60 LATIN CAPITAL LETTER S WITH DOT ABOVE */
pub const sabovedot: Keysym = 0x1001e61; /* U+1E61 LATIN SMALL LETTER S WITH DOT ABOVE */
pub const Tabovedot: Keysym = 0x1001e6a; /* U+1E6A LATIN CAPITAL LETTER T WITH DOT ABOVE */
pub const tabovedot: Keysym = 0x1001e6b; /* U+1E6B LATIN SMALL LETTER T WITH DOT ABOVE */
pub const Wgrave: Keysym = 0x1001e80; /* U+1E80 LATIN CAPITAL LETTER W WITH GRAVE */
pub const wgrave: Keysym = 0x1001e81; /* U+1E81 LATIN SMALL LETTER W WITH GRAVE */
pub const Wacute: Keysym = 0x1001e82; /* U+1E82 LATIN CAPITAL LETTER W WITH ACUTE */
pub const wacute: Keysym = 0x1001e83; /* U+1E83 LATIN SMALL LETTER W WITH ACUTE */
pub const Wdiaeresis: Keysym = 0x1001e84; /* U+1E84 LATIN CAPITAL LETTER W WITH DIAERESIS */
pub const wdiaeresis: Keysym = 0x1001e85; /* U+1E85 LATIN SMALL LETTER W WITH DIAERESIS */
pub const Ygrave: Keysym = 0x1001ef2; /* U+1EF2 LATIN CAPITAL LETTER Y WITH GRAVE */
pub const ygrave: Keysym = 0x1001ef3; /* U+1EF3 LATIN SMALL LETTER Y WITH GRAVE */

/*
 * Latin 9
 * Byte 3 = 0x13;
 */

pub const OE: Keysym = 0x13bc; /* U+0152 LATIN CAPITAL LIGATURE OE */
pub const oe: Keysym = 0x13bd; /* U+0153 LATIN SMALL LIGATURE OE */
pub const Ydiaeresis: Keysym = 0x13be; /* U+0178 LATIN CAPITAL LETTER Y WITH DIAERESIS */

/*
 * Katakana
 * Byte 3 = 4;
 */

pub const overline: Keysym = 0x047e; /* U+203E OVERLINE */
pub const kana_fullstop: Keysym = 0x04a1; /* U+3002 IDEOGRAPHIC FULL STOP */
pub const kana_openingbracket: Keysym = 0x04a2; /* U+300C LEFT CORNER BRACKET */
pub const kana_closingbracket: Keysym = 0x04a3; /* U+300D RIGHT CORNER BRACKET */
pub const kana_comma: Keysym = 0x04a4; /* U+3001 IDEOGRAPHIC COMMA */
pub const kana_conjunctive: Keysym = 0x04a5; /* U+30FB KATAKANA MIDDLE DOT */
pub const kana_middledot: Keysym = 0x04a5; /* deprecated */
pub const kana_WO: Keysym = 0x04a6; /* U+30F2 KATAKANA LETTER WO */
pub const kana_a: Keysym = 0x04a7; /* U+30A1 KATAKANA LETTER SMALL A */
pub const kana_i: Keysym = 0x04a8; /* U+30A3 KATAKANA LETTER SMALL I */
pub const kana_u: Keysym = 0x04a9; /* U+30A5 KATAKANA LETTER SMALL U */
pub const kana_e: Keysym = 0x04aa; /* U+30A7 KATAKANA LETTER SMALL E */
pub const kana_o: Keysym = 0x04ab; /* U+30A9 KATAKANA LETTER SMALL O */
pub const kana_ya: Keysym = 0x04ac; /* U+30E3 KATAKANA LETTER SMALL YA */
pub const kana_yu: Keysym = 0x04ad; /* U+30E5 KATAKANA LETTER SMALL YU */
pub const kana_yo: Keysym = 0x04ae; /* U+30E7 KATAKANA LETTER SMALL YO */
pub const kana_tsu: Keysym = 0x04af; /* U+30C3 KATAKANA LETTER SMALL TU */
pub const kana_tu: Keysym = 0x04af; /* deprecated */
pub const prolongedsound: Keysym = 0x04b0; /* U+30FC KATAKANA-HIRAGANA PROLONGED SOUND MARK */
pub const kana_A: Keysym = 0x04b1; /* U+30A2 KATAKANA LETTER A */
pub const kana_I: Keysym = 0x04b2; /* U+30A4 KATAKANA LETTER I */
pub const kana_U: Keysym = 0x04b3; /* U+30A6 KATAKANA LETTER U */
pub const kana_E: Keysym = 0x04b4; /* U+30A8 KATAKANA LETTER E */
pub const kana_O: Keysym = 0x04b5; /* U+30AA KATAKANA LETTER O */
pub const kana_KA: Keysym = 0x04b6; /* U+30AB KATAKANA LETTER KA */
pub const kana_KI: Keysym = 0x04b7; /* U+30AD KATAKANA LETTER KI */
pub const kana_KU: Keysym = 0x04b8; /* U+30AF KATAKANA LETTER KU */
pub const kana_KE: Keysym = 0x04b9; /* U+30B1 KATAKANA LETTER KE */
pub const kana_KO: Keysym = 0x04ba; /* U+30B3 KATAKANA LETTER KO */
pub const kana_SA: Keysym = 0x04bb; /* U+30B5 KATAKANA LETTER SA */
pub const kana_SHI: Keysym = 0x04bc; /* U+30B7 KATAKANA LETTER SI */
pub const kana_SU: Keysym = 0x04bd; /* U+30B9 KATAKANA LETTER SU */
pub const kana_SE: Keysym = 0x04be; /* U+30BB KATAKANA LETTER SE */
pub const kana_SO: Keysym = 0x04bf; /* U+30BD KATAKANA LETTER SO */
pub const kana_TA: Keysym = 0x04c0; /* U+30BF KATAKANA LETTER TA */
pub const kana_CHI: Keysym = 0x04c1; /* U+30C1 KATAKANA LETTER TI */
pub const kana_TI: Keysym = 0x04c1; /* deprecated */
pub const kana_TSU: Keysym = 0x04c2; /* U+30C4 KATAKANA LETTER TU */
pub const kana_TU: Keysym = 0x04c2; /* deprecated */
pub const kana_TE: Keysym = 0x04c3; /* U+30C6 KATAKANA LETTER TE */
pub const kana_TO: Keysym = 0x04c4; /* U+30C8 KATAKANA LETTER TO */
pub const kana_NA: Keysym = 0x04c5; /* U+30CA KATAKANA LETTER NA */
pub const kana_NI: Keysym = 0x04c6; /* U+30CB KATAKANA LETTER NI */
pub const kana_NU: Keysym = 0x04c7; /* U+30CC KATAKANA LETTER NU */
pub const kana_NE: Keysym = 0x04c8; /* U+30CD KATAKANA LETTER NE */
pub const kana_NO: Keysym = 0x04c9; /* U+30CE KATAKANA LETTER NO */
pub const kana_HA: Keysym = 0x04ca; /* U+30CF KATAKANA LETTER HA */
pub const kana_HI: Keysym = 0x04cb; /* U+30D2 KATAKANA LETTER HI */
pub const kana_FU: Keysym = 0x04cc; /* U+30D5 KATAKANA LETTER HU */
pub const kana_HU: Keysym = 0x04cc; /* deprecated */
pub const kana_HE: Keysym = 0x04cd; /* U+30D8 KATAKANA LETTER HE */
pub const kana_HO: Keysym = 0x04ce; /* U+30DB KATAKANA LETTER HO */
pub const kana_MA: Keysym = 0x04cf; /* U+30DE KATAKANA LETTER MA */
pub const kana_MI: Keysym = 0x04d0; /* U+30DF KATAKANA LETTER MI */
pub const kana_MU: Keysym = 0x04d1; /* U+30E0 KATAKANA LETTER MU */
pub const kana_ME: Keysym = 0x04d2; /* U+30E1 KATAKANA LETTER ME */
pub const kana_MO: Keysym = 0x04d3; /* U+30E2 KATAKANA LETTER MO */
pub const kana_YA: Keysym = 0x04d4; /* U+30E4 KATAKANA LETTER YA */
pub const kana_YU: Keysym = 0x04d5; /* U+30E6 KATAKANA LETTER YU */
pub const kana_YO: Keysym = 0x04d6; /* U+30E8 KATAKANA LETTER YO */
pub const kana_RA: Keysym = 0x04d7; /* U+30E9 KATAKANA LETTER RA */
pub const kana_RI: Keysym = 0x04d8; /* U+30EA KATAKANA LETTER RI */
pub const kana_RU: Keysym = 0x04d9; /* U+30EB KATAKANA LETTER RU */
pub const kana_RE: Keysym = 0x04da; /* U+30EC KATAKANA LETTER RE */
pub const kana_RO: Keysym = 0x04db; /* U+30ED KATAKANA LETTER RO */
pub const kana_WA: Keysym = 0x04dc; /* U+30EF KATAKANA LETTER WA */
pub const kana_N: Keysym = 0x04dd; /* U+30F3 KATAKANA LETTER N */
pub const voicedsound: Keysym = 0x04de; /* U+309B KATAKANA-HIRAGANA VOICED SOUND MARK */
pub const semivoicedsound: Keysym = 0x04df; /* U+309C KATAKANA-HIRAGANA SEMI-VOICED SOUND MARK */
pub const kana_switch: Keysym = 0xff7e; /* Alias for mode_switch */

/*
 * Arabic
 * Byte 3 = 5;
 */

pub const Farsi_0: Keysym = 0x10006f0; /* U+06F0 EXTENDED ARABIC-INDIC DIGIT ZERO */
pub const Farsi_1: Keysym = 0x10006f1; /* U+06F1 EXTENDED ARABIC-INDIC DIGIT ONE */
pub const Farsi_2: Keysym = 0x10006f2; /* U+06F2 EXTENDED ARABIC-INDIC DIGIT TWO */
pub const Farsi_3: Keysym = 0x10006f3; /* U+06F3 EXTENDED ARABIC-INDIC DIGIT THREE */
pub const Farsi_4: Keysym = 0x10006f4; /* U+06F4 EXTENDED ARABIC-INDIC DIGIT FOUR */
pub const Farsi_5: Keysym = 0x10006f5; /* U+06F5 EXTENDED ARABIC-INDIC DIGIT FIVE */
pub const Farsi_6: Keysym = 0x10006f6; /* U+06F6 EXTENDED ARABIC-INDIC DIGIT SIX */
pub const Farsi_7: Keysym = 0x10006f7; /* U+06F7 EXTENDED ARABIC-INDIC DIGIT SEVEN */
pub const Farsi_8: Keysym = 0x10006f8; /* U+06F8 EXTENDED ARABIC-INDIC DIGIT EIGHT */
pub const Farsi_9: Keysym = 0x10006f9; /* U+06F9 EXTENDED ARABIC-INDIC DIGIT NINE */
pub const Arabic_percent: Keysym = 0x100066a; /* U+066A ARABIC PERCENT SIGN */
pub const Arabic_superscript_alef: Keysym = 0x1000670; /* U+0670 ARABIC LETTER SUPERSCRIPT ALEF */
pub const Arabic_tteh: Keysym = 0x1000679; /* U+0679 ARABIC LETTER TTEH */
pub const Arabic_peh: Keysym = 0x100067e; /* U+067E ARABIC LETTER PEH */
pub const Arabic_tcheh: Keysym = 0x1000686; /* U+0686 ARABIC LETTER TCHEH */
pub const Arabic_ddal: Keysym = 0x1000688; /* U+0688 ARABIC LETTER DDAL */
pub const Arabic_rreh: Keysym = 0x1000691; /* U+0691 ARABIC LETTER RREH */
pub const Arabic_comma: Keysym = 0x05ac; /* U+060C ARABIC COMMA */
pub const Arabic_fullstop: Keysym = 0x10006d4; /* U+06D4 ARABIC FULL STOP */
pub const Arabic_0: Keysym = 0x1000660; /* U+0660 ARABIC-INDIC DIGIT ZERO */
pub const Arabic_1: Keysym = 0x1000661; /* U+0661 ARABIC-INDIC DIGIT ONE */
pub const Arabic_2: Keysym = 0x1000662; /* U+0662 ARABIC-INDIC DIGIT TWO */
pub const Arabic_3: Keysym = 0x1000663; /* U+0663 ARABIC-INDIC DIGIT THREE */
pub const Arabic_4: Keysym = 0x1000664; /* U+0664 ARABIC-INDIC DIGIT FOUR */
pub const Arabic_5: Keysym = 0x1000665; /* U+0665 ARABIC-INDIC DIGIT FIVE */
pub const Arabic_6: Keysym = 0x1000666; /* U+0666 ARABIC-INDIC DIGIT SIX */
pub const Arabic_7: Keysym = 0x1000667; /* U+0667 ARABIC-INDIC DIGIT SEVEN */
pub const Arabic_8: Keysym = 0x1000668; /* U+0668 ARABIC-INDIC DIGIT EIGHT */
pub const Arabic_9: Keysym = 0x1000669; /* U+0669 ARABIC-INDIC DIGIT NINE */
pub const Arabic_semicolon: Keysym = 0x05bb; /* U+061B ARABIC SEMICOLON */
pub const Arabic_question_mark: Keysym = 0x05bf; /* U+061F ARABIC QUESTION MARK */
pub const Arabic_hamza: Keysym = 0x05c1; /* U+0621 ARABIC LETTER HAMZA */
pub const Arabic_maddaonalef: Keysym = 0x05c2; /* U+0622 ARABIC LETTER ALEF WITH MADDA ABOVE */
pub const Arabic_hamzaonalef: Keysym = 0x05c3; /* U+0623 ARABIC LETTER ALEF WITH HAMZA ABOVE */
pub const Arabic_hamzaonwaw: Keysym = 0x05c4; /* U+0624 ARABIC LETTER WAW WITH HAMZA ABOVE */
pub const Arabic_hamzaunderalef: Keysym = 0x05c5; /* U+0625 ARABIC LETTER ALEF WITH HAMZA BELOW */
pub const Arabic_hamzaonyeh: Keysym = 0x05c6; /* U+0626 ARABIC LETTER YEH WITH HAMZA ABOVE */
pub const Arabic_alef: Keysym = 0x05c7; /* U+0627 ARABIC LETTER ALEF */
pub const Arabic_beh: Keysym = 0x05c8; /* U+0628 ARABIC LETTER BEH */
pub const Arabic_tehmarbuta: Keysym = 0x05c9; /* U+0629 ARABIC LETTER TEH MARBUTA */
pub const Arabic_teh: Keysym = 0x05ca; /* U+062A ARABIC LETTER TEH */
pub const Arabic_theh: Keysym = 0x05cb; /* U+062B ARABIC LETTER THEH */
pub const Arabic_jeem: Keysym = 0x05cc; /* U+062C ARABIC LETTER JEEM */
pub const Arabic_hah: Keysym = 0x05cd; /* U+062D ARABIC LETTER HAH */
pub const Arabic_khah: Keysym = 0x05ce; /* U+062E ARABIC LETTER KHAH */
pub const Arabic_dal: Keysym = 0x05cf; /* U+062F ARABIC LETTER DAL */
pub const Arabic_thal: Keysym = 0x05d0; /* U+0630 ARABIC LETTER THAL */
pub const Arabic_ra: Keysym = 0x05d1; /* U+0631 ARABIC LETTER REH */
pub const Arabic_zain: Keysym = 0x05d2; /* U+0632 ARABIC LETTER ZAIN */
pub const Arabic_seen: Keysym = 0x05d3; /* U+0633 ARABIC LETTER SEEN */
pub const Arabic_sheen: Keysym = 0x05d4; /* U+0634 ARABIC LETTER SHEEN */
pub const Arabic_sad: Keysym = 0x05d5; /* U+0635 ARABIC LETTER SAD */
pub const Arabic_dad: Keysym = 0x05d6; /* U+0636 ARABIC LETTER DAD */
pub const Arabic_tah: Keysym = 0x05d7; /* U+0637 ARABIC LETTER TAH */
pub const Arabic_zah: Keysym = 0x05d8; /* U+0638 ARABIC LETTER ZAH */
pub const Arabic_ain: Keysym = 0x05d9; /* U+0639 ARABIC LETTER AIN */
pub const Arabic_ghain: Keysym = 0x05da; /* U+063A ARABIC LETTER GHAIN */
pub const Arabic_tatweel: Keysym = 0x05e0; /* U+0640 ARABIC TATWEEL */
pub const Arabic_feh: Keysym = 0x05e1; /* U+0641 ARABIC LETTER FEH */
pub const Arabic_qaf: Keysym = 0x05e2; /* U+0642 ARABIC LETTER QAF */
pub const Arabic_kaf: Keysym = 0x05e3; /* U+0643 ARABIC LETTER KAF */
pub const Arabic_lam: Keysym = 0x05e4; /* U+0644 ARABIC LETTER LAM */
pub const Arabic_meem: Keysym = 0x05e5; /* U+0645 ARABIC LETTER MEEM */
pub const Arabic_noon: Keysym = 0x05e6; /* U+0646 ARABIC LETTER NOON */
pub const Arabic_ha: Keysym = 0x05e7; /* U+0647 ARABIC LETTER HEH */
pub const Arabic_heh: Keysym = 0x05e7; /* deprecated */
pub const Arabic_waw: Keysym = 0x05e8; /* U+0648 ARABIC LETTER WAW */
pub const Arabic_alefmaksura: Keysym = 0x05e9; /* U+0649 ARABIC LETTER ALEF MAKSURA */
pub const Arabic_yeh: Keysym = 0x05ea; /* U+064A ARABIC LETTER YEH */
pub const Arabic_fathatan: Keysym = 0x05eb; /* U+064B ARABIC FATHATAN */
pub const Arabic_dammatan: Keysym = 0x05ec; /* U+064C ARABIC DAMMATAN */
pub const Arabic_kasratan: Keysym = 0x05ed; /* U+064D ARABIC KASRATAN */
pub const Arabic_fatha: Keysym = 0x05ee; /* U+064E ARABIC FATHA */
pub const Arabic_damma: Keysym = 0x05ef; /* U+064F ARABIC DAMMA */
pub const Arabic_kasra: Keysym = 0x05f0; /* U+0650 ARABIC KASRA */
pub const Arabic_shadda: Keysym = 0x05f1; /* U+0651 ARABIC SHADDA */
pub const Arabic_sukun: Keysym = 0x05f2; /* U+0652 ARABIC SUKUN */
pub const Arabic_madda_above: Keysym = 0x1000653; /* U+0653 ARABIC MADDAH ABOVE */
pub const Arabic_hamza_above: Keysym = 0x1000654; /* U+0654 ARABIC HAMZA ABOVE */
pub const Arabic_hamza_below: Keysym = 0x1000655; /* U+0655 ARABIC HAMZA BELOW */
pub const Arabic_jeh: Keysym = 0x1000698; /* U+0698 ARABIC LETTER JEH */
pub const Arabic_veh: Keysym = 0x10006a4; /* U+06A4 ARABIC LETTER VEH */
pub const Arabic_keheh: Keysym = 0x10006a9; /* U+06A9 ARABIC LETTER KEHEH */
pub const Arabic_gaf: Keysym = 0x10006af; /* U+06AF ARABIC LETTER GAF */
pub const Arabic_noon_ghunna: Keysym = 0x10006ba; /* U+06BA ARABIC LETTER NOON GHUNNA */
pub const Arabic_heh_doachashmee: Keysym = 0x10006be; /* U+06BE ARABIC LETTER HEH DOACHASHMEE */
pub const Farsi_yeh: Keysym = 0x10006cc; /* U+06CC ARABIC LETTER FARSI YEH */
pub const Arabic_farsi_yeh: Keysym = 0x10006cc; /* U+06CC ARABIC LETTER FARSI YEH */
pub const Arabic_yeh_baree: Keysym = 0x10006d2; /* U+06D2 ARABIC LETTER YEH BARREE */
pub const Arabic_heh_goal: Keysym = 0x10006c1; /* U+06C1 ARABIC LETTER HEH GOAL */
pub const Arabic_switch: Keysym = 0xff7e; /* Alias for mode_switch */

/*
 * Cyrillic
 * Byte 3 = 6;
 */
pub const Cyrillic_GHE_bar: Keysym = 0x1000492; /* U+0492 CYRILLIC CAPITAL LETTER GHE WITH STROKE */
pub const Cyrillic_ghe_bar: Keysym = 0x1000493; /* U+0493 CYRILLIC SMALL LETTER GHE WITH STROKE */
pub const Cyrillic_ZHE_descender: Keysym = 0x1000496; /* U+0496 CYRILLIC CAPITAL LETTER ZHE WITH DESCENDER */
pub const Cyrillic_zhe_descender: Keysym = 0x1000497; /* U+0497 CYRILLIC SMALL LETTER ZHE WITH DESCENDER */
pub const Cyrillic_KA_descender: Keysym = 0x100049a; /* U+049A CYRILLIC CAPITAL LETTER KA WITH DESCENDER */
pub const Cyrillic_ka_descender: Keysym = 0x100049b; /* U+049B CYRILLIC SMALL LETTER KA WITH DESCENDER */
pub const Cyrillic_KA_vertstroke: Keysym = 0x100049c; /* U+049C CYRILLIC CAPITAL LETTER KA WITH VERTICAL STROKE */
pub const Cyrillic_ka_vertstroke: Keysym = 0x100049d; /* U+049D CYRILLIC SMALL LETTER KA WITH VERTICAL STROKE */
pub const Cyrillic_EN_descender: Keysym = 0x10004a2; /* U+04A2 CYRILLIC CAPITAL LETTER EN WITH DESCENDER */
pub const Cyrillic_en_descender: Keysym = 0x10004a3; /* U+04A3 CYRILLIC SMALL LETTER EN WITH DESCENDER */
pub const Cyrillic_U_straight: Keysym = 0x10004ae; /* U+04AE CYRILLIC CAPITAL LETTER STRAIGHT U */
pub const Cyrillic_u_straight: Keysym = 0x10004af; /* U+04AF CYRILLIC SMALL LETTER STRAIGHT U */
pub const Cyrillic_U_straight_bar: Keysym = 0x10004b0; /* U+04B0 CYRILLIC CAPITAL LETTER STRAIGHT U WITH STROKE */
pub const Cyrillic_u_straight_bar: Keysym = 0x10004b1; /* U+04B1 CYRILLIC SMALL LETTER STRAIGHT U WITH STROKE */
pub const Cyrillic_HA_descender: Keysym = 0x10004b2; /* U+04B2 CYRILLIC CAPITAL LETTER HA WITH DESCENDER */
pub const Cyrillic_ha_descender: Keysym = 0x10004b3; /* U+04B3 CYRILLIC SMALL LETTER HA WITH DESCENDER */
pub const Cyrillic_CHE_descender: Keysym = 0x10004b6; /* U+04B6 CYRILLIC CAPITAL LETTER CHE WITH DESCENDER */
pub const Cyrillic_che_descender: Keysym = 0x10004b7; /* U+04B7 CYRILLIC SMALL LETTER CHE WITH DESCENDER */
pub const Cyrillic_CHE_vertstroke: Keysym = 0x10004b8; /* U+04B8 CYRILLIC CAPITAL LETTER CHE WITH VERTICAL STROKE */
pub const Cyrillic_che_vertstroke: Keysym = 0x10004b9; /* U+04B9 CYRILLIC SMALL LETTER CHE WITH VERTICAL STROKE */
pub const Cyrillic_SHHA: Keysym = 0x10004ba; /* U+04BA CYRILLIC CAPITAL LETTER SHHA */
pub const Cyrillic_shha: Keysym = 0x10004bb; /* U+04BB CYRILLIC SMALL LETTER SHHA */

pub const Cyrillic_SCHWA: Keysym = 0x10004d8; /* U+04D8 CYRILLIC CAPITAL LETTER SCHWA */
pub const Cyrillic_schwa: Keysym = 0x10004d9; /* U+04D9 CYRILLIC SMALL LETTER SCHWA */
pub const Cyrillic_I_macron: Keysym = 0x10004e2; /* U+04E2 CYRILLIC CAPITAL LETTER I WITH MACRON */
pub const Cyrillic_i_macron: Keysym = 0x10004e3; /* U+04E3 CYRILLIC SMALL LETTER I WITH MACRON */
pub const Cyrillic_O_bar: Keysym = 0x10004e8; /* U+04E8 CYRILLIC CAPITAL LETTER BARRED O */
pub const Cyrillic_o_bar: Keysym = 0x10004e9; /* U+04E9 CYRILLIC SMALL LETTER BARRED O */
pub const Cyrillic_U_macron: Keysym = 0x10004ee; /* U+04EE CYRILLIC CAPITAL LETTER U WITH MACRON */
pub const Cyrillic_u_macron: Keysym = 0x10004ef; /* U+04EF CYRILLIC SMALL LETTER U WITH MACRON */

pub const Serbian_dje: Keysym = 0x06a1; /* U+0452 CYRILLIC SMALL LETTER DJE */
pub const Macedonia_gje: Keysym = 0x06a2; /* U+0453 CYRILLIC SMALL LETTER GJE */
pub const Cyrillic_io: Keysym = 0x06a3; /* U+0451 CYRILLIC SMALL LETTER IO */
pub const Ukrainian_ie: Keysym = 0x06a4; /* U+0454 CYRILLIC SMALL LETTER UKRAINIAN IE */
pub const Ukranian_je: Keysym = 0x06a4; /* deprecated */
pub const Macedonia_dse: Keysym = 0x06a5; /* U+0455 CYRILLIC SMALL LETTER DZE */
pub const Ukrainian_i: Keysym = 0x06a6; /* U+0456 CYRILLIC SMALL LETTER BYELORUSSIAN-UKRAINIAN I */
pub const Ukranian_i: Keysym = 0x06a6; /* deprecated */
pub const Ukrainian_yi: Keysym = 0x06a7; /* U+0457 CYRILLIC SMALL LETTER YI */
pub const Ukranian_yi: Keysym = 0x06a7; /* deprecated */
pub const Cyrillic_je: Keysym = 0x06a8; /* U+0458 CYRILLIC SMALL LETTER JE */
pub const Serbian_je: Keysym = 0x06a8; /* deprecated */
pub const Cyrillic_lje: Keysym = 0x06a9; /* U+0459 CYRILLIC SMALL LETTER LJE */
pub const Serbian_lje: Keysym = 0x06a9; /* deprecated */
pub const Cyrillic_nje: Keysym = 0x06aa; /* U+045A CYRILLIC SMALL LETTER NJE */
pub const Serbian_nje: Keysym = 0x06aa; /* deprecated */
pub const Serbian_tshe: Keysym = 0x06ab; /* U+045B CYRILLIC SMALL LETTER TSHE */
pub const Macedonia_kje: Keysym = 0x06ac; /* U+045C CYRILLIC SMALL LETTER KJE */
pub const Ukrainian_ghe_with_upturn: Keysym = 0x06ad; /* U+0491 CYRILLIC SMALL LETTER GHE WITH UPTURN */
pub const Byelorussian_shortu: Keysym = 0x06ae; /* U+045E CYRILLIC SMALL LETTER SHORT U */
pub const Cyrillic_dzhe: Keysym = 0x06af; /* U+045F CYRILLIC SMALL LETTER DZHE */
pub const Serbian_dze: Keysym = 0x06af; /* deprecated */
pub const numerosign: Keysym = 0x06b0; /* U+2116 NUMERO SIGN */
pub const Serbian_DJE: Keysym = 0x06b1; /* U+0402 CYRILLIC CAPITAL LETTER DJE */
pub const Macedonia_GJE: Keysym = 0x06b2; /* U+0403 CYRILLIC CAPITAL LETTER GJE */
pub const Cyrillic_IO: Keysym = 0x06b3; /* U+0401 CYRILLIC CAPITAL LETTER IO */
pub const Ukrainian_IE: Keysym = 0x06b4; /* U+0404 CYRILLIC CAPITAL LETTER UKRAINIAN IE */
pub const Ukranian_JE: Keysym = 0x06b4; /* deprecated */
pub const Macedonia_DSE: Keysym = 0x06b5; /* U+0405 CYRILLIC CAPITAL LETTER DZE */
pub const Ukrainian_I: Keysym = 0x06b6; /* U+0406 CYRILLIC CAPITAL LETTER BYELORUSSIAN-UKRAINIAN I */
pub const Ukranian_I: Keysym = 0x06b6; /* deprecated */
pub const Ukrainian_YI: Keysym = 0x06b7; /* U+0407 CYRILLIC CAPITAL LETTER YI */
pub const Ukranian_YI: Keysym = 0x06b7; /* deprecated */
pub const Cyrillic_JE: Keysym = 0x06b8; /* U+0408 CYRILLIC CAPITAL LETTER JE */
pub const Serbian_JE: Keysym = 0x06b8; /* deprecated */
pub const Cyrillic_LJE: Keysym = 0x06b9; /* U+0409 CYRILLIC CAPITAL LETTER LJE */
pub const Serbian_LJE: Keysym = 0x06b9; /* deprecated */
pub const Cyrillic_NJE: Keysym = 0x06ba; /* U+040A CYRILLIC CAPITAL LETTER NJE */
pub const Serbian_NJE: Keysym = 0x06ba; /* deprecated */
pub const Serbian_TSHE: Keysym = 0x06bb; /* U+040B CYRILLIC CAPITAL LETTER TSHE */
pub const Macedonia_KJE: Keysym = 0x06bc; /* U+040C CYRILLIC CAPITAL LETTER KJE */
pub const Ukrainian_GHE_WITH_UPTURN: Keysym = 0x06bd; /* U+0490 CYRILLIC CAPITAL LETTER GHE WITH UPTURN */
pub const Byelorussian_SHORTU: Keysym = 0x06be; /* U+040E CYRILLIC CAPITAL LETTER SHORT U */
pub const Cyrillic_DZHE: Keysym = 0x06bf; /* U+040F CYRILLIC CAPITAL LETTER DZHE */
pub const Serbian_DZE: Keysym = 0x06bf; /* deprecated */
pub const Cyrillic_yu: Keysym = 0x06c0; /* U+044E CYRILLIC SMALL LETTER YU */
pub const Cyrillic_a: Keysym = 0x06c1; /* U+0430 CYRILLIC SMALL LETTER A */
pub const Cyrillic_be: Keysym = 0x06c2; /* U+0431 CYRILLIC SMALL LETTER BE */
pub const Cyrillic_tse: Keysym = 0x06c3; /* U+0446 CYRILLIC SMALL LETTER TSE */
pub const Cyrillic_de: Keysym = 0x06c4; /* U+0434 CYRILLIC SMALL LETTER DE */
pub const Cyrillic_ie: Keysym = 0x06c5; /* U+0435 CYRILLIC SMALL LETTER IE */
pub const Cyrillic_ef: Keysym = 0x06c6; /* U+0444 CYRILLIC SMALL LETTER EF */
pub const Cyrillic_ghe: Keysym = 0x06c7; /* U+0433 CYRILLIC SMALL LETTER GHE */
pub const Cyrillic_ha: Keysym = 0x06c8; /* U+0445 CYRILLIC SMALL LETTER HA */
pub const Cyrillic_i: Keysym = 0x06c9; /* U+0438 CYRILLIC SMALL LETTER I */
pub const Cyrillic_shorti: Keysym = 0x06ca; /* U+0439 CYRILLIC SMALL LETTER SHORT I */
pub const Cyrillic_ka: Keysym = 0x06cb; /* U+043A CYRILLIC SMALL LETTER KA */
pub const Cyrillic_el: Keysym = 0x06cc; /* U+043B CYRILLIC SMALL LETTER EL */
pub const Cyrillic_em: Keysym = 0x06cd; /* U+043C CYRILLIC SMALL LETTER EM */
pub const Cyrillic_en: Keysym = 0x06ce; /* U+043D CYRILLIC SMALL LETTER EN */
pub const Cyrillic_o: Keysym = 0x06cf; /* U+043E CYRILLIC SMALL LETTER O */
pub const Cyrillic_pe: Keysym = 0x06d0; /* U+043F CYRILLIC SMALL LETTER PE */
pub const Cyrillic_ya: Keysym = 0x06d1; /* U+044F CYRILLIC SMALL LETTER YA */
pub const Cyrillic_er: Keysym = 0x06d2; /* U+0440 CYRILLIC SMALL LETTER ER */
pub const Cyrillic_es: Keysym = 0x06d3; /* U+0441 CYRILLIC SMALL LETTER ES */
pub const Cyrillic_te: Keysym = 0x06d4; /* U+0442 CYRILLIC SMALL LETTER TE */
pub const Cyrillic_u: Keysym = 0x06d5; /* U+0443 CYRILLIC SMALL LETTER U */
pub const Cyrillic_zhe: Keysym = 0x06d6; /* U+0436 CYRILLIC SMALL LETTER ZHE */
pub const Cyrillic_ve: Keysym = 0x06d7; /* U+0432 CYRILLIC SMALL LETTER VE */
pub const Cyrillic_softsign: Keysym = 0x06d8; /* U+044C CYRILLIC SMALL LETTER SOFT SIGN */
pub const Cyrillic_yeru: Keysym = 0x06d9; /* U+044B CYRILLIC SMALL LETTER YERU */
pub const Cyrillic_ze: Keysym = 0x06da; /* U+0437 CYRILLIC SMALL LETTER ZE */
pub const Cyrillic_sha: Keysym = 0x06db; /* U+0448 CYRILLIC SMALL LETTER SHA */
pub const Cyrillic_e: Keysym = 0x06dc; /* U+044D CYRILLIC SMALL LETTER E */
pub const Cyrillic_shcha: Keysym = 0x06dd; /* U+0449 CYRILLIC SMALL LETTER SHCHA */
pub const Cyrillic_che: Keysym = 0x06de; /* U+0447 CYRILLIC SMALL LETTER CHE */
pub const Cyrillic_hardsign: Keysym = 0x06df; /* U+044A CYRILLIC SMALL LETTER HARD SIGN */
pub const Cyrillic_YU: Keysym = 0x06e0; /* U+042E CYRILLIC CAPITAL LETTER YU */
pub const Cyrillic_A: Keysym = 0x06e1; /* U+0410 CYRILLIC CAPITAL LETTER A */
pub const Cyrillic_BE: Keysym = 0x06e2; /* U+0411 CYRILLIC CAPITAL LETTER BE */
pub const Cyrillic_TSE: Keysym = 0x06e3; /* U+0426 CYRILLIC CAPITAL LETTER TSE */
pub const Cyrillic_DE: Keysym = 0x06e4; /* U+0414 CYRILLIC CAPITAL LETTER DE */
pub const Cyrillic_IE: Keysym = 0x06e5; /* U+0415 CYRILLIC CAPITAL LETTER IE */
pub const Cyrillic_EF: Keysym = 0x06e6; /* U+0424 CYRILLIC CAPITAL LETTER EF */
pub const Cyrillic_GHE: Keysym = 0x06e7; /* U+0413 CYRILLIC CAPITAL LETTER GHE */
pub const Cyrillic_HA: Keysym = 0x06e8; /* U+0425 CYRILLIC CAPITAL LETTER HA */
pub const Cyrillic_I: Keysym = 0x06e9; /* U+0418 CYRILLIC CAPITAL LETTER I */
pub const Cyrillic_SHORTI: Keysym = 0x06ea; /* U+0419 CYRILLIC CAPITAL LETTER SHORT I */
pub const Cyrillic_KA: Keysym = 0x06eb; /* U+041A CYRILLIC CAPITAL LETTER KA */
pub const Cyrillic_EL: Keysym = 0x06ec; /* U+041B CYRILLIC CAPITAL LETTER EL */
pub const Cyrillic_EM: Keysym = 0x06ed; /* U+041C CYRILLIC CAPITAL LETTER EM */
pub const Cyrillic_EN: Keysym = 0x06ee; /* U+041D CYRILLIC CAPITAL LETTER EN */
pub const Cyrillic_O: Keysym = 0x06ef; /* U+041E CYRILLIC CAPITAL LETTER O */
pub const Cyrillic_PE: Keysym = 0x06f0; /* U+041F CYRILLIC CAPITAL LETTER PE */
pub const Cyrillic_YA: Keysym = 0x06f1; /* U+042F CYRILLIC CAPITAL LETTER YA */
pub const Cyrillic_ER: Keysym = 0x06f2; /* U+0420 CYRILLIC CAPITAL LETTER ER */
pub const Cyrillic_ES: Keysym = 0x06f3; /* U+0421 CYRILLIC CAPITAL LETTER ES */
pub const Cyrillic_TE: Keysym = 0x06f4; /* U+0422 CYRILLIC CAPITAL LETTER TE */
pub const Cyrillic_U: Keysym = 0x06f5; /* U+0423 CYRILLIC CAPITAL LETTER U */
pub const Cyrillic_ZHE: Keysym = 0x06f6; /* U+0416 CYRILLIC CAPITAL LETTER ZHE */
pub const Cyrillic_VE: Keysym = 0x06f7; /* U+0412 CYRILLIC CAPITAL LETTER VE */
pub const Cyrillic_SOFTSIGN: Keysym = 0x06f8; /* U+042C CYRILLIC CAPITAL LETTER SOFT SIGN */
pub const Cyrillic_YERU: Keysym = 0x06f9; /* U+042B CYRILLIC CAPITAL LETTER YERU */
pub const Cyrillic_ZE: Keysym = 0x06fa; /* U+0417 CYRILLIC CAPITAL LETTER ZE */
pub const Cyrillic_SHA: Keysym = 0x06fb; /* U+0428 CYRILLIC CAPITAL LETTER SHA */
pub const Cyrillic_E: Keysym = 0x06fc; /* U+042D CYRILLIC CAPITAL LETTER E */
pub const Cyrillic_SHCHA: Keysym = 0x06fd; /* U+0429 CYRILLIC CAPITAL LETTER SHCHA */
pub const Cyrillic_CHE: Keysym = 0x06fe; /* U+0427 CYRILLIC CAPITAL LETTER CHE */
pub const Cyrillic_HARDSIGN: Keysym = 0x06ff; /* U+042A CYRILLIC CAPITAL LETTER HARD SIGN */

/*
 * Greek
 * (based on an early draft of, and not quite identical to, ISO/IEC 8859-7)
 * Byte 3 = 7;
 */

pub const Greek_ALPHAaccent: Keysym = 0x07a1; /* U+0386 GREEK CAPITAL LETTER ALPHA WITH TONOS */
pub const Greek_EPSILONaccent: Keysym = 0x07a2; /* U+0388 GREEK CAPITAL LETTER EPSILON WITH TONOS */
pub const Greek_ETAaccent: Keysym = 0x07a3; /* U+0389 GREEK CAPITAL LETTER ETA WITH TONOS */
pub const Greek_IOTAaccent: Keysym = 0x07a4; /* U+038A GREEK CAPITAL LETTER IOTA WITH TONOS */
pub const Greek_IOTAdieresis: Keysym = 0x07a5; /* U+03AA GREEK CAPITAL LETTER IOTA WITH DIALYTIKA */
pub const Greek_IOTAdiaeresis: Keysym = 0x07a5; /* old typo */
pub const Greek_OMICRONaccent: Keysym = 0x07a7; /* U+038C GREEK CAPITAL LETTER OMICRON WITH TONOS */
pub const Greek_UPSILONaccent: Keysym = 0x07a8; /* U+038E GREEK CAPITAL LETTER UPSILON WITH TONOS */
pub const Greek_UPSILONdieresis: Keysym = 0x07a9; /* U+03AB GREEK CAPITAL LETTER UPSILON WITH DIALYTIKA */
pub const Greek_OMEGAaccent: Keysym = 0x07ab; /* U+038F GREEK CAPITAL LETTER OMEGA WITH TONOS */
pub const Greek_accentdieresis: Keysym = 0x07ae; /* U+0385 GREEK DIALYTIKA TONOS */
pub const Greek_horizbar: Keysym = 0x07af; /* U+2015 HORIZONTAL BAR */
pub const Greek_alphaaccent: Keysym = 0x07b1; /* U+03AC GREEK SMALL LETTER ALPHA WITH TONOS */
pub const Greek_epsilonaccent: Keysym = 0x07b2; /* U+03AD GREEK SMALL LETTER EPSILON WITH TONOS */
pub const Greek_etaaccent: Keysym = 0x07b3; /* U+03AE GREEK SMALL LETTER ETA WITH TONOS */
pub const Greek_iotaaccent: Keysym = 0x07b4; /* U+03AF GREEK SMALL LETTER IOTA WITH TONOS */
pub const Greek_iotadieresis: Keysym = 0x07b5; /* U+03CA GREEK SMALL LETTER IOTA WITH DIALYTIKA */
pub const Greek_iotaaccentdieresis: Keysym = 0x07b6; /* U+0390 GREEK SMALL LETTER IOTA WITH DIALYTIKA AND TONOS */
pub const Greek_omicronaccent: Keysym = 0x07b7; /* U+03CC GREEK SMALL LETTER OMICRON WITH TONOS */
pub const Greek_upsilonaccent: Keysym = 0x07b8; /* U+03CD GREEK SMALL LETTER UPSILON WITH TONOS */
pub const Greek_upsilondieresis: Keysym = 0x07b9; /* U+03CB GREEK SMALL LETTER UPSILON WITH DIALYTIKA */
pub const Greek_upsilonaccentdieresis: Keysym = 0x07ba; /* U+03B0 GREEK SMALL LETTER UPSILON WITH DIALYTIKA AND TONOS */
pub const Greek_omegaaccent: Keysym = 0x07bb; /* U+03CE GREEK SMALL LETTER OMEGA WITH TONOS */
pub const Greek_ALPHA: Keysym = 0x07c1; /* U+0391 GREEK CAPITAL LETTER ALPHA */
pub const Greek_BETA: Keysym = 0x07c2; /* U+0392 GREEK CAPITAL LETTER BETA */
pub const Greek_GAMMA: Keysym = 0x07c3; /* U+0393 GREEK CAPITAL LETTER GAMMA */
pub const Greek_DELTA: Keysym = 0x07c4; /* U+0394 GREEK CAPITAL LETTER DELTA */
pub const Greek_EPSILON: Keysym = 0x07c5; /* U+0395 GREEK CAPITAL LETTER EPSILON */
pub const Greek_ZETA: Keysym = 0x07c6; /* U+0396 GREEK CAPITAL LETTER ZETA */
pub const Greek_ETA: Keysym = 0x07c7; /* U+0397 GREEK CAPITAL LETTER ETA */
pub const Greek_THETA: Keysym = 0x07c8; /* U+0398 GREEK CAPITAL LETTER THETA */
pub const Greek_IOTA: Keysym = 0x07c9; /* U+0399 GREEK CAPITAL LETTER IOTA */
pub const Greek_KAPPA: Keysym = 0x07ca; /* U+039A GREEK CAPITAL LETTER KAPPA */
pub const Greek_LAMDA: Keysym = 0x07cb; /* U+039B GREEK CAPITAL LETTER LAMDA */
pub const Greek_LAMBDA: Keysym = 0x07cb; /* U+039B GREEK CAPITAL LETTER LAMDA */
pub const Greek_MU: Keysym = 0x07cc; /* U+039C GREEK CAPITAL LETTER MU */
pub const Greek_NU: Keysym = 0x07cd; /* U+039D GREEK CAPITAL LETTER NU */
pub const Greek_XI: Keysym = 0x07ce; /* U+039E GREEK CAPITAL LETTER XI */
pub const Greek_OMICRON: Keysym = 0x07cf; /* U+039F GREEK CAPITAL LETTER OMICRON */
pub const Greek_PI: Keysym = 0x07d0; /* U+03A0 GREEK CAPITAL LETTER PI */
pub const Greek_RHO: Keysym = 0x07d1; /* U+03A1 GREEK CAPITAL LETTER RHO */
pub const Greek_SIGMA: Keysym = 0x07d2; /* U+03A3 GREEK CAPITAL LETTER SIGMA */
pub const Greek_TAU: Keysym = 0x07d4; /* U+03A4 GREEK CAPITAL LETTER TAU */
pub const Greek_UPSILON: Keysym = 0x07d5; /* U+03A5 GREEK CAPITAL LETTER UPSILON */
pub const Greek_PHI: Keysym = 0x07d6; /* U+03A6 GREEK CAPITAL LETTER PHI */
pub const Greek_CHI: Keysym = 0x07d7; /* U+03A7 GREEK CAPITAL LETTER CHI */
pub const Greek_PSI: Keysym = 0x07d8; /* U+03A8 GREEK CAPITAL LETTER PSI */
pub const Greek_OMEGA: Keysym = 0x07d9; /* U+03A9 GREEK CAPITAL LETTER OMEGA */
pub const Greek_alpha: Keysym = 0x07e1; /* U+03B1 GREEK SMALL LETTER ALPHA */
pub const Greek_beta: Keysym = 0x07e2; /* U+03B2 GREEK SMALL LETTER BETA */
pub const Greek_gamma: Keysym = 0x07e3; /* U+03B3 GREEK SMALL LETTER GAMMA */
pub const Greek_delta: Keysym = 0x07e4; /* U+03B4 GREEK SMALL LETTER DELTA */
pub const Greek_epsilon: Keysym = 0x07e5; /* U+03B5 GREEK SMALL LETTER EPSILON */
pub const Greek_zeta: Keysym = 0x07e6; /* U+03B6 GREEK SMALL LETTER ZETA */
pub const Greek_eta: Keysym = 0x07e7; /* U+03B7 GREEK SMALL LETTER ETA */
pub const Greek_theta: Keysym = 0x07e8; /* U+03B8 GREEK SMALL LETTER THETA */
pub const Greek_iota: Keysym = 0x07e9; /* U+03B9 GREEK SMALL LETTER IOTA */
pub const Greek_kappa: Keysym = 0x07ea; /* U+03BA GREEK SMALL LETTER KAPPA */
pub const Greek_lamda: Keysym = 0x07eb; /* U+03BB GREEK SMALL LETTER LAMDA */
pub const Greek_lambda: Keysym = 0x07eb; /* U+03BB GREEK SMALL LETTER LAMDA */
pub const Greek_mu: Keysym = 0x07ec; /* U+03BC GREEK SMALL LETTER MU */
pub const Greek_nu: Keysym = 0x07ed; /* U+03BD GREEK SMALL LETTER NU */
pub const Greek_xi: Keysym = 0x07ee; /* U+03BE GREEK SMALL LETTER XI */
pub const Greek_omicron: Keysym = 0x07ef; /* U+03BF GREEK SMALL LETTER OMICRON */
pub const Greek_pi: Keysym = 0x07f0; /* U+03C0 GREEK SMALL LETTER PI */
pub const Greek_rho: Keysym = 0x07f1; /* U+03C1 GREEK SMALL LETTER RHO */
pub const Greek_sigma: Keysym = 0x07f2; /* U+03C3 GREEK SMALL LETTER SIGMA */
pub const Greek_finalsmallsigma: Keysym = 0x07f3; /* U+03C2 GREEK SMALL LETTER FINAL SIGMA */
pub const Greek_tau: Keysym = 0x07f4; /* U+03C4 GREEK SMALL LETTER TAU */
pub const Greek_upsilon: Keysym = 0x07f5; /* U+03C5 GREEK SMALL LETTER UPSILON */
pub const Greek_phi: Keysym = 0x07f6; /* U+03C6 GREEK SMALL LETTER PHI */
pub const Greek_chi: Keysym = 0x07f7; /* U+03C7 GREEK SMALL LETTER CHI */
pub const Greek_psi: Keysym = 0x07f8; /* U+03C8 GREEK SMALL LETTER PSI */
pub const Greek_omega: Keysym = 0x07f9; /* U+03C9 GREEK SMALL LETTER OMEGA */
pub const Greek_switch: Keysym = 0xff7e; /* Alias for mode_switch */

/*
 * Technical
 * (from the DEC VT330/VT420 Technical Character Set, http://vt100.net/charsets/technical.html)
 * Byte 3 = 8;
 */

pub const leftradical: Keysym = 0x08a1; /* U+23B7 RADICAL SYMBOL BOTTOM */
pub const topleftradical: Keysym = 0x08a2; /*(U+250C BOX DRAWINGS LIGHT DOWN AND RIGHT)*/
pub const horizconnector: Keysym = 0x08a3; /*(U+2500 BOX DRAWINGS LIGHT HORIZONTAL)*/
pub const topintegral: Keysym = 0x08a4; /* U+2320 TOP HALF INTEGRAL */
pub const botintegral: Keysym = 0x08a5; /* U+2321 BOTTOM HALF INTEGRAL */
pub const vertconnector: Keysym = 0x08a6; /*(U+2502 BOX DRAWINGS LIGHT VERTICAL)*/
pub const topleftsqbracket: Keysym = 0x08a7; /* U+23A1 LEFT SQUARE BRACKET UPPER CORNER */
pub const botleftsqbracket: Keysym = 0x08a8; /* U+23A3 LEFT SQUARE BRACKET LOWER CORNER */
pub const toprightsqbracket: Keysym = 0x08a9; /* U+23A4 RIGHT SQUARE BRACKET UPPER CORNER */
pub const botrightsqbracket: Keysym = 0x08aa; /* U+23A6 RIGHT SQUARE BRACKET LOWER CORNER */
pub const topleftparens: Keysym = 0x08ab; /* U+239B LEFT PARENTHESIS UPPER HOOK */
pub const botleftparens: Keysym = 0x08ac; /* U+239D LEFT PARENTHESIS LOWER HOOK */
pub const toprightparens: Keysym = 0x08ad; /* U+239E RIGHT PARENTHESIS UPPER HOOK */
pub const botrightparens: Keysym = 0x08ae; /* U+23A0 RIGHT PARENTHESIS LOWER HOOK */
pub const leftmiddlecurlybrace: Keysym = 0x08af; /* U+23A8 LEFT CURLY BRACKET MIDDLE PIECE */
pub const rightmiddlecurlybrace: Keysym = 0x08b0; /* U+23AC RIGHT CURLY BRACKET MIDDLE PIECE */
pub const topleftsummation: Keysym = 0x08b1;
pub const botleftsummation: Keysym = 0x08b2;
pub const topvertsummationconnector: Keysym = 0x08b3;
pub const botvertsummationconnector: Keysym = 0x08b4;
pub const toprightsummation: Keysym = 0x08b5;
pub const botrightsummation: Keysym = 0x08b6;
pub const rightmiddlesummation: Keysym = 0x08b7;
pub const lessthanequal: Keysym = 0x08bc; /* U+2264 LESS-THAN OR EQUAL TO */
pub const notequal: Keysym = 0x08bd; /* U+2260 NOT EQUAL TO */
pub const greaterthanequal: Keysym = 0x08be; /* U+2265 GREATER-THAN OR EQUAL TO */
pub const integral: Keysym = 0x08bf; /* U+222B INTEGRAL */
pub const therefore: Keysym = 0x08c0; /* U+2234 THEREFORE */
pub const variation: Keysym = 0x08c1; /* U+221D PROPORTIONAL TO */
pub const infinity: Keysym = 0x08c2; /* U+221E INFINITY */
pub const nabla: Keysym = 0x08c5; /* U+2207 NABLA */
pub const approximate: Keysym = 0x08c8; /* U+223C TILDE OPERATOR */
pub const similarequal: Keysym = 0x08c9; /* U+2243 ASYMPTOTICALLY EQUAL TO */
pub const ifonlyif: Keysym = 0x08cd; /* U+21D4 LEFT RIGHT DOUBLE ARROW */
pub const implies: Keysym = 0x08ce; /* U+21D2 RIGHTWARDS DOUBLE ARROW */
pub const identical: Keysym = 0x08cf; /* U+2261 IDENTICAL TO */
pub const radical: Keysym = 0x08d6; /* U+221A SQUARE ROOT */
pub const includedin: Keysym = 0x08da; /* U+2282 SUBSET OF */
pub const includes: Keysym = 0x08db; /* U+2283 SUPERSET OF */
pub const intersection: Keysym = 0x08dc; /* U+2229 INTERSECTION */
pub const union: Keysym = 0x08dd; /* U+222A UNION */
pub const logicaland: Keysym = 0x08de; /* U+2227 LOGICAL AND */
pub const logicalor: Keysym = 0x08df; /* U+2228 LOGICAL OR */
pub const partialderivative: Keysym = 0x08ef; /* U+2202 PARTIAL DIFFERENTIAL */
pub const function: Keysym = 0x08f6; /* U+0192 LATIN SMALL LETTER F WITH HOOK */
pub const leftarrow: Keysym = 0x08fb; /* U+2190 LEFTWARDS ARROW */
pub const uparrow: Keysym = 0x08fc; /* U+2191 UPWARDS ARROW */
pub const rightarrow: Keysym = 0x08fd; /* U+2192 RIGHTWARDS ARROW */
pub const downarrow: Keysym = 0x08fe; /* U+2193 DOWNWARDS ARROW */

/*
 * Special
 * (from the DEC VT100 Special Graphics Character Set)
 * Byte 3 = 9;
 */

pub const blank: Keysym = 0x09df;
pub const soliddiamond: Keysym = 0x09e0; /* U+25C6 BLACK DIAMOND */
pub const checkerboard: Keysym = 0x09e1; /* U+2592 MEDIUM SHADE */
pub const ht: Keysym = 0x09e2; /* U+2409 SYMBOL FOR HORIZONTAL TABULATION */
pub const ff: Keysym = 0x09e3; /* U+240C SYMBOL FOR FORM FEED */
pub const cr: Keysym = 0x09e4; /* U+240D SYMBOL FOR CARRIAGE RETURN */
pub const lf: Keysym = 0x09e5; /* U+240A SYMBOL FOR LINE FEED */
pub const nl: Keysym = 0x09e8; /* U+2424 SYMBOL FOR NEWLINE */
pub const vt: Keysym = 0x09e9; /* U+240B SYMBOL FOR VERTICAL TABULATION */
pub const lowrightcorner: Keysym = 0x09ea; /* U+2518 BOX DRAWINGS LIGHT UP AND LEFT */
pub const uprightcorner: Keysym = 0x09eb; /* U+2510 BOX DRAWINGS LIGHT DOWN AND LEFT */
pub const upleftcorner: Keysym = 0x09ec; /* U+250C BOX DRAWINGS LIGHT DOWN AND RIGHT */
pub const lowleftcorner: Keysym = 0x09ed; /* U+2514 BOX DRAWINGS LIGHT UP AND RIGHT */
pub const crossinglines: Keysym = 0x09ee; /* U+253C BOX DRAWINGS LIGHT VERTICAL AND HORIZONTAL */
pub const horizlinescan1: Keysym = 0x09ef; /* U+23BA HORIZONTAL SCAN LINE-1 */
pub const horizlinescan3: Keysym = 0x09f0; /* U+23BB HORIZONTAL SCAN LINE-3 */
pub const horizlinescan5: Keysym = 0x09f1; /* U+2500 BOX DRAWINGS LIGHT HORIZONTAL */
pub const horizlinescan7: Keysym = 0x09f2; /* U+23BC HORIZONTAL SCAN LINE-7 */
pub const horizlinescan9: Keysym = 0x09f3; /* U+23BD HORIZONTAL SCAN LINE-9 */
pub const leftt: Keysym = 0x09f4; /* U+251C BOX DRAWINGS LIGHT VERTICAL AND RIGHT */
pub const rightt: Keysym = 0x09f5; /* U+2524 BOX DRAWINGS LIGHT VERTICAL AND LEFT */
pub const bott: Keysym = 0x09f6; /* U+2534 BOX DRAWINGS LIGHT UP AND HORIZONTAL */
pub const topt: Keysym = 0x09f7; /* U+252C BOX DRAWINGS LIGHT DOWN AND HORIZONTAL */
pub const vertbar: Keysym = 0x09f8; /* U+2502 BOX DRAWINGS LIGHT VERTICAL */

/*
 * Publishing
 * (these are probably from a long forgotten DEC Publishing
 * font that once shipped with DECwrite)
 * Byte 3 = 0x0a;
 */

pub const emspace: Keysym = 0x0aa1; /* U+2003 EM SPACE */
pub const enspace: Keysym = 0x0aa2; /* U+2002 EN SPACE */
pub const em3space: Keysym = 0x0aa3; /* U+2004 THREE-PER-EM SPACE */
pub const em4space: Keysym = 0x0aa4; /* U+2005 FOUR-PER-EM SPACE */
pub const digitspace: Keysym = 0x0aa5; /* U+2007 FIGURE SPACE */
pub const punctspace: Keysym = 0x0aa6; /* U+2008 PUNCTUATION SPACE */
pub const thinspace: Keysym = 0x0aa7; /* U+2009 THIN SPACE */
pub const hairspace: Keysym = 0x0aa8; /* U+200A HAIR SPACE */
pub const emdash: Keysym = 0x0aa9; /* U+2014 EM DASH */
pub const endash: Keysym = 0x0aaa; /* U+2013 EN DASH */
pub const signifblank: Keysym = 0x0aac; /*(U+2423 OPEN BOX)*/
pub const ellipsis: Keysym = 0x0aae; /* U+2026 HORIZONTAL ELLIPSIS */
pub const doubbaselinedot: Keysym = 0x0aaf; /* U+2025 TWO DOT LEADER */
pub const onethird: Keysym = 0x0ab0; /* U+2153 VULGAR FRACTION ONE THIRD */
pub const twothirds: Keysym = 0x0ab1; /* U+2154 VULGAR FRACTION TWO THIRDS */
pub const onefifth: Keysym = 0x0ab2; /* U+2155 VULGAR FRACTION ONE FIFTH */
pub const twofifths: Keysym = 0x0ab3; /* U+2156 VULGAR FRACTION TWO FIFTHS */
pub const threefifths: Keysym = 0x0ab4; /* U+2157 VULGAR FRACTION THREE FIFTHS */
pub const fourfifths: Keysym = 0x0ab5; /* U+2158 VULGAR FRACTION FOUR FIFTHS */
pub const onesixth: Keysym = 0x0ab6; /* U+2159 VULGAR FRACTION ONE SIXTH */
pub const fivesixths: Keysym = 0x0ab7; /* U+215A VULGAR FRACTION FIVE SIXTHS */
pub const careof: Keysym = 0x0ab8; /* U+2105 CARE OF */
pub const figdash: Keysym = 0x0abb; /* U+2012 FIGURE DASH */
pub const leftanglebracket: Keysym = 0x0abc; /*(U+2329 LEFT-POINTING ANGLE BRACKET)*/
pub const decimalpoint: Keysym = 0x0abd; /*(U+002E FULL STOP)*/
pub const rightanglebracket: Keysym = 0x0abe; /*(U+232A RIGHT-POINTING ANGLE BRACKET)*/
pub const marker: Keysym = 0x0abf;
pub const oneeighth: Keysym = 0x0ac3; /* U+215B VULGAR FRACTION ONE EIGHTH */
pub const threeeighths: Keysym = 0x0ac4; /* U+215C VULGAR FRACTION THREE EIGHTHS */
pub const fiveeighths: Keysym = 0x0ac5; /* U+215D VULGAR FRACTION FIVE EIGHTHS */
pub const seveneighths: Keysym = 0x0ac6; /* U+215E VULGAR FRACTION SEVEN EIGHTHS */
pub const trademark: Keysym = 0x0ac9; /* U+2122 TRADE MARK SIGN */
pub const signaturemark: Keysym = 0x0aca; /*(U+2613 SALTIRE)*/
pub const trademarkincircle: Keysym = 0x0acb;
pub const leftopentriangle: Keysym = 0x0acc; /*(U+25C1 WHITE LEFT-POINTING TRIANGLE)*/
pub const rightopentriangle: Keysym = 0x0acd; /*(U+25B7 WHITE RIGHT-POINTING TRIANGLE)*/
pub const emopencircle: Keysym = 0x0ace; /*(U+25CB WHITE CIRCLE)*/
pub const emopenrectangle: Keysym = 0x0acf; /*(U+25AF WHITE VERTICAL RECTANGLE)*/
pub const leftsinglequotemark: Keysym = 0x0ad0; /* U+2018 LEFT SINGLE QUOTATION MARK */
pub const rightsinglequotemark: Keysym = 0x0ad1; /* U+2019 RIGHT SINGLE QUOTATION MARK */
pub const leftdoublequotemark: Keysym = 0x0ad2; /* U+201C LEFT DOUBLE QUOTATION MARK */
pub const rightdoublequotemark: Keysym = 0x0ad3; /* U+201D RIGHT DOUBLE QUOTATION MARK */
pub const prescription: Keysym = 0x0ad4; /* U+211E PRESCRIPTION TAKE */
pub const permille: Keysym = 0x0ad5; /* U+2030 PER MILLE SIGN */
pub const minutes: Keysym = 0x0ad6; /* U+2032 PRIME */
pub const seconds: Keysym = 0x0ad7; /* U+2033 DOUBLE PRIME */
pub const latincross: Keysym = 0x0ad9; /* U+271D LATIN CROSS */
pub const hexagram: Keysym = 0x0ada;
pub const filledrectbullet: Keysym = 0x0adb; /*(U+25AC BLACK RECTANGLE)*/
pub const filledlefttribullet: Keysym = 0x0adc; /*(U+25C0 BLACK LEFT-POINTING TRIANGLE)*/
pub const filledrighttribullet: Keysym = 0x0add; /*(U+25B6 BLACK RIGHT-POINTING TRIANGLE)*/
pub const emfilledcircle: Keysym = 0x0ade; /*(U+25CF BLACK CIRCLE)*/
pub const emfilledrect: Keysym = 0x0adf; /*(U+25AE BLACK VERTICAL RECTANGLE)*/
pub const enopencircbullet: Keysym = 0x0ae0; /*(U+25E6 WHITE BULLET)*/
pub const enopensquarebullet: Keysym = 0x0ae1; /*(U+25AB WHITE SMALL SQUARE)*/
pub const openrectbullet: Keysym = 0x0ae2; /*(U+25AD WHITE RECTANGLE)*/
pub const opentribulletup: Keysym = 0x0ae3; /*(U+25B3 WHITE UP-POINTING TRIANGLE)*/
pub const opentribulletdown: Keysym = 0x0ae4; /*(U+25BD WHITE DOWN-POINTING TRIANGLE)*/
pub const openstar: Keysym = 0x0ae5; /*(U+2606 WHITE STAR)*/
pub const enfilledcircbullet: Keysym = 0x0ae6; /*(U+2022 BULLET)*/
pub const enfilledsqbullet: Keysym = 0x0ae7; /*(U+25AA BLACK SMALL SQUARE)*/
pub const filledtribulletup: Keysym = 0x0ae8; /*(U+25B2 BLACK UP-POINTING TRIANGLE)*/
pub const filledtribulletdown: Keysym = 0x0ae9; /*(U+25BC BLACK DOWN-POINTING TRIANGLE)*/
pub const leftpointer: Keysym = 0x0aea; /*(U+261C WHITE LEFT POINTING INDEX)*/
pub const rightpointer: Keysym = 0x0aeb; /*(U+261E WHITE RIGHT POINTING INDEX)*/
pub const club: Keysym = 0x0aec; /* U+2663 BLACK CLUB SUIT */
pub const diamond: Keysym = 0x0aed; /* U+2666 BLACK DIAMOND SUIT */
pub const heart: Keysym = 0x0aee; /* U+2665 BLACK HEART SUIT */
pub const maltesecross: Keysym = 0x0af0; /* U+2720 MALTESE CROSS */
pub const dagger: Keysym = 0x0af1; /* U+2020 DAGGER */
pub const doubledagger: Keysym = 0x0af2; /* U+2021 DOUBLE DAGGER */
pub const checkmark: Keysym = 0x0af3; /* U+2713 CHECK MARK */
pub const ballotcross: Keysym = 0x0af4; /* U+2717 BALLOT X */
pub const musicalsharp: Keysym = 0x0af5; /* U+266F MUSIC SHARP SIGN */
pub const musicalflat: Keysym = 0x0af6; /* U+266D MUSIC FLAT SIGN */
pub const malesymbol: Keysym = 0x0af7; /* U+2642 MALE SIGN */
pub const femalesymbol: Keysym = 0x0af8; /* U+2640 FEMALE SIGN */
pub const telephone: Keysym = 0x0af9; /* U+260E BLACK TELEPHONE */
pub const telephonerecorder: Keysym = 0x0afa; /* U+2315 TELEPHONE RECORDER */
pub const phonographcopyright: Keysym = 0x0afb; /* U+2117 SOUND RECORDING COPYRIGHT */
pub const caret: Keysym = 0x0afc; /* U+2038 CARET */
pub const singlelowquotemark: Keysym = 0x0afd; /* U+201A SINGLE LOW-9 QUOTATION MARK */
pub const doublelowquotemark: Keysym = 0x0afe; /* U+201E DOUBLE LOW-9 QUOTATION MARK */
pub const cursor: Keysym = 0x0aff;

/*
 * APL
 * Byte 3 = 0x0b;
 */

pub const leftcaret: Keysym = 0x0ba3; /*(U+003C LESS-THAN SIGN)*/
pub const rightcaret: Keysym = 0x0ba6; /*(U+003E GREATER-THAN SIGN)*/
pub const downcaret: Keysym = 0x0ba8; /*(U+2228 LOGICAL OR)*/
pub const upcaret: Keysym = 0x0ba9; /*(U+2227 LOGICAL AND)*/
pub const overbar: Keysym = 0x0bc0; /*(U+00AF MACRON)*/
pub const downtack: Keysym = 0x0bc2; /* U+22A4 DOWN TACK */
pub const upshoe: Keysym = 0x0bc3; /*(U+2229 INTERSECTION)*/
pub const downstile: Keysym = 0x0bc4; /* U+230A LEFT FLOOR */
pub const underbar: Keysym = 0x0bc6; /*(U+005F LOW LINE)*/
pub const jot: Keysym = 0x0bca; /* U+2218 RING OPERATOR */
pub const quad: Keysym = 0x0bcc; /* U+2395 APL FUNCTIONAL SYMBOL QUAD */
pub const uptack: Keysym = 0x0bce; /* U+22A5 UP TACK */
pub const circle: Keysym = 0x0bcf; /* U+25CB WHITE CIRCLE */
pub const upstile: Keysym = 0x0bd3; /* U+2308 LEFT CEILING */
pub const downshoe: Keysym = 0x0bd6; /*(U+222A UNION)*/
pub const rightshoe: Keysym = 0x0bd8; /*(U+2283 SUPERSET OF)*/
pub const leftshoe: Keysym = 0x0bda; /*(U+2282 SUBSET OF)*/
pub const lefttack: Keysym = 0x0bdc; /* U+22A3 LEFT TACK */
pub const righttack: Keysym = 0x0bfc; /* U+22A2 RIGHT TACK */

/*
 * Hebrew
 * Byte 3 = 0x0c;
 */

pub const hebrew_doublelowline: Keysym = 0x0cdf; /* U+2017 DOUBLE LOW LINE */
pub const hebrew_aleph: Keysym = 0x0ce0; /* U+05D0 HEBREW LETTER ALEF */
pub const hebrew_bet: Keysym = 0x0ce1; /* U+05D1 HEBREW LETTER BET */
pub const hebrew_beth: Keysym = 0x0ce1; /* deprecated */
pub const hebrew_gimel: Keysym = 0x0ce2; /* U+05D2 HEBREW LETTER GIMEL */
pub const hebrew_gimmel: Keysym = 0x0ce2; /* deprecated */
pub const hebrew_dalet: Keysym = 0x0ce3; /* U+05D3 HEBREW LETTER DALET */
pub const hebrew_daleth: Keysym = 0x0ce3; /* deprecated */
pub const hebrew_he: Keysym = 0x0ce4; /* U+05D4 HEBREW LETTER HE */
pub const hebrew_waw: Keysym = 0x0ce5; /* U+05D5 HEBREW LETTER VAV */
pub const hebrew_zain: Keysym = 0x0ce6; /* U+05D6 HEBREW LETTER ZAYIN */
pub const hebrew_zayin: Keysym = 0x0ce6; /* deprecated */
pub const hebrew_chet: Keysym = 0x0ce7; /* U+05D7 HEBREW LETTER HET */
pub const hebrew_het: Keysym = 0x0ce7; /* deprecated */
pub const hebrew_tet: Keysym = 0x0ce8; /* U+05D8 HEBREW LETTER TET */
pub const hebrew_teth: Keysym = 0x0ce8; /* deprecated */
pub const hebrew_yod: Keysym = 0x0ce9; /* U+05D9 HEBREW LETTER YOD */
pub const hebrew_finalkaph: Keysym = 0x0cea; /* U+05DA HEBREW LETTER FINAL KAF */
pub const hebrew_kaph: Keysym = 0x0ceb; /* U+05DB HEBREW LETTER KAF */
pub const hebrew_lamed: Keysym = 0x0cec; /* U+05DC HEBREW LETTER LAMED */
pub const hebrew_finalmem: Keysym = 0x0ced; /* U+05DD HEBREW LETTER FINAL MEM */
pub const hebrew_mem: Keysym = 0x0cee; /* U+05DE HEBREW LETTER MEM */
pub const hebrew_finalnun: Keysym = 0x0cef; /* U+05DF HEBREW LETTER FINAL NUN */
pub const hebrew_nun: Keysym = 0x0cf0; /* U+05E0 HEBREW LETTER NUN */
pub const hebrew_samech: Keysym = 0x0cf1; /* U+05E1 HEBREW LETTER SAMEKH */
pub const hebrew_samekh: Keysym = 0x0cf1; /* deprecated */
pub const hebrew_ayin: Keysym = 0x0cf2; /* U+05E2 HEBREW LETTER AYIN */
pub const hebrew_finalpe: Keysym = 0x0cf3; /* U+05E3 HEBREW LETTER FINAL PE */
pub const hebrew_pe: Keysym = 0x0cf4; /* U+05E4 HEBREW LETTER PE */
pub const hebrew_finalzade: Keysym = 0x0cf5; /* U+05E5 HEBREW LETTER FINAL TSADI */
pub const hebrew_finalzadi: Keysym = 0x0cf5; /* deprecated */
pub const hebrew_zade: Keysym = 0x0cf6; /* U+05E6 HEBREW LETTER TSADI */
pub const hebrew_zadi: Keysym = 0x0cf6; /* deprecated */
pub const hebrew_qoph: Keysym = 0x0cf7; /* U+05E7 HEBREW LETTER QOF */
pub const hebrew_kuf: Keysym = 0x0cf7; /* deprecated */
pub const hebrew_resh: Keysym = 0x0cf8; /* U+05E8 HEBREW LETTER RESH */
pub const hebrew_shin: Keysym = 0x0cf9; /* U+05E9 HEBREW LETTER SHIN */
pub const hebrew_taw: Keysym = 0x0cfa; /* U+05EA HEBREW LETTER TAV */
pub const hebrew_taf: Keysym = 0x0cfa; /* deprecated */
pub const Hebrew_switch: Keysym = 0xff7e; /* Alias for mode_switch */

/*
 * Thai
 * Byte 3 = 0x0d;
 */

pub const Thai_kokai: Keysym = 0x0da1; /* U+0E01 THAI CHARACTER KO KAI */
pub const Thai_khokhai: Keysym = 0x0da2; /* U+0E02 THAI CHARACTER KHO KHAI */
pub const Thai_khokhuat: Keysym = 0x0da3; /* U+0E03 THAI CHARACTER KHO KHUAT */
pub const Thai_khokhwai: Keysym = 0x0da4; /* U+0E04 THAI CHARACTER KHO KHWAI */
pub const Thai_khokhon: Keysym = 0x0da5; /* U+0E05 THAI CHARACTER KHO KHON */
pub const Thai_khorakhang: Keysym = 0x0da6; /* U+0E06 THAI CHARACTER KHO RAKHANG */
pub const Thai_ngongu: Keysym = 0x0da7; /* U+0E07 THAI CHARACTER NGO NGU */
pub const Thai_chochan: Keysym = 0x0da8; /* U+0E08 THAI CHARACTER CHO CHAN */
pub const Thai_choching: Keysym = 0x0da9; /* U+0E09 THAI CHARACTER CHO CHING */
pub const Thai_chochang: Keysym = 0x0daa; /* U+0E0A THAI CHARACTER CHO CHANG */
pub const Thai_soso: Keysym = 0x0dab; /* U+0E0B THAI CHARACTER SO SO */
pub const Thai_chochoe: Keysym = 0x0dac; /* U+0E0C THAI CHARACTER CHO CHOE */
pub const Thai_yoying: Keysym = 0x0dad; /* U+0E0D THAI CHARACTER YO YING */
pub const Thai_dochada: Keysym = 0x0dae; /* U+0E0E THAI CHARACTER DO CHADA */
pub const Thai_topatak: Keysym = 0x0daf; /* U+0E0F THAI CHARACTER TO PATAK */
pub const Thai_thothan: Keysym = 0x0db0; /* U+0E10 THAI CHARACTER THO THAN */
pub const Thai_thonangmontho: Keysym = 0x0db1; /* U+0E11 THAI CHARACTER THO NANGMONTHO */
pub const Thai_thophuthao: Keysym = 0x0db2; /* U+0E12 THAI CHARACTER THO PHUTHAO */
pub const Thai_nonen: Keysym = 0x0db3; /* U+0E13 THAI CHARACTER NO NEN */
pub const Thai_dodek: Keysym = 0x0db4; /* U+0E14 THAI CHARACTER DO DEK */
pub const Thai_totao: Keysym = 0x0db5; /* U+0E15 THAI CHARACTER TO TAO */
pub const Thai_thothung: Keysym = 0x0db6; /* U+0E16 THAI CHARACTER THO THUNG */
pub const Thai_thothahan: Keysym = 0x0db7; /* U+0E17 THAI CHARACTER THO THAHAN */
pub const Thai_thothong: Keysym = 0x0db8; /* U+0E18 THAI CHARACTER THO THONG */
pub const Thai_nonu: Keysym = 0x0db9; /* U+0E19 THAI CHARACTER NO NU */
pub const Thai_bobaimai: Keysym = 0x0dba; /* U+0E1A THAI CHARACTER BO BAIMAI */
pub const Thai_popla: Keysym = 0x0dbb; /* U+0E1B THAI CHARACTER PO PLA */
pub const Thai_phophung: Keysym = 0x0dbc; /* U+0E1C THAI CHARACTER PHO PHUNG */
pub const Thai_fofa: Keysym = 0x0dbd; /* U+0E1D THAI CHARACTER FO FA */
pub const Thai_phophan: Keysym = 0x0dbe; /* U+0E1E THAI CHARACTER PHO PHAN */
pub const Thai_fofan: Keysym = 0x0dbf; /* U+0E1F THAI CHARACTER FO FAN */
pub const Thai_phosamphao: Keysym = 0x0dc0; /* U+0E20 THAI CHARACTER PHO SAMPHAO */
pub const Thai_moma: Keysym = 0x0dc1; /* U+0E21 THAI CHARACTER MO MA */
pub const Thai_yoyak: Keysym = 0x0dc2; /* U+0E22 THAI CHARACTER YO YAK */
pub const Thai_rorua: Keysym = 0x0dc3; /* U+0E23 THAI CHARACTER RO RUA */
pub const Thai_ru: Keysym = 0x0dc4; /* U+0E24 THAI CHARACTER RU */
pub const Thai_loling: Keysym = 0x0dc5; /* U+0E25 THAI CHARACTER LO LING */
pub const Thai_lu: Keysym = 0x0dc6; /* U+0E26 THAI CHARACTER LU */
pub const Thai_wowaen: Keysym = 0x0dc7; /* U+0E27 THAI CHARACTER WO WAEN */
pub const Thai_sosala: Keysym = 0x0dc8; /* U+0E28 THAI CHARACTER SO SALA */
pub const Thai_sorusi: Keysym = 0x0dc9; /* U+0E29 THAI CHARACTER SO RUSI */
pub const Thai_sosua: Keysym = 0x0dca; /* U+0E2A THAI CHARACTER SO SUA */
pub const Thai_hohip: Keysym = 0x0dcb; /* U+0E2B THAI CHARACTER HO HIP */
pub const Thai_lochula: Keysym = 0x0dcc; /* U+0E2C THAI CHARACTER LO CHULA */
pub const Thai_oang: Keysym = 0x0dcd; /* U+0E2D THAI CHARACTER O ANG */
pub const Thai_honokhuk: Keysym = 0x0dce; /* U+0E2E THAI CHARACTER HO NOKHUK */
pub const Thai_paiyannoi: Keysym = 0x0dcf; /* U+0E2F THAI CHARACTER PAIYANNOI */
pub const Thai_saraa: Keysym = 0x0dd0; /* U+0E30 THAI CHARACTER SARA A */
pub const Thai_maihanakat: Keysym = 0x0dd1; /* U+0E31 THAI CHARACTER MAI HAN-AKAT */
pub const Thai_saraaa: Keysym = 0x0dd2; /* U+0E32 THAI CHARACTER SARA AA */
pub const Thai_saraam: Keysym = 0x0dd3; /* U+0E33 THAI CHARACTER SARA AM */
pub const Thai_sarai: Keysym = 0x0dd4; /* U+0E34 THAI CHARACTER SARA I */
pub const Thai_saraii: Keysym = 0x0dd5; /* U+0E35 THAI CHARACTER SARA II */
pub const Thai_saraue: Keysym = 0x0dd6; /* U+0E36 THAI CHARACTER SARA UE */
pub const Thai_sarauee: Keysym = 0x0dd7; /* U+0E37 THAI CHARACTER SARA UEE */
pub const Thai_sarau: Keysym = 0x0dd8; /* U+0E38 THAI CHARACTER SARA U */
pub const Thai_sarauu: Keysym = 0x0dd9; /* U+0E39 THAI CHARACTER SARA UU */
pub const Thai_phinthu: Keysym = 0x0dda; /* U+0E3A THAI CHARACTER PHINTHU */
pub const Thai_maihanakat_maitho: Keysym = 0x0dde;
pub const Thai_baht: Keysym = 0x0ddf; /* U+0E3F THAI CURRENCY SYMBOL BAHT */
pub const Thai_sarae: Keysym = 0x0de0; /* U+0E40 THAI CHARACTER SARA E */
pub const Thai_saraae: Keysym = 0x0de1; /* U+0E41 THAI CHARACTER SARA AE */
pub const Thai_sarao: Keysym = 0x0de2; /* U+0E42 THAI CHARACTER SARA O */
pub const Thai_saraaimaimuan: Keysym = 0x0de3; /* U+0E43 THAI CHARACTER SARA AI MAIMUAN */
pub const Thai_saraaimaimalai: Keysym = 0x0de4; /* U+0E44 THAI CHARACTER SARA AI MAIMALAI */
pub const Thai_lakkhangyao: Keysym = 0x0de5; /* U+0E45 THAI CHARACTER LAKKHANGYAO */
pub const Thai_maiyamok: Keysym = 0x0de6; /* U+0E46 THAI CHARACTER MAIYAMOK */
pub const Thai_maitaikhu: Keysym = 0x0de7; /* U+0E47 THAI CHARACTER MAITAIKHU */
pub const Thai_maiek: Keysym = 0x0de8; /* U+0E48 THAI CHARACTER MAI EK */
pub const Thai_maitho: Keysym = 0x0de9; /* U+0E49 THAI CHARACTER MAI THO */
pub const Thai_maitri: Keysym = 0x0dea; /* U+0E4A THAI CHARACTER MAI TRI */
pub const Thai_maichattawa: Keysym = 0x0deb; /* U+0E4B THAI CHARACTER MAI CHATTAWA */
pub const Thai_thanthakhat: Keysym = 0x0dec; /* U+0E4C THAI CHARACTER THANTHAKHAT */
pub const Thai_nikhahit: Keysym = 0x0ded; /* U+0E4D THAI CHARACTER NIKHAHIT */
pub const Thai_leksun: Keysym = 0x0df0; /* U+0E50 THAI DIGIT ZERO */
pub const Thai_leknung: Keysym = 0x0df1; /* U+0E51 THAI DIGIT ONE */
pub const Thai_leksong: Keysym = 0x0df2; /* U+0E52 THAI DIGIT TWO */
pub const Thai_leksam: Keysym = 0x0df3; /* U+0E53 THAI DIGIT THREE */
pub const Thai_leksi: Keysym = 0x0df4; /* U+0E54 THAI DIGIT FOUR */
pub const Thai_lekha: Keysym = 0x0df5; /* U+0E55 THAI DIGIT FIVE */
pub const Thai_lekhok: Keysym = 0x0df6; /* U+0E56 THAI DIGIT SIX */
pub const Thai_lekchet: Keysym = 0x0df7; /* U+0E57 THAI DIGIT SEVEN */
pub const Thai_lekpaet: Keysym = 0x0df8; /* U+0E58 THAI DIGIT EIGHT */
pub const Thai_lekkao: Keysym = 0x0df9; /* U+0E59 THAI DIGIT NINE */

/*
 * Korean
 * Byte 3 = 0x0e;
 */

pub const Hangul: Keysym = 0xff31; /* Hangul start/stop(toggle) */
pub const Hangul_Start: Keysym = 0xff32; /* Hangul start */
pub const Hangul_End: Keysym = 0xff33; /* Hangul end, English start */
pub const Hangul_Hanja: Keysym = 0xff34; /* Start Hangul->Hanja Conversion */
pub const Hangul_Jamo: Keysym = 0xff35; /* Hangul Jamo mode */
pub const Hangul_Romaja: Keysym = 0xff36; /* Hangul Romaja mode */
pub const Hangul_Codeinput: Keysym = 0xff37; /* Hangul code input mode */
pub const Hangul_Jeonja: Keysym = 0xff38; /* Jeonja mode */
pub const Hangul_Banja: Keysym = 0xff39; /* Banja mode */
pub const Hangul_PreHanja: Keysym = 0xff3a; /* Pre Hanja conversion */
pub const Hangul_PostHanja: Keysym = 0xff3b; /* Post Hanja conversion */
pub const Hangul_SingleCandidate: Keysym = 0xff3c; /* Single candidate */
pub const Hangul_MultipleCandidate: Keysym = 0xff3d; /* Multiple candidate */
pub const Hangul_PreviousCandidate: Keysym = 0xff3e; /* Previous candidate */
pub const Hangul_Special: Keysym = 0xff3f; /* Special symbols */
pub const Hangul_switch: Keysym = 0xff7e; /* Alias for mode_switch */

/* Hangul Consonant Characters */
pub const Hangul_Kiyeog: Keysym = 0x0ea1; /* U+3131 HANGUL LETTER KIYEOK */
pub const Hangul_SsangKiyeog: Keysym = 0x0ea2; /* U+3132 HANGUL LETTER SSANGKIYEOK */
pub const Hangul_KiyeogSios: Keysym = 0x0ea3; /* U+3133 HANGUL LETTER KIYEOK-SIOS */
pub const Hangul_Nieun: Keysym = 0x0ea4; /* U+3134 HANGUL LETTER NIEUN */
pub const Hangul_NieunJieuj: Keysym = 0x0ea5; /* U+3135 HANGUL LETTER NIEUN-CIEUC */
pub const Hangul_NieunHieuh: Keysym = 0x0ea6; /* U+3136 HANGUL LETTER NIEUN-HIEUH */
pub const Hangul_Dikeud: Keysym = 0x0ea7; /* U+3137 HANGUL LETTER TIKEUT */
pub const Hangul_SsangDikeud: Keysym = 0x0ea8; /* U+3138 HANGUL LETTER SSANGTIKEUT */
pub const Hangul_Rieul: Keysym = 0x0ea9; /* U+3139 HANGUL LETTER RIEUL */
pub const Hangul_RieulKiyeog: Keysym = 0x0eaa; /* U+313A HANGUL LETTER RIEUL-KIYEOK */
pub const Hangul_RieulMieum: Keysym = 0x0eab; /* U+313B HANGUL LETTER RIEUL-MIEUM */
pub const Hangul_RieulPieub: Keysym = 0x0eac; /* U+313C HANGUL LETTER RIEUL-PIEUP */
pub const Hangul_RieulSios: Keysym = 0x0ead; /* U+313D HANGUL LETTER RIEUL-SIOS */
pub const Hangul_RieulTieut: Keysym = 0x0eae; /* U+313E HANGUL LETTER RIEUL-THIEUTH */
pub const Hangul_RieulPhieuf: Keysym = 0x0eaf; /* U+313F HANGUL LETTER RIEUL-PHIEUPH */
pub const Hangul_RieulHieuh: Keysym = 0x0eb0; /* U+3140 HANGUL LETTER RIEUL-HIEUH */
pub const Hangul_Mieum: Keysym = 0x0eb1; /* U+3141 HANGUL LETTER MIEUM */
pub const Hangul_Pieub: Keysym = 0x0eb2; /* U+3142 HANGUL LETTER PIEUP */
pub const Hangul_SsangPieub: Keysym = 0x0eb3; /* U+3143 HANGUL LETTER SSANGPIEUP */
pub const Hangul_PieubSios: Keysym = 0x0eb4; /* U+3144 HANGUL LETTER PIEUP-SIOS */
pub const Hangul_Sios: Keysym = 0x0eb5; /* U+3145 HANGUL LETTER SIOS */
pub const Hangul_SsangSios: Keysym = 0x0eb6; /* U+3146 HANGUL LETTER SSANGSIOS */
pub const Hangul_Ieung: Keysym = 0x0eb7; /* U+3147 HANGUL LETTER IEUNG */
pub const Hangul_Jieuj: Keysym = 0x0eb8; /* U+3148 HANGUL LETTER CIEUC */
pub const Hangul_SsangJieuj: Keysym = 0x0eb9; /* U+3149 HANGUL LETTER SSANGCIEUC */
pub const Hangul_Cieuc: Keysym = 0x0eba; /* U+314A HANGUL LETTER CHIEUCH */
pub const Hangul_Khieuq: Keysym = 0x0ebb; /* U+314B HANGUL LETTER KHIEUKH */
pub const Hangul_Tieut: Keysym = 0x0ebc; /* U+314C HANGUL LETTER THIEUTH */
pub const Hangul_Phieuf: Keysym = 0x0ebd; /* U+314D HANGUL LETTER PHIEUPH */
pub const Hangul_Hieuh: Keysym = 0x0ebe; /* U+314E HANGUL LETTER HIEUH */

/* Hangul Vowel Characters */
pub const Hangul_A: Keysym = 0x0ebf; /* U+314F HANGUL LETTER A */
pub const Hangul_AE: Keysym = 0x0ec0; /* U+3150 HANGUL LETTER AE */
pub const Hangul_YA: Keysym = 0x0ec1; /* U+3151 HANGUL LETTER YA */
pub const Hangul_YAE: Keysym = 0x0ec2; /* U+3152 HANGUL LETTER YAE */
pub const Hangul_EO: Keysym = 0x0ec3; /* U+3153 HANGUL LETTER EO */
pub const Hangul_E: Keysym = 0x0ec4; /* U+3154 HANGUL LETTER E */
pub const Hangul_YEO: Keysym = 0x0ec5; /* U+3155 HANGUL LETTER YEO */
pub const Hangul_YE: Keysym = 0x0ec6; /* U+3156 HANGUL LETTER YE */
pub const Hangul_O: Keysym = 0x0ec7; /* U+3157 HANGUL LETTER O */
pub const Hangul_WA: Keysym = 0x0ec8; /* U+3158 HANGUL LETTER WA */
pub const Hangul_WAE: Keysym = 0x0ec9; /* U+3159 HANGUL LETTER WAE */
pub const Hangul_OE: Keysym = 0x0eca; /* U+315A HANGUL LETTER OE */
pub const Hangul_YO: Keysym = 0x0ecb; /* U+315B HANGUL LETTER YO */
pub const Hangul_U: Keysym = 0x0ecc; /* U+315C HANGUL LETTER U */
pub const Hangul_WEO: Keysym = 0x0ecd; /* U+315D HANGUL LETTER WEO */
pub const Hangul_WE: Keysym = 0x0ece; /* U+315E HANGUL LETTER WE */
pub const Hangul_WI: Keysym = 0x0ecf; /* U+315F HANGUL LETTER WI */
pub const Hangul_YU: Keysym = 0x0ed0; /* U+3160 HANGUL LETTER YU */
pub const Hangul_EU: Keysym = 0x0ed1; /* U+3161 HANGUL LETTER EU */
pub const Hangul_YI: Keysym = 0x0ed2; /* U+3162 HANGUL LETTER YI */
pub const Hangul_I: Keysym = 0x0ed3; /* U+3163 HANGUL LETTER I */

/* Hangul syllable-final (JongSeong) Characters */
pub const Hangul_J_Kiyeog: Keysym = 0x0ed4; /* U+11A8 HANGUL JONGSEONG KIYEOK */
pub const Hangul_J_SsangKiyeog: Keysym = 0x0ed5; /* U+11A9 HANGUL JONGSEONG SSANGKIYEOK */
pub const Hangul_J_KiyeogSios: Keysym = 0x0ed6; /* U+11AA HANGUL JONGSEONG KIYEOK-SIOS */
pub const Hangul_J_Nieun: Keysym = 0x0ed7; /* U+11AB HANGUL JONGSEONG NIEUN */
pub const Hangul_J_NieunJieuj: Keysym = 0x0ed8; /* U+11AC HANGUL JONGSEONG NIEUN-CIEUC */
pub const Hangul_J_NieunHieuh: Keysym = 0x0ed9; /* U+11AD HANGUL JONGSEONG NIEUN-HIEUH */
pub const Hangul_J_Dikeud: Keysym = 0x0eda; /* U+11AE HANGUL JONGSEONG TIKEUT */
pub const Hangul_J_Rieul: Keysym = 0x0edb; /* U+11AF HANGUL JONGSEONG RIEUL */
pub const Hangul_J_RieulKiyeog: Keysym = 0x0edc; /* U+11B0 HANGUL JONGSEONG RIEUL-KIYEOK */
pub const Hangul_J_RieulMieum: Keysym = 0x0edd; /* U+11B1 HANGUL JONGSEONG RIEUL-MIEUM */
pub const Hangul_J_RieulPieub: Keysym = 0x0ede; /* U+11B2 HANGUL JONGSEONG RIEUL-PIEUP */
pub const Hangul_J_RieulSios: Keysym = 0x0edf; /* U+11B3 HANGUL JONGSEONG RIEUL-SIOS */
pub const Hangul_J_RieulTieut: Keysym = 0x0ee0; /* U+11B4 HANGUL JONGSEONG RIEUL-THIEUTH */
pub const Hangul_J_RieulPhieuf: Keysym = 0x0ee1; /* U+11B5 HANGUL JONGSEONG RIEUL-PHIEUPH */
pub const Hangul_J_RieulHieuh: Keysym = 0x0ee2; /* U+11B6 HANGUL JONGSEONG RIEUL-HIEUH */
pub const Hangul_J_Mieum: Keysym = 0x0ee3; /* U+11B7 HANGUL JONGSEONG MIEUM */
pub const Hangul_J_Pieub: Keysym = 0x0ee4; /* U+11B8 HANGUL JONGSEONG PIEUP */
pub const Hangul_J_PieubSios: Keysym = 0x0ee5; /* U+11B9 HANGUL JONGSEONG PIEUP-SIOS */
pub const Hangul_J_Sios: Keysym = 0x0ee6; /* U+11BA HANGUL JONGSEONG SIOS */
pub const Hangul_J_SsangSios: Keysym = 0x0ee7; /* U+11BB HANGUL JONGSEONG SSANGSIOS */
pub const Hangul_J_Ieung: Keysym = 0x0ee8; /* U+11BC HANGUL JONGSEONG IEUNG */
pub const Hangul_J_Jieuj: Keysym = 0x0ee9; /* U+11BD HANGUL JONGSEONG CIEUC */
pub const Hangul_J_Cieuc: Keysym = 0x0eea; /* U+11BE HANGUL JONGSEONG CHIEUCH */
pub const Hangul_J_Khieuq: Keysym = 0x0eeb; /* U+11BF HANGUL JONGSEONG KHIEUKH */
pub const Hangul_J_Tieut: Keysym = 0x0eec; /* U+11C0 HANGUL JONGSEONG THIEUTH */
pub const Hangul_J_Phieuf: Keysym = 0x0eed; /* U+11C1 HANGUL JONGSEONG PHIEUPH */
pub const Hangul_J_Hieuh: Keysym = 0x0eee; /* U+11C2 HANGUL JONGSEONG HIEUH */

/* Ancient Hangul Consonant Characters */
pub const Hangul_RieulYeorinHieuh: Keysym = 0x0eef; /* U+316D HANGUL LETTER RIEUL-YEORINHIEUH */
pub const Hangul_SunkyeongeumMieum: Keysym = 0x0ef0; /* U+3171 HANGUL LETTER KAPYEOUNMIEUM */
pub const Hangul_SunkyeongeumPieub: Keysym = 0x0ef1; /* U+3178 HANGUL LETTER KAPYEOUNPIEUP */
pub const Hangul_PanSios: Keysym = 0x0ef2; /* U+317F HANGUL LETTER PANSIOS */
pub const Hangul_KkogjiDalrinIeung: Keysym = 0x0ef3; /* U+3181 HANGUL LETTER YESIEUNG */
pub const Hangul_SunkyeongeumPhieuf: Keysym = 0x0ef4; /* U+3184 HANGUL LETTER KAPYEOUNPHIEUPH */
pub const Hangul_YeorinHieuh: Keysym = 0x0ef5; /* U+3186 HANGUL LETTER YEORINHIEUH */

/* Ancient Hangul Vowel Characters */
pub const Hangul_AraeA: Keysym = 0x0ef6; /* U+318D HANGUL LETTER ARAEA */
pub const Hangul_AraeAE: Keysym = 0x0ef7; /* U+318E HANGUL LETTER ARAEAE */

/* Ancient Hangul syllable-final (JongSeong) Characters */
pub const Hangul_J_PanSios: Keysym = 0x0ef8; /* U+11EB HANGUL JONGSEONG PANSIOS */
pub const Hangul_J_KkogjiDalrinIeung: Keysym = 0x0ef9; /* U+11F0 HANGUL JONGSEONG YESIEUNG */
pub const Hangul_J_YeorinHieuh: Keysym = 0x0efa; /* U+11F9 HANGUL JONGSEONG YEORINHIEUH */

/* Korean currency symbol */
pub const Korean_Won: Keysym = 0x0eff; /*(U+20A9 WON SIGN)*/

/*
 * Armenian
 */

pub const Armenian_ligature_ew: Keysym = 0x1000587; /* U+0587 ARMENIAN SMALL LIGATURE ECH YIWN */
pub const Armenian_full_stop: Keysym = 0x1000589; /* U+0589 ARMENIAN FULL STOP */
pub const Armenian_verjaket: Keysym = 0x1000589; /* U+0589 ARMENIAN FULL STOP */
pub const Armenian_separation_mark: Keysym = 0x100055d; /* U+055D ARMENIAN COMMA */
pub const Armenian_but: Keysym = 0x100055d; /* U+055D ARMENIAN COMMA */
pub const Armenian_hyphen: Keysym = 0x100058a; /* U+058A ARMENIAN HYPHEN */
pub const Armenian_yentamna: Keysym = 0x100058a; /* U+058A ARMENIAN HYPHEN */
pub const Armenian_exclam: Keysym = 0x100055c; /* U+055C ARMENIAN EXCLAMATION MARK */
pub const Armenian_amanak: Keysym = 0x100055c; /* U+055C ARMENIAN EXCLAMATION MARK */
pub const Armenian_accent: Keysym = 0x100055b; /* U+055B ARMENIAN EMPHASIS MARK */
pub const Armenian_shesht: Keysym = 0x100055b; /* U+055B ARMENIAN EMPHASIS MARK */
pub const Armenian_question: Keysym = 0x100055e; /* U+055E ARMENIAN QUESTION MARK */
pub const Armenian_paruyk: Keysym = 0x100055e; /* U+055E ARMENIAN QUESTION MARK */
pub const Armenian_AYB: Keysym = 0x1000531; /* U+0531 ARMENIAN CAPITAL LETTER AYB */
pub const Armenian_ayb: Keysym = 0x1000561; /* U+0561 ARMENIAN SMALL LETTER AYB */
pub const Armenian_BEN: Keysym = 0x1000532; /* U+0532 ARMENIAN CAPITAL LETTER BEN */
pub const Armenian_ben: Keysym = 0x1000562; /* U+0562 ARMENIAN SMALL LETTER BEN */
pub const Armenian_GIM: Keysym = 0x1000533; /* U+0533 ARMENIAN CAPITAL LETTER GIM */
pub const Armenian_gim: Keysym = 0x1000563; /* U+0563 ARMENIAN SMALL LETTER GIM */
pub const Armenian_DA: Keysym = 0x1000534; /* U+0534 ARMENIAN CAPITAL LETTER DA */
pub const Armenian_da: Keysym = 0x1000564; /* U+0564 ARMENIAN SMALL LETTER DA */
pub const Armenian_YECH: Keysym = 0x1000535; /* U+0535 ARMENIAN CAPITAL LETTER ECH */
pub const Armenian_yech: Keysym = 0x1000565; /* U+0565 ARMENIAN SMALL LETTER ECH */
pub const Armenian_ZA: Keysym = 0x1000536; /* U+0536 ARMENIAN CAPITAL LETTER ZA */
pub const Armenian_za: Keysym = 0x1000566; /* U+0566 ARMENIAN SMALL LETTER ZA */
pub const Armenian_E: Keysym = 0x1000537; /* U+0537 ARMENIAN CAPITAL LETTER EH */
pub const Armenian_e: Keysym = 0x1000567; /* U+0567 ARMENIAN SMALL LETTER EH */
pub const Armenian_AT: Keysym = 0x1000538; /* U+0538 ARMENIAN CAPITAL LETTER ET */
pub const Armenian_at: Keysym = 0x1000568; /* U+0568 ARMENIAN SMALL LETTER ET */
pub const Armenian_TO: Keysym = 0x1000539; /* U+0539 ARMENIAN CAPITAL LETTER TO */
pub const Armenian_to: Keysym = 0x1000569; /* U+0569 ARMENIAN SMALL LETTER TO */
pub const Armenian_ZHE: Keysym = 0x100053a; /* U+053A ARMENIAN CAPITAL LETTER ZHE */
pub const Armenian_zhe: Keysym = 0x100056a; /* U+056A ARMENIAN SMALL LETTER ZHE */
pub const Armenian_INI: Keysym = 0x100053b; /* U+053B ARMENIAN CAPITAL LETTER INI */
pub const Armenian_ini: Keysym = 0x100056b; /* U+056B ARMENIAN SMALL LETTER INI */
pub const Armenian_LYUN: Keysym = 0x100053c; /* U+053C ARMENIAN CAPITAL LETTER LIWN */
pub const Armenian_lyun: Keysym = 0x100056c; /* U+056C ARMENIAN SMALL LETTER LIWN */
pub const Armenian_KHE: Keysym = 0x100053d; /* U+053D ARMENIAN CAPITAL LETTER XEH */
pub const Armenian_khe: Keysym = 0x100056d; /* U+056D ARMENIAN SMALL LETTER XEH */
pub const Armenian_TSA: Keysym = 0x100053e; /* U+053E ARMENIAN CAPITAL LETTER CA */
pub const Armenian_tsa: Keysym = 0x100056e; /* U+056E ARMENIAN SMALL LETTER CA */
pub const Armenian_KEN: Keysym = 0x100053f; /* U+053F ARMENIAN CAPITAL LETTER KEN */
pub const Armenian_ken: Keysym = 0x100056f; /* U+056F ARMENIAN SMALL LETTER KEN */
pub const Armenian_HO: Keysym = 0x1000540; /* U+0540 ARMENIAN CAPITAL LETTER HO */
pub const Armenian_ho: Keysym = 0x1000570; /* U+0570 ARMENIAN SMALL LETTER HO */
pub const Armenian_DZA: Keysym = 0x1000541; /* U+0541 ARMENIAN CAPITAL LETTER JA */
pub const Armenian_dza: Keysym = 0x1000571; /* U+0571 ARMENIAN SMALL LETTER JA */
pub const Armenian_GHAT: Keysym = 0x1000542; /* U+0542 ARMENIAN CAPITAL LETTER GHAD */
pub const Armenian_ghat: Keysym = 0x1000572; /* U+0572 ARMENIAN SMALL LETTER GHAD */
pub const Armenian_TCHE: Keysym = 0x1000543; /* U+0543 ARMENIAN CAPITAL LETTER CHEH */
pub const Armenian_tche: Keysym = 0x1000573; /* U+0573 ARMENIAN SMALL LETTER CHEH */
pub const Armenian_MEN: Keysym = 0x1000544; /* U+0544 ARMENIAN CAPITAL LETTER MEN */
pub const Armenian_men: Keysym = 0x1000574; /* U+0574 ARMENIAN SMALL LETTER MEN */
pub const Armenian_HI: Keysym = 0x1000545; /* U+0545 ARMENIAN CAPITAL LETTER YI */
pub const Armenian_hi: Keysym = 0x1000575; /* U+0575 ARMENIAN SMALL LETTER YI */
pub const Armenian_NU: Keysym = 0x1000546; /* U+0546 ARMENIAN CAPITAL LETTER NOW */
pub const Armenian_nu: Keysym = 0x1000576; /* U+0576 ARMENIAN SMALL LETTER NOW */
pub const Armenian_SHA: Keysym = 0x1000547; /* U+0547 ARMENIAN CAPITAL LETTER SHA */
pub const Armenian_sha: Keysym = 0x1000577; /* U+0577 ARMENIAN SMALL LETTER SHA */
pub const Armenian_VO: Keysym = 0x1000548; /* U+0548 ARMENIAN CAPITAL LETTER VO */
pub const Armenian_vo: Keysym = 0x1000578; /* U+0578 ARMENIAN SMALL LETTER VO */
pub const Armenian_CHA: Keysym = 0x1000549; /* U+0549 ARMENIAN CAPITAL LETTER CHA */
pub const Armenian_cha: Keysym = 0x1000579; /* U+0579 ARMENIAN SMALL LETTER CHA */
pub const Armenian_PE: Keysym = 0x100054a; /* U+054A ARMENIAN CAPITAL LETTER PEH */
pub const Armenian_pe: Keysym = 0x100057a; /* U+057A ARMENIAN SMALL LETTER PEH */
pub const Armenian_JE: Keysym = 0x100054b; /* U+054B ARMENIAN CAPITAL LETTER JHEH */
pub const Armenian_je: Keysym = 0x100057b; /* U+057B ARMENIAN SMALL LETTER JHEH */
pub const Armenian_RA: Keysym = 0x100054c; /* U+054C ARMENIAN CAPITAL LETTER RA */
pub const Armenian_ra: Keysym = 0x100057c; /* U+057C ARMENIAN SMALL LETTER RA */
pub const Armenian_SE: Keysym = 0x100054d; /* U+054D ARMENIAN CAPITAL LETTER SEH */
pub const Armenian_se: Keysym = 0x100057d; /* U+057D ARMENIAN SMALL LETTER SEH */
pub const Armenian_VEV: Keysym = 0x100054e; /* U+054E ARMENIAN CAPITAL LETTER VEW */
pub const Armenian_vev: Keysym = 0x100057e; /* U+057E ARMENIAN SMALL LETTER VEW */
pub const Armenian_TYUN: Keysym = 0x100054f; /* U+054F ARMENIAN CAPITAL LETTER TIWN */
pub const Armenian_tyun: Keysym = 0x100057f; /* U+057F ARMENIAN SMALL LETTER TIWN */
pub const Armenian_RE: Keysym = 0x1000550; /* U+0550 ARMENIAN CAPITAL LETTER REH */
pub const Armenian_re: Keysym = 0x1000580; /* U+0580 ARMENIAN SMALL LETTER REH */
pub const Armenian_TSO: Keysym = 0x1000551; /* U+0551 ARMENIAN CAPITAL LETTER CO */
pub const Armenian_tso: Keysym = 0x1000581; /* U+0581 ARMENIAN SMALL LETTER CO */
pub const Armenian_VYUN: Keysym = 0x1000552; /* U+0552 ARMENIAN CAPITAL LETTER YIWN */
pub const Armenian_vyun: Keysym = 0x1000582; /* U+0582 ARMENIAN SMALL LETTER YIWN */
pub const Armenian_PYUR: Keysym = 0x1000553; /* U+0553 ARMENIAN CAPITAL LETTER PIWR */
pub const Armenian_pyur: Keysym = 0x1000583; /* U+0583 ARMENIAN SMALL LETTER PIWR */
pub const Armenian_KE: Keysym = 0x1000554; /* U+0554 ARMENIAN CAPITAL LETTER KEH */
pub const Armenian_ke: Keysym = 0x1000584; /* U+0584 ARMENIAN SMALL LETTER KEH */
pub const Armenian_O: Keysym = 0x1000555; /* U+0555 ARMENIAN CAPITAL LETTER OH */
pub const Armenian_o: Keysym = 0x1000585; /* U+0585 ARMENIAN SMALL LETTER OH */
pub const Armenian_FE: Keysym = 0x1000556; /* U+0556 ARMENIAN CAPITAL LETTER FEH */
pub const Armenian_fe: Keysym = 0x1000586; /* U+0586 ARMENIAN SMALL LETTER FEH */
pub const Armenian_apostrophe: Keysym = 0x100055a; /* U+055A ARMENIAN APOSTROPHE */

/*
 * Georgian
 */

pub const Georgian_an: Keysym = 0x10010d0; /* U+10D0 GEORGIAN LETTER AN */
pub const Georgian_ban: Keysym = 0x10010d1; /* U+10D1 GEORGIAN LETTER BAN */
pub const Georgian_gan: Keysym = 0x10010d2; /* U+10D2 GEORGIAN LETTER GAN */
pub const Georgian_don: Keysym = 0x10010d3; /* U+10D3 GEORGIAN LETTER DON */
pub const Georgian_en: Keysym = 0x10010d4; /* U+10D4 GEORGIAN LETTER EN */
pub const Georgian_vin: Keysym = 0x10010d5; /* U+10D5 GEORGIAN LETTER VIN */
pub const Georgian_zen: Keysym = 0x10010d6; /* U+10D6 GEORGIAN LETTER ZEN */
pub const Georgian_tan: Keysym = 0x10010d7; /* U+10D7 GEORGIAN LETTER TAN */
pub const Georgian_in: Keysym = 0x10010d8; /* U+10D8 GEORGIAN LETTER IN */
pub const Georgian_kan: Keysym = 0x10010d9; /* U+10D9 GEORGIAN LETTER KAN */
pub const Georgian_las: Keysym = 0x10010da; /* U+10DA GEORGIAN LETTER LAS */
pub const Georgian_man: Keysym = 0x10010db; /* U+10DB GEORGIAN LETTER MAN */
pub const Georgian_nar: Keysym = 0x10010dc; /* U+10DC GEORGIAN LETTER NAR */
pub const Georgian_on: Keysym = 0x10010dd; /* U+10DD GEORGIAN LETTER ON */
pub const Georgian_par: Keysym = 0x10010de; /* U+10DE GEORGIAN LETTER PAR */
pub const Georgian_zhar: Keysym = 0x10010df; /* U+10DF GEORGIAN LETTER ZHAR */
pub const Georgian_rae: Keysym = 0x10010e0; /* U+10E0 GEORGIAN LETTER RAE */
pub const Georgian_san: Keysym = 0x10010e1; /* U+10E1 GEORGIAN LETTER SAN */
pub const Georgian_tar: Keysym = 0x10010e2; /* U+10E2 GEORGIAN LETTER TAR */
pub const Georgian_un: Keysym = 0x10010e3; /* U+10E3 GEORGIAN LETTER UN */
pub const Georgian_phar: Keysym = 0x10010e4; /* U+10E4 GEORGIAN LETTER PHAR */
pub const Georgian_khar: Keysym = 0x10010e5; /* U+10E5 GEORGIAN LETTER KHAR */
pub const Georgian_ghan: Keysym = 0x10010e6; /* U+10E6 GEORGIAN LETTER GHAN */
pub const Georgian_qar: Keysym = 0x10010e7; /* U+10E7 GEORGIAN LETTER QAR */
pub const Georgian_shin: Keysym = 0x10010e8; /* U+10E8 GEORGIAN LETTER SHIN */
pub const Georgian_chin: Keysym = 0x10010e9; /* U+10E9 GEORGIAN LETTER CHIN */
pub const Georgian_can: Keysym = 0x10010ea; /* U+10EA GEORGIAN LETTER CAN */
pub const Georgian_jil: Keysym = 0x10010eb; /* U+10EB GEORGIAN LETTER JIL */
pub const Georgian_cil: Keysym = 0x10010ec; /* U+10EC GEORGIAN LETTER CIL */
pub const Georgian_char: Keysym = 0x10010ed; /* U+10ED GEORGIAN LETTER CHAR */
pub const Georgian_xan: Keysym = 0x10010ee; /* U+10EE GEORGIAN LETTER XAN */
pub const Georgian_jhan: Keysym = 0x10010ef; /* U+10EF GEORGIAN LETTER JHAN */
pub const Georgian_hae: Keysym = 0x10010f0; /* U+10F0 GEORGIAN LETTER HAE */
pub const Georgian_he: Keysym = 0x10010f1; /* U+10F1 GEORGIAN LETTER HE */
pub const Georgian_hie: Keysym = 0x10010f2; /* U+10F2 GEORGIAN LETTER HIE */
pub const Georgian_we: Keysym = 0x10010f3; /* U+10F3 GEORGIAN LETTER WE */
pub const Georgian_har: Keysym = 0x10010f4; /* U+10F4 GEORGIAN LETTER HAR */
pub const Georgian_hoe: Keysym = 0x10010f5; /* U+10F5 GEORGIAN LETTER HOE */
pub const Georgian_fi: Keysym = 0x10010f6; /* U+10F6 GEORGIAN LETTER FI */

/*
 * Azeri (and other Turkic or Caucasian languages)
 */

/* latin */
pub const Xabovedot: Keysym = 0x1001e8a; /* U+1E8A LATIN CAPITAL LETTER X WITH DOT ABOVE */
pub const Ibreve: Keysym = 0x100012c; /* U+012C LATIN CAPITAL LETTER I WITH BREVE */
pub const Zstroke: Keysym = 0x10001b5; /* U+01B5 LATIN CAPITAL LETTER Z WITH STROKE */
pub const Gcaron: Keysym = 0x10001e6; /* U+01E6 LATIN CAPITAL LETTER G WITH CARON */
pub const Ocaron: Keysym = 0x10001d1; /* U+01D1 LATIN CAPITAL LETTER O WITH CARON */
pub const Obarred: Keysym = 0x100019f; /* U+019F LATIN CAPITAL LETTER O WITH MIDDLE TILDE */
pub const xabovedot: Keysym = 0x1001e8b; /* U+1E8B LATIN SMALL LETTER X WITH DOT ABOVE */
pub const ibreve: Keysym = 0x100012d; /* U+012D LATIN SMALL LETTER I WITH BREVE */
pub const zstroke: Keysym = 0x10001b6; /* U+01B6 LATIN SMALL LETTER Z WITH STROKE */
pub const gcaron: Keysym = 0x10001e7; /* U+01E7 LATIN SMALL LETTER G WITH CARON */
pub const ocaron: Keysym = 0x10001d2; /* U+01D2 LATIN SMALL LETTER O WITH CARON */
pub const obarred: Keysym = 0x1000275; /* U+0275 LATIN SMALL LETTER BARRED O */
pub const SCHWA: Keysym = 0x100018f; /* U+018F LATIN CAPITAL LETTER SCHWA */
pub const schwa: Keysym = 0x1000259; /* U+0259 LATIN SMALL LETTER SCHWA */
pub const EZH: Keysym = 0x10001b7; /* U+01B7 LATIN CAPITAL LETTER EZH */
pub const ezh: Keysym = 0x1000292; /* U+0292 LATIN SMALL LETTER EZH */
/* those are not really Caucasus */
/* For Inupiak */
pub const Lbelowdot: Keysym = 0x1001e36; /* U+1E36 LATIN CAPITAL LETTER L WITH DOT BELOW */
pub const lbelowdot: Keysym = 0x1001e37; /* U+1E37 LATIN SMALL LETTER L WITH DOT BELOW */

/*
 * Vietnamese
 */

pub const Abelowdot: Keysym = 0x1001ea0; /* U+1EA0 LATIN CAPITAL LETTER A WITH DOT BELOW */
pub const abelowdot: Keysym = 0x1001ea1; /* U+1EA1 LATIN SMALL LETTER A WITH DOT BELOW */
pub const Ahook: Keysym = 0x1001ea2; /* U+1EA2 LATIN CAPITAL LETTER A WITH HOOK ABOVE */
pub const ahook: Keysym = 0x1001ea3; /* U+1EA3 LATIN SMALL LETTER A WITH HOOK ABOVE */
pub const Acircumflexacute: Keysym = 0x1001ea4; /* U+1EA4 LATIN CAPITAL LETTER A WITH CIRCUMFLEX AND ACUTE */
pub const acircumflexacute: Keysym = 0x1001ea5; /* U+1EA5 LATIN SMALL LETTER A WITH CIRCUMFLEX AND ACUTE */
pub const Acircumflexgrave: Keysym = 0x1001ea6; /* U+1EA6 LATIN CAPITAL LETTER A WITH CIRCUMFLEX AND GRAVE */
pub const acircumflexgrave: Keysym = 0x1001ea7; /* U+1EA7 LATIN SMALL LETTER A WITH CIRCUMFLEX AND GRAVE */
pub const Acircumflexhook: Keysym = 0x1001ea8; /* U+1EA8 LATIN CAPITAL LETTER A WITH CIRCUMFLEX AND HOOK ABOVE */
pub const acircumflexhook: Keysym = 0x1001ea9; /* U+1EA9 LATIN SMALL LETTER A WITH CIRCUMFLEX AND HOOK ABOVE */
pub const Acircumflextilde: Keysym = 0x1001eaa; /* U+1EAA LATIN CAPITAL LETTER A WITH CIRCUMFLEX AND TILDE */
pub const acircumflextilde: Keysym = 0x1001eab; /* U+1EAB LATIN SMALL LETTER A WITH CIRCUMFLEX AND TILDE */
pub const Acircumflexbelowdot: Keysym = 0x1001eac; /* U+1EAC LATIN CAPITAL LETTER A WITH CIRCUMFLEX AND DOT BELOW */
pub const acircumflexbelowdot: Keysym = 0x1001ead; /* U+1EAD LATIN SMALL LETTER A WITH CIRCUMFLEX AND DOT BELOW */
pub const Abreveacute: Keysym = 0x1001eae; /* U+1EAE LATIN CAPITAL LETTER A WITH BREVE AND ACUTE */
pub const abreveacute: Keysym = 0x1001eaf; /* U+1EAF LATIN SMALL LETTER A WITH BREVE AND ACUTE */
pub const Abrevegrave: Keysym = 0x1001eb0; /* U+1EB0 LATIN CAPITAL LETTER A WITH BREVE AND GRAVE */
pub const abrevegrave: Keysym = 0x1001eb1; /* U+1EB1 LATIN SMALL LETTER A WITH BREVE AND GRAVE */
pub const Abrevehook: Keysym = 0x1001eb2; /* U+1EB2 LATIN CAPITAL LETTER A WITH BREVE AND HOOK ABOVE */
pub const abrevehook: Keysym = 0x1001eb3; /* U+1EB3 LATIN SMALL LETTER A WITH BREVE AND HOOK ABOVE */
pub const Abrevetilde: Keysym = 0x1001eb4; /* U+1EB4 LATIN CAPITAL LETTER A WITH BREVE AND TILDE */
pub const abrevetilde: Keysym = 0x1001eb5; /* U+1EB5 LATIN SMALL LETTER A WITH BREVE AND TILDE */
pub const Abrevebelowdot: Keysym = 0x1001eb6; /* U+1EB6 LATIN CAPITAL LETTER A WITH BREVE AND DOT BELOW */
pub const abrevebelowdot: Keysym = 0x1001eb7; /* U+1EB7 LATIN SMALL LETTER A WITH BREVE AND DOT BELOW */
pub const Ebelowdot: Keysym = 0x1001eb8; /* U+1EB8 LATIN CAPITAL LETTER E WITH DOT BELOW */
pub const ebelowdot: Keysym = 0x1001eb9; /* U+1EB9 LATIN SMALL LETTER E WITH DOT BELOW */
pub const Ehook: Keysym = 0x1001eba; /* U+1EBA LATIN CAPITAL LETTER E WITH HOOK ABOVE */
pub const ehook: Keysym = 0x1001ebb; /* U+1EBB LATIN SMALL LETTER E WITH HOOK ABOVE */
pub const Etilde: Keysym = 0x1001ebc; /* U+1EBC LATIN CAPITAL LETTER E WITH TILDE */
pub const etilde: Keysym = 0x1001ebd; /* U+1EBD LATIN SMALL LETTER E WITH TILDE */
pub const Ecircumflexacute: Keysym = 0x1001ebe; /* U+1EBE LATIN CAPITAL LETTER E WITH CIRCUMFLEX AND ACUTE */
pub const ecircumflexacute: Keysym = 0x1001ebf; /* U+1EBF LATIN SMALL LETTER E WITH CIRCUMFLEX AND ACUTE */
pub const Ecircumflexgrave: Keysym = 0x1001ec0; /* U+1EC0 LATIN CAPITAL LETTER E WITH CIRCUMFLEX AND GRAVE */
pub const ecircumflexgrave: Keysym = 0x1001ec1; /* U+1EC1 LATIN SMALL LETTER E WITH CIRCUMFLEX AND GRAVE */
pub const Ecircumflexhook: Keysym = 0x1001ec2; /* U+1EC2 LATIN CAPITAL LETTER E WITH CIRCUMFLEX AND HOOK ABOVE */
pub const ecircumflexhook: Keysym = 0x1001ec3; /* U+1EC3 LATIN SMALL LETTER E WITH CIRCUMFLEX AND HOOK ABOVE */
pub const Ecircumflextilde: Keysym = 0x1001ec4; /* U+1EC4 LATIN CAPITAL LETTER E WITH CIRCUMFLEX AND TILDE */
pub const ecircumflextilde: Keysym = 0x1001ec5; /* U+1EC5 LATIN SMALL LETTER E WITH CIRCUMFLEX AND TILDE */
pub const Ecircumflexbelowdot: Keysym = 0x1001ec6; /* U+1EC6 LATIN CAPITAL LETTER E WITH CIRCUMFLEX AND DOT BELOW */
pub const ecircumflexbelowdot: Keysym = 0x1001ec7; /* U+1EC7 LATIN SMALL LETTER E WITH CIRCUMFLEX AND DOT BELOW */
pub const Ihook: Keysym = 0x1001ec8; /* U+1EC8 LATIN CAPITAL LETTER I WITH HOOK ABOVE */
pub const ihook: Keysym = 0x1001ec9; /* U+1EC9 LATIN SMALL LETTER I WITH HOOK ABOVE */
pub const Ibelowdot: Keysym = 0x1001eca; /* U+1ECA LATIN CAPITAL LETTER I WITH DOT BELOW */
pub const ibelowdot: Keysym = 0x1001ecb; /* U+1ECB LATIN SMALL LETTER I WITH DOT BELOW */
pub const Obelowdot: Keysym = 0x1001ecc; /* U+1ECC LATIN CAPITAL LETTER O WITH DOT BELOW */
pub const obelowdot: Keysym = 0x1001ecd; /* U+1ECD LATIN SMALL LETTER O WITH DOT BELOW */
pub const Ohook: Keysym = 0x1001ece; /* U+1ECE LATIN CAPITAL LETTER O WITH HOOK ABOVE */
pub const ohook: Keysym = 0x1001ecf; /* U+1ECF LATIN SMALL LETTER O WITH HOOK ABOVE */
pub const Ocircumflexacute: Keysym = 0x1001ed0; /* U+1ED0 LATIN CAPITAL LETTER O WITH CIRCUMFLEX AND ACUTE */
pub const ocircumflexacute: Keysym = 0x1001ed1; /* U+1ED1 LATIN SMALL LETTER O WITH CIRCUMFLEX AND ACUTE */
pub const Ocircumflexgrave: Keysym = 0x1001ed2; /* U+1ED2 LATIN CAPITAL LETTER O WITH CIRCUMFLEX AND GRAVE */
pub const ocircumflexgrave: Keysym = 0x1001ed3; /* U+1ED3 LATIN SMALL LETTER O WITH CIRCUMFLEX AND GRAVE */
pub const Ocircumflexhook: Keysym = 0x1001ed4; /* U+1ED4 LATIN CAPITAL LETTER O WITH CIRCUMFLEX AND HOOK ABOVE */
pub const ocircumflexhook: Keysym = 0x1001ed5; /* U+1ED5 LATIN SMALL LETTER O WITH CIRCUMFLEX AND HOOK ABOVE */
pub const Ocircumflextilde: Keysym = 0x1001ed6; /* U+1ED6 LATIN CAPITAL LETTER O WITH CIRCUMFLEX AND TILDE */
pub const ocircumflextilde: Keysym = 0x1001ed7; /* U+1ED7 LATIN SMALL LETTER O WITH CIRCUMFLEX AND TILDE */
pub const Ocircumflexbelowdot: Keysym = 0x1001ed8; /* U+1ED8 LATIN CAPITAL LETTER O WITH CIRCUMFLEX AND DOT BELOW */
pub const ocircumflexbelowdot: Keysym = 0x1001ed9; /* U+1ED9 LATIN SMALL LETTER O WITH CIRCUMFLEX AND DOT BELOW */
pub const Ohornacute: Keysym = 0x1001eda; /* U+1EDA LATIN CAPITAL LETTER O WITH HORN AND ACUTE */
pub const ohornacute: Keysym = 0x1001edb; /* U+1EDB LATIN SMALL LETTER O WITH HORN AND ACUTE */
pub const Ohorngrave: Keysym = 0x1001edc; /* U+1EDC LATIN CAPITAL LETTER O WITH HORN AND GRAVE */
pub const ohorngrave: Keysym = 0x1001edd; /* U+1EDD LATIN SMALL LETTER O WITH HORN AND GRAVE */
pub const Ohornhook: Keysym = 0x1001ede; /* U+1EDE LATIN CAPITAL LETTER O WITH HORN AND HOOK ABOVE */
pub const ohornhook: Keysym = 0x1001edf; /* U+1EDF LATIN SMALL LETTER O WITH HORN AND HOOK ABOVE */
pub const Ohorntilde: Keysym = 0x1001ee0; /* U+1EE0 LATIN CAPITAL LETTER O WITH HORN AND TILDE */
pub const ohorntilde: Keysym = 0x1001ee1; /* U+1EE1 LATIN SMALL LETTER O WITH HORN AND TILDE */
pub const Ohornbelowdot: Keysym = 0x1001ee2; /* U+1EE2 LATIN CAPITAL LETTER O WITH HORN AND DOT BELOW */
pub const ohornbelowdot: Keysym = 0x1001ee3; /* U+1EE3 LATIN SMALL LETTER O WITH HORN AND DOT BELOW */
pub const Ubelowdot: Keysym = 0x1001ee4; /* U+1EE4 LATIN CAPITAL LETTER U WITH DOT BELOW */
pub const ubelowdot: Keysym = 0x1001ee5; /* U+1EE5 LATIN SMALL LETTER U WITH DOT BELOW */
pub const Uhook: Keysym = 0x1001ee6; /* U+1EE6 LATIN CAPITAL LETTER U WITH HOOK ABOVE */
pub const uhook: Keysym = 0x1001ee7; /* U+1EE7 LATIN SMALL LETTER U WITH HOOK ABOVE */
pub const Uhornacute: Keysym = 0x1001ee8; /* U+1EE8 LATIN CAPITAL LETTER U WITH HORN AND ACUTE */
pub const uhornacute: Keysym = 0x1001ee9; /* U+1EE9 LATIN SMALL LETTER U WITH HORN AND ACUTE */
pub const Uhorngrave: Keysym = 0x1001eea; /* U+1EEA LATIN CAPITAL LETTER U WITH HORN AND GRAVE */
pub const uhorngrave: Keysym = 0x1001eeb; /* U+1EEB LATIN SMALL LETTER U WITH HORN AND GRAVE */
pub const Uhornhook: Keysym = 0x1001eec; /* U+1EEC LATIN CAPITAL LETTER U WITH HORN AND HOOK ABOVE */
pub const uhornhook: Keysym = 0x1001eed; /* U+1EED LATIN SMALL LETTER U WITH HORN AND HOOK ABOVE */
pub const Uhorntilde: Keysym = 0x1001eee; /* U+1EEE LATIN CAPITAL LETTER U WITH HORN AND TILDE */
pub const uhorntilde: Keysym = 0x1001eef; /* U+1EEF LATIN SMALL LETTER U WITH HORN AND TILDE */
pub const Uhornbelowdot: Keysym = 0x1001ef0; /* U+1EF0 LATIN CAPITAL LETTER U WITH HORN AND DOT BELOW */
pub const uhornbelowdot: Keysym = 0x1001ef1; /* U+1EF1 LATIN SMALL LETTER U WITH HORN AND DOT BELOW */
pub const Ybelowdot: Keysym = 0x1001ef4; /* U+1EF4 LATIN CAPITAL LETTER Y WITH DOT BELOW */
pub const ybelowdot: Keysym = 0x1001ef5; /* U+1EF5 LATIN SMALL LETTER Y WITH DOT BELOW */
pub const Yhook: Keysym = 0x1001ef6; /* U+1EF6 LATIN CAPITAL LETTER Y WITH HOOK ABOVE */
pub const yhook: Keysym = 0x1001ef7; /* U+1EF7 LATIN SMALL LETTER Y WITH HOOK ABOVE */
pub const Ytilde: Keysym = 0x1001ef8; /* U+1EF8 LATIN CAPITAL LETTER Y WITH TILDE */
pub const ytilde: Keysym = 0x1001ef9; /* U+1EF9 LATIN SMALL LETTER Y WITH TILDE */
pub const Ohorn: Keysym = 0x10001a0; /* U+01A0 LATIN CAPITAL LETTER O WITH HORN */
pub const ohorn: Keysym = 0x10001a1; /* U+01A1 LATIN SMALL LETTER O WITH HORN */
pub const Uhorn: Keysym = 0x10001af; /* U+01AF LATIN CAPITAL LETTER U WITH HORN */
pub const uhorn: Keysym = 0x10001b0; /* U+01B0 LATIN SMALL LETTER U WITH HORN */
pub const combining_tilde: Keysym = 0x1000303; /* U+0303 COMBINING TILDE */
pub const combining_grave: Keysym = 0x1000300; /* U+0300 COMBINING GRAVE ACCENT */
pub const combining_acute: Keysym = 0x1000301; /* U+0301 COMBINING ACUTE ACCENT */
pub const combining_hook: Keysym = 0x1000309; /* U+0309 COMBINING HOOK ABOVE */
pub const combining_belowdot: Keysym = 0x1000323; /* U+0323 COMBINING DOT BELOW */

pub const EcuSign: Keysym = 0x10020a0; /* U+20A0 EURO-CURRENCY SIGN */
pub const ColonSign: Keysym = 0x10020a1; /* U+20A1 COLON SIGN */
pub const CruzeiroSign: Keysym = 0x10020a2; /* U+20A2 CRUZEIRO SIGN */
pub const FFrancSign: Keysym = 0x10020a3; /* U+20A3 FRENCH FRANC SIGN */
pub const LiraSign: Keysym = 0x10020a4; /* U+20A4 LIRA SIGN */
pub const MillSign: Keysym = 0x10020a5; /* U+20A5 MILL SIGN */
pub const NairaSign: Keysym = 0x10020a6; /* U+20A6 NAIRA SIGN */
pub const PesetaSign: Keysym = 0x10020a7; /* U+20A7 PESETA SIGN */
pub const RupeeSign: Keysym = 0x10020a8; /* U+20A8 RUPEE SIGN */
pub const WonSign: Keysym = 0x10020a9; /* U+20A9 WON SIGN */
pub const NewSheqelSign: Keysym = 0x10020aa; /* U+20AA NEW SHEQEL SIGN */
pub const DongSign: Keysym = 0x10020ab; /* U+20AB DONG SIGN */
pub const EuroSign: Keysym = 0x20ac; /* U+20AC EURO SIGN */

/* one, two and three are constd: Keysym = above;. */
pub const zerosuperior: Keysym = 0x1002070; /* U+2070 SUPERSCRIPT ZERO */
pub const foursuperior: Keysym = 0x1002074; /* U+2074 SUPERSCRIPT FOUR */
pub const fivesuperior: Keysym = 0x1002075; /* U+2075 SUPERSCRIPT FIVE */
pub const sixsuperior: Keysym = 0x1002076; /* U+2076 SUPERSCRIPT SIX */
pub const sevensuperior: Keysym = 0x1002077; /* U+2077 SUPERSCRIPT SEVEN */
pub const eightsuperior: Keysym = 0x1002078; /* U+2078 SUPERSCRIPT EIGHT */
pub const ninesuperior: Keysym = 0x1002079; /* U+2079 SUPERSCRIPT NINE */
pub const zerosubscript: Keysym = 0x1002080; /* U+2080 SUBSCRIPT ZERO */
pub const onesubscript: Keysym = 0x1002081; /* U+2081 SUBSCRIPT ONE */
pub const twosubscript: Keysym = 0x1002082; /* U+2082 SUBSCRIPT TWO */
pub const threesubscript: Keysym = 0x1002083; /* U+2083 SUBSCRIPT THREE */
pub const foursubscript: Keysym = 0x1002084; /* U+2084 SUBSCRIPT FOUR */
pub const fivesubscript: Keysym = 0x1002085; /* U+2085 SUBSCRIPT FIVE */
pub const sixsubscript: Keysym = 0x1002086; /* U+2086 SUBSCRIPT SIX */
pub const sevensubscript: Keysym = 0x1002087; /* U+2087 SUBSCRIPT SEVEN */
pub const eightsubscript: Keysym = 0x1002088; /* U+2088 SUBSCRIPT EIGHT */
pub const ninesubscript: Keysym = 0x1002089; /* U+2089 SUBSCRIPT NINE */
pub const partdifferential: Keysym = 0x1002202; /* U+2202 PARTIAL DIFFERENTIAL */
pub const emptyset: Keysym = 0x1002205; /* U+2205 NULL SET */
pub const elementof: Keysym = 0x1002208; /* U+2208 ELEMENT OF */
pub const notelementof: Keysym = 0x1002209; /* U+2209 NOT AN ELEMENT OF */
pub const containsas: Keysym = 0x100220B; /* U+220B CONTAINS AS MEMBER */
pub const squareroot: Keysym = 0x100221A; /* U+221A SQUARE ROOT */
pub const cuberoot: Keysym = 0x100221B; /* U+221B CUBE ROOT */
pub const fourthroot: Keysym = 0x100221C; /* U+221C FOURTH ROOT */
pub const dintegral: Keysym = 0x100222C; /* U+222C DOUBLE INTEGRAL */
pub const tintegral: Keysym = 0x100222D; /* U+222D TRIPLE INTEGRAL */
pub const because: Keysym = 0x1002235; /* U+2235 BECAUSE */
pub const approxeq: Keysym = 0x1002248; /*(U+2248 ALMOST EQUAL TO)*/
pub const notapproxeq: Keysym = 0x1002247; /*(U+2247 NEITHER APPROXIMATELY NOR ACTUALLY EQUAL TO)*/
pub const notidentical: Keysym = 0x1002262; /* U+2262 NOT IDENTICAL TO */
pub const stricteq: Keysym = 0x1002263; /* U+2263 STRICTLY EQUIVALENT TO */

pub const braille_dot_1: Keysym = 0xfff1;
pub const braille_dot_2: Keysym = 0xfff2;
pub const braille_dot_3: Keysym = 0xfff3;
pub const braille_dot_4: Keysym = 0xfff4;
pub const braille_dot_5: Keysym = 0xfff5;
pub const braille_dot_6: Keysym = 0xfff6;
pub const braille_dot_7: Keysym = 0xfff7;
pub const braille_dot_8: Keysym = 0xfff8;
pub const braille_dot_9: Keysym = 0xfff9;
pub const braille_dot_10: Keysym = 0xfffa;
pub const braille_blank: Keysym = 0x1002800; /* U+2800 BRAILLE PATTERN BLANK */
pub const braille_dots_1: Keysym = 0x1002801; /* U+2801 BRAILLE PATTERN DOTS-1 */
pub const braille_dots_2: Keysym = 0x1002802; /* U+2802 BRAILLE PATTERN DOTS-2 */
pub const braille_dots_12: Keysym = 0x1002803; /* U+2803 BRAILLE PATTERN DOTS-12 */
pub const braille_dots_3: Keysym = 0x1002804; /* U+2804 BRAILLE PATTERN DOTS-3 */
pub const braille_dots_13: Keysym = 0x1002805; /* U+2805 BRAILLE PATTERN DOTS-13 */
pub const braille_dots_23: Keysym = 0x1002806; /* U+2806 BRAILLE PATTERN DOTS-23 */
pub const braille_dots_123: Keysym = 0x1002807; /* U+2807 BRAILLE PATTERN DOTS-123 */
pub const braille_dots_4: Keysym = 0x1002808; /* U+2808 BRAILLE PATTERN DOTS-4 */
pub const braille_dots_14: Keysym = 0x1002809; /* U+2809 BRAILLE PATTERN DOTS-14 */
pub const braille_dots_24: Keysym = 0x100280a; /* U+280a BRAILLE PATTERN DOTS-24 */
pub const braille_dots_124: Keysym = 0x100280b; /* U+280b BRAILLE PATTERN DOTS-124 */
pub const braille_dots_34: Keysym = 0x100280c; /* U+280c BRAILLE PATTERN DOTS-34 */
pub const braille_dots_134: Keysym = 0x100280d; /* U+280d BRAILLE PATTERN DOTS-134 */
pub const braille_dots_234: Keysym = 0x100280e; /* U+280e BRAILLE PATTERN DOTS-234 */
pub const braille_dots_1234: Keysym = 0x100280f; /* U+280f BRAILLE PATTERN DOTS-1234 */
pub const braille_dots_5: Keysym = 0x1002810; /* U+2810 BRAILLE PATTERN DOTS-5 */
pub const braille_dots_15: Keysym = 0x1002811; /* U+2811 BRAILLE PATTERN DOTS-15 */
pub const braille_dots_25: Keysym = 0x1002812; /* U+2812 BRAILLE PATTERN DOTS-25 */
pub const braille_dots_125: Keysym = 0x1002813; /* U+2813 BRAILLE PATTERN DOTS-125 */
pub const braille_dots_35: Keysym = 0x1002814; /* U+2814 BRAILLE PATTERN DOTS-35 */
pub const braille_dots_135: Keysym = 0x1002815; /* U+2815 BRAILLE PATTERN DOTS-135 */
pub const braille_dots_235: Keysym = 0x1002816; /* U+2816 BRAILLE PATTERN DOTS-235 */
pub const braille_dots_1235: Keysym = 0x1002817; /* U+2817 BRAILLE PATTERN DOTS-1235 */
pub const braille_dots_45: Keysym = 0x1002818; /* U+2818 BRAILLE PATTERN DOTS-45 */
pub const braille_dots_145: Keysym = 0x1002819; /* U+2819 BRAILLE PATTERN DOTS-145 */
pub const braille_dots_245: Keysym = 0x100281a; /* U+281a BRAILLE PATTERN DOTS-245 */
pub const braille_dots_1245: Keysym = 0x100281b; /* U+281b BRAILLE PATTERN DOTS-1245 */
pub const braille_dots_345: Keysym = 0x100281c; /* U+281c BRAILLE PATTERN DOTS-345 */
pub const braille_dots_1345: Keysym = 0x100281d; /* U+281d BRAILLE PATTERN DOTS-1345 */
pub const braille_dots_2345: Keysym = 0x100281e; /* U+281e BRAILLE PATTERN DOTS-2345 */
pub const braille_dots_12345: Keysym = 0x100281f; /* U+281f BRAILLE PATTERN DOTS-12345 */
pub const braille_dots_6: Keysym = 0x1002820; /* U+2820 BRAILLE PATTERN DOTS-6 */
pub const braille_dots_16: Keysym = 0x1002821; /* U+2821 BRAILLE PATTERN DOTS-16 */
pub const braille_dots_26: Keysym = 0x1002822; /* U+2822 BRAILLE PATTERN DOTS-26 */
pub const braille_dots_126: Keysym = 0x1002823; /* U+2823 BRAILLE PATTERN DOTS-126 */
pub const braille_dots_36: Keysym = 0x1002824; /* U+2824 BRAILLE PATTERN DOTS-36 */
pub const braille_dots_136: Keysym = 0x1002825; /* U+2825 BRAILLE PATTERN DOTS-136 */
pub const braille_dots_236: Keysym = 0x1002826; /* U+2826 BRAILLE PATTERN DOTS-236 */
pub const braille_dots_1236: Keysym = 0x1002827; /* U+2827 BRAILLE PATTERN DOTS-1236 */
pub const braille_dots_46: Keysym = 0x1002828; /* U+2828 BRAILLE PATTERN DOTS-46 */
pub const braille_dots_146: Keysym = 0x1002829; /* U+2829 BRAILLE PATTERN DOTS-146 */
pub const braille_dots_246: Keysym = 0x100282a; /* U+282a BRAILLE PATTERN DOTS-246 */
pub const braille_dots_1246: Keysym = 0x100282b; /* U+282b BRAILLE PATTERN DOTS-1246 */
pub const braille_dots_346: Keysym = 0x100282c; /* U+282c BRAILLE PATTERN DOTS-346 */
pub const braille_dots_1346: Keysym = 0x100282d; /* U+282d BRAILLE PATTERN DOTS-1346 */
pub const braille_dots_2346: Keysym = 0x100282e; /* U+282e BRAILLE PATTERN DOTS-2346 */
pub const braille_dots_12346: Keysym = 0x100282f; /* U+282f BRAILLE PATTERN DOTS-12346 */
pub const braille_dots_56: Keysym = 0x1002830; /* U+2830 BRAILLE PATTERN DOTS-56 */
pub const braille_dots_156: Keysym = 0x1002831; /* U+2831 BRAILLE PATTERN DOTS-156 */
pub const braille_dots_256: Keysym = 0x1002832; /* U+2832 BRAILLE PATTERN DOTS-256 */
pub const braille_dots_1256: Keysym = 0x1002833; /* U+2833 BRAILLE PATTERN DOTS-1256 */
pub const braille_dots_356: Keysym = 0x1002834; /* U+2834 BRAILLE PATTERN DOTS-356 */
pub const braille_dots_1356: Keysym = 0x1002835; /* U+2835 BRAILLE PATTERN DOTS-1356 */
pub const braille_dots_2356: Keysym = 0x1002836; /* U+2836 BRAILLE PATTERN DOTS-2356 */
pub const braille_dots_12356: Keysym = 0x1002837; /* U+2837 BRAILLE PATTERN DOTS-12356 */
pub const braille_dots_456: Keysym = 0x1002838; /* U+2838 BRAILLE PATTERN DOTS-456 */
pub const braille_dots_1456: Keysym = 0x1002839; /* U+2839 BRAILLE PATTERN DOTS-1456 */
pub const braille_dots_2456: Keysym = 0x100283a; /* U+283a BRAILLE PATTERN DOTS-2456 */
pub const braille_dots_12456: Keysym = 0x100283b; /* U+283b BRAILLE PATTERN DOTS-12456 */
pub const braille_dots_3456: Keysym = 0x100283c; /* U+283c BRAILLE PATTERN DOTS-3456 */
pub const braille_dots_13456: Keysym = 0x100283d; /* U+283d BRAILLE PATTERN DOTS-13456 */
pub const braille_dots_23456: Keysym = 0x100283e; /* U+283e BRAILLE PATTERN DOTS-23456 */
pub const braille_dots_123456: Keysym = 0x100283f; /* U+283f BRAILLE PATTERN DOTS-123456 */
pub const braille_dots_7: Keysym = 0x1002840; /* U+2840 BRAILLE PATTERN DOTS-7 */
pub const braille_dots_17: Keysym = 0x1002841; /* U+2841 BRAILLE PATTERN DOTS-17 */
pub const braille_dots_27: Keysym = 0x1002842; /* U+2842 BRAILLE PATTERN DOTS-27 */
pub const braille_dots_127: Keysym = 0x1002843; /* U+2843 BRAILLE PATTERN DOTS-127 */
pub const braille_dots_37: Keysym = 0x1002844; /* U+2844 BRAILLE PATTERN DOTS-37 */
pub const braille_dots_137: Keysym = 0x1002845; /* U+2845 BRAILLE PATTERN DOTS-137 */
pub const braille_dots_237: Keysym = 0x1002846; /* U+2846 BRAILLE PATTERN DOTS-237 */
pub const braille_dots_1237: Keysym = 0x1002847; /* U+2847 BRAILLE PATTERN DOTS-1237 */
pub const braille_dots_47: Keysym = 0x1002848; /* U+2848 BRAILLE PATTERN DOTS-47 */
pub const braille_dots_147: Keysym = 0x1002849; /* U+2849 BRAILLE PATTERN DOTS-147 */
pub const braille_dots_247: Keysym = 0x100284a; /* U+284a BRAILLE PATTERN DOTS-247 */
pub const braille_dots_1247: Keysym = 0x100284b; /* U+284b BRAILLE PATTERN DOTS-1247 */
pub const braille_dots_347: Keysym = 0x100284c; /* U+284c BRAILLE PATTERN DOTS-347 */
pub const braille_dots_1347: Keysym = 0x100284d; /* U+284d BRAILLE PATTERN DOTS-1347 */
pub const braille_dots_2347: Keysym = 0x100284e; /* U+284e BRAILLE PATTERN DOTS-2347 */
pub const braille_dots_12347: Keysym = 0x100284f; /* U+284f BRAILLE PATTERN DOTS-12347 */
pub const braille_dots_57: Keysym = 0x1002850; /* U+2850 BRAILLE PATTERN DOTS-57 */
pub const braille_dots_157: Keysym = 0x1002851; /* U+2851 BRAILLE PATTERN DOTS-157 */
pub const braille_dots_257: Keysym = 0x1002852; /* U+2852 BRAILLE PATTERN DOTS-257 */
pub const braille_dots_1257: Keysym = 0x1002853; /* U+2853 BRAILLE PATTERN DOTS-1257 */
pub const braille_dots_357: Keysym = 0x1002854; /* U+2854 BRAILLE PATTERN DOTS-357 */
pub const braille_dots_1357: Keysym = 0x1002855; /* U+2855 BRAILLE PATTERN DOTS-1357 */
pub const braille_dots_2357: Keysym = 0x1002856; /* U+2856 BRAILLE PATTERN DOTS-2357 */
pub const braille_dots_12357: Keysym = 0x1002857; /* U+2857 BRAILLE PATTERN DOTS-12357 */
pub const braille_dots_457: Keysym = 0x1002858; /* U+2858 BRAILLE PATTERN DOTS-457 */
pub const braille_dots_1457: Keysym = 0x1002859; /* U+2859 BRAILLE PATTERN DOTS-1457 */
pub const braille_dots_2457: Keysym = 0x100285a; /* U+285a BRAILLE PATTERN DOTS-2457 */
pub const braille_dots_12457: Keysym = 0x100285b; /* U+285b BRAILLE PATTERN DOTS-12457 */
pub const braille_dots_3457: Keysym = 0x100285c; /* U+285c BRAILLE PATTERN DOTS-3457 */
pub const braille_dots_13457: Keysym = 0x100285d; /* U+285d BRAILLE PATTERN DOTS-13457 */
pub const braille_dots_23457: Keysym = 0x100285e; /* U+285e BRAILLE PATTERN DOTS-23457 */
pub const braille_dots_123457: Keysym = 0x100285f; /* U+285f BRAILLE PATTERN DOTS-123457 */
pub const braille_dots_67: Keysym = 0x1002860; /* U+2860 BRAILLE PATTERN DOTS-67 */
pub const braille_dots_167: Keysym = 0x1002861; /* U+2861 BRAILLE PATTERN DOTS-167 */
pub const braille_dots_267: Keysym = 0x1002862; /* U+2862 BRAILLE PATTERN DOTS-267 */
pub const braille_dots_1267: Keysym = 0x1002863; /* U+2863 BRAILLE PATTERN DOTS-1267 */
pub const braille_dots_367: Keysym = 0x1002864; /* U+2864 BRAILLE PATTERN DOTS-367 */
pub const braille_dots_1367: Keysym = 0x1002865; /* U+2865 BRAILLE PATTERN DOTS-1367 */
pub const braille_dots_2367: Keysym = 0x1002866; /* U+2866 BRAILLE PATTERN DOTS-2367 */
pub const braille_dots_12367: Keysym = 0x1002867; /* U+2867 BRAILLE PATTERN DOTS-12367 */
pub const braille_dots_467: Keysym = 0x1002868; /* U+2868 BRAILLE PATTERN DOTS-467 */
pub const braille_dots_1467: Keysym = 0x1002869; /* U+2869 BRAILLE PATTERN DOTS-1467 */
pub const braille_dots_2467: Keysym = 0x100286a; /* U+286a BRAILLE PATTERN DOTS-2467 */
pub const braille_dots_12467: Keysym = 0x100286b; /* U+286b BRAILLE PATTERN DOTS-12467 */
pub const braille_dots_3467: Keysym = 0x100286c; /* U+286c BRAILLE PATTERN DOTS-3467 */
pub const braille_dots_13467: Keysym = 0x100286d; /* U+286d BRAILLE PATTERN DOTS-13467 */
pub const braille_dots_23467: Keysym = 0x100286e; /* U+286e BRAILLE PATTERN DOTS-23467 */
pub const braille_dots_123467: Keysym = 0x100286f; /* U+286f BRAILLE PATTERN DOTS-123467 */
pub const braille_dots_567: Keysym = 0x1002870; /* U+2870 BRAILLE PATTERN DOTS-567 */
pub const braille_dots_1567: Keysym = 0x1002871; /* U+2871 BRAILLE PATTERN DOTS-1567 */
pub const braille_dots_2567: Keysym = 0x1002872; /* U+2872 BRAILLE PATTERN DOTS-2567 */
pub const braille_dots_12567: Keysym = 0x1002873; /* U+2873 BRAILLE PATTERN DOTS-12567 */
pub const braille_dots_3567: Keysym = 0x1002874; /* U+2874 BRAILLE PATTERN DOTS-3567 */
pub const braille_dots_13567: Keysym = 0x1002875; /* U+2875 BRAILLE PATTERN DOTS-13567 */
pub const braille_dots_23567: Keysym = 0x1002876; /* U+2876 BRAILLE PATTERN DOTS-23567 */
pub const braille_dots_123567: Keysym = 0x1002877; /* U+2877 BRAILLE PATTERN DOTS-123567 */
pub const braille_dots_4567: Keysym = 0x1002878; /* U+2878 BRAILLE PATTERN DOTS-4567 */
pub const braille_dots_14567: Keysym = 0x1002879; /* U+2879 BRAILLE PATTERN DOTS-14567 */
pub const braille_dots_24567: Keysym = 0x100287a; /* U+287a BRAILLE PATTERN DOTS-24567 */
pub const braille_dots_124567: Keysym = 0x100287b; /* U+287b BRAILLE PATTERN DOTS-124567 */
pub const braille_dots_34567: Keysym = 0x100287c; /* U+287c BRAILLE PATTERN DOTS-34567 */
pub const braille_dots_134567: Keysym = 0x100287d; /* U+287d BRAILLE PATTERN DOTS-134567 */
pub const braille_dots_234567: Keysym = 0x100287e; /* U+287e BRAILLE PATTERN DOTS-234567 */
pub const braille_dots_1234567: Keysym = 0x100287f; /* U+287f BRAILLE PATTERN DOTS-1234567 */
pub const braille_dots_8: Keysym = 0x1002880; /* U+2880 BRAILLE PATTERN DOTS-8 */
pub const braille_dots_18: Keysym = 0x1002881; /* U+2881 BRAILLE PATTERN DOTS-18 */
pub const braille_dots_28: Keysym = 0x1002882; /* U+2882 BRAILLE PATTERN DOTS-28 */
pub const braille_dots_128: Keysym = 0x1002883; /* U+2883 BRAILLE PATTERN DOTS-128 */
pub const braille_dots_38: Keysym = 0x1002884; /* U+2884 BRAILLE PATTERN DOTS-38 */
pub const braille_dots_138: Keysym = 0x1002885; /* U+2885 BRAILLE PATTERN DOTS-138 */
pub const braille_dots_238: Keysym = 0x1002886; /* U+2886 BRAILLE PATTERN DOTS-238 */
pub const braille_dots_1238: Keysym = 0x1002887; /* U+2887 BRAILLE PATTERN DOTS-1238 */
pub const braille_dots_48: Keysym = 0x1002888; /* U+2888 BRAILLE PATTERN DOTS-48 */
pub const braille_dots_148: Keysym = 0x1002889; /* U+2889 BRAILLE PATTERN DOTS-148 */
pub const braille_dots_248: Keysym = 0x100288a; /* U+288a BRAILLE PATTERN DOTS-248 */
pub const braille_dots_1248: Keysym = 0x100288b; /* U+288b BRAILLE PATTERN DOTS-1248 */
pub const braille_dots_348: Keysym = 0x100288c; /* U+288c BRAILLE PATTERN DOTS-348 */
pub const braille_dots_1348: Keysym = 0x100288d; /* U+288d BRAILLE PATTERN DOTS-1348 */
pub const braille_dots_2348: Keysym = 0x100288e; /* U+288e BRAILLE PATTERN DOTS-2348 */
pub const braille_dots_12348: Keysym = 0x100288f; /* U+288f BRAILLE PATTERN DOTS-12348 */
pub const braille_dots_58: Keysym = 0x1002890; /* U+2890 BRAILLE PATTERN DOTS-58 */
pub const braille_dots_158: Keysym = 0x1002891; /* U+2891 BRAILLE PATTERN DOTS-158 */
pub const braille_dots_258: Keysym = 0x1002892; /* U+2892 BRAILLE PATTERN DOTS-258 */
pub const braille_dots_1258: Keysym = 0x1002893; /* U+2893 BRAILLE PATTERN DOTS-1258 */
pub const braille_dots_358: Keysym = 0x1002894; /* U+2894 BRAILLE PATTERN DOTS-358 */
pub const braille_dots_1358: Keysym = 0x1002895; /* U+2895 BRAILLE PATTERN DOTS-1358 */
pub const braille_dots_2358: Keysym = 0x1002896; /* U+2896 BRAILLE PATTERN DOTS-2358 */
pub const braille_dots_12358: Keysym = 0x1002897; /* U+2897 BRAILLE PATTERN DOTS-12358 */
pub const braille_dots_458: Keysym = 0x1002898; /* U+2898 BRAILLE PATTERN DOTS-458 */
pub const braille_dots_1458: Keysym = 0x1002899; /* U+2899 BRAILLE PATTERN DOTS-1458 */
pub const braille_dots_2458: Keysym = 0x100289a; /* U+289a BRAILLE PATTERN DOTS-2458 */
pub const braille_dots_12458: Keysym = 0x100289b; /* U+289b BRAILLE PATTERN DOTS-12458 */
pub const braille_dots_3458: Keysym = 0x100289c; /* U+289c BRAILLE PATTERN DOTS-3458 */
pub const braille_dots_13458: Keysym = 0x100289d; /* U+289d BRAILLE PATTERN DOTS-13458 */
pub const braille_dots_23458: Keysym = 0x100289e; /* U+289e BRAILLE PATTERN DOTS-23458 */
pub const braille_dots_123458: Keysym = 0x100289f; /* U+289f BRAILLE PATTERN DOTS-123458 */
pub const braille_dots_68: Keysym = 0x10028a0; /* U+28a0 BRAILLE PATTERN DOTS-68 */
pub const braille_dots_168: Keysym = 0x10028a1; /* U+28a1 BRAILLE PATTERN DOTS-168 */
pub const braille_dots_268: Keysym = 0x10028a2; /* U+28a2 BRAILLE PATTERN DOTS-268 */
pub const braille_dots_1268: Keysym = 0x10028a3; /* U+28a3 BRAILLE PATTERN DOTS-1268 */
pub const braille_dots_368: Keysym = 0x10028a4; /* U+28a4 BRAILLE PATTERN DOTS-368 */
pub const braille_dots_1368: Keysym = 0x10028a5; /* U+28a5 BRAILLE PATTERN DOTS-1368 */
pub const braille_dots_2368: Keysym = 0x10028a6; /* U+28a6 BRAILLE PATTERN DOTS-2368 */
pub const braille_dots_12368: Keysym = 0x10028a7; /* U+28a7 BRAILLE PATTERN DOTS-12368 */
pub const braille_dots_468: Keysym = 0x10028a8; /* U+28a8 BRAILLE PATTERN DOTS-468 */
pub const braille_dots_1468: Keysym = 0x10028a9; /* U+28a9 BRAILLE PATTERN DOTS-1468 */
pub const braille_dots_2468: Keysym = 0x10028aa; /* U+28aa BRAILLE PATTERN DOTS-2468 */
pub const braille_dots_12468: Keysym = 0x10028ab; /* U+28ab BRAILLE PATTERN DOTS-12468 */
pub const braille_dots_3468: Keysym = 0x10028ac; /* U+28ac BRAILLE PATTERN DOTS-3468 */
pub const braille_dots_13468: Keysym = 0x10028ad; /* U+28ad BRAILLE PATTERN DOTS-13468 */
pub const braille_dots_23468: Keysym = 0x10028ae; /* U+28ae BRAILLE PATTERN DOTS-23468 */
pub const braille_dots_123468: Keysym = 0x10028af; /* U+28af BRAILLE PATTERN DOTS-123468 */
pub const braille_dots_568: Keysym = 0x10028b0; /* U+28b0 BRAILLE PATTERN DOTS-568 */
pub const braille_dots_1568: Keysym = 0x10028b1; /* U+28b1 BRAILLE PATTERN DOTS-1568 */
pub const braille_dots_2568: Keysym = 0x10028b2; /* U+28b2 BRAILLE PATTERN DOTS-2568 */
pub const braille_dots_12568: Keysym = 0x10028b3; /* U+28b3 BRAILLE PATTERN DOTS-12568 */
pub const braille_dots_3568: Keysym = 0x10028b4; /* U+28b4 BRAILLE PATTERN DOTS-3568 */
pub const braille_dots_13568: Keysym = 0x10028b5; /* U+28b5 BRAILLE PATTERN DOTS-13568 */
pub const braille_dots_23568: Keysym = 0x10028b6; /* U+28b6 BRAILLE PATTERN DOTS-23568 */
pub const braille_dots_123568: Keysym = 0x10028b7; /* U+28b7 BRAILLE PATTERN DOTS-123568 */
pub const braille_dots_4568: Keysym = 0x10028b8; /* U+28b8 BRAILLE PATTERN DOTS-4568 */
pub const braille_dots_14568: Keysym = 0x10028b9; /* U+28b9 BRAILLE PATTERN DOTS-14568 */
pub const braille_dots_24568: Keysym = 0x10028ba; /* U+28ba BRAILLE PATTERN DOTS-24568 */
pub const braille_dots_124568: Keysym = 0x10028bb; /* U+28bb BRAILLE PATTERN DOTS-124568 */
pub const braille_dots_34568: Keysym = 0x10028bc; /* U+28bc BRAILLE PATTERN DOTS-34568 */
pub const braille_dots_134568: Keysym = 0x10028bd; /* U+28bd BRAILLE PATTERN DOTS-134568 */
pub const braille_dots_234568: Keysym = 0x10028be; /* U+28be BRAILLE PATTERN DOTS-234568 */
pub const braille_dots_1234568: Keysym = 0x10028bf; /* U+28bf BRAILLE PATTERN DOTS-1234568 */
pub const braille_dots_78: Keysym = 0x10028c0; /* U+28c0 BRAILLE PATTERN DOTS-78 */
pub const braille_dots_178: Keysym = 0x10028c1; /* U+28c1 BRAILLE PATTERN DOTS-178 */
pub const braille_dots_278: Keysym = 0x10028c2; /* U+28c2 BRAILLE PATTERN DOTS-278 */
pub const braille_dots_1278: Keysym = 0x10028c3; /* U+28c3 BRAILLE PATTERN DOTS-1278 */
pub const braille_dots_378: Keysym = 0x10028c4; /* U+28c4 BRAILLE PATTERN DOTS-378 */
pub const braille_dots_1378: Keysym = 0x10028c5; /* U+28c5 BRAILLE PATTERN DOTS-1378 */
pub const braille_dots_2378: Keysym = 0x10028c6; /* U+28c6 BRAILLE PATTERN DOTS-2378 */
pub const braille_dots_12378: Keysym = 0x10028c7; /* U+28c7 BRAILLE PATTERN DOTS-12378 */
pub const braille_dots_478: Keysym = 0x10028c8; /* U+28c8 BRAILLE PATTERN DOTS-478 */
pub const braille_dots_1478: Keysym = 0x10028c9; /* U+28c9 BRAILLE PATTERN DOTS-1478 */
pub const braille_dots_2478: Keysym = 0x10028ca; /* U+28ca BRAILLE PATTERN DOTS-2478 */
pub const braille_dots_12478: Keysym = 0x10028cb; /* U+28cb BRAILLE PATTERN DOTS-12478 */
pub const braille_dots_3478: Keysym = 0x10028cc; /* U+28cc BRAILLE PATTERN DOTS-3478 */
pub const braille_dots_13478: Keysym = 0x10028cd; /* U+28cd BRAILLE PATTERN DOTS-13478 */
pub const braille_dots_23478: Keysym = 0x10028ce; /* U+28ce BRAILLE PATTERN DOTS-23478 */
pub const braille_dots_123478: Keysym = 0x10028cf; /* U+28cf BRAILLE PATTERN DOTS-123478 */
pub const braille_dots_578: Keysym = 0x10028d0; /* U+28d0 BRAILLE PATTERN DOTS-578 */
pub const braille_dots_1578: Keysym = 0x10028d1; /* U+28d1 BRAILLE PATTERN DOTS-1578 */
pub const braille_dots_2578: Keysym = 0x10028d2; /* U+28d2 BRAILLE PATTERN DOTS-2578 */
pub const braille_dots_12578: Keysym = 0x10028d3; /* U+28d3 BRAILLE PATTERN DOTS-12578 */
pub const braille_dots_3578: Keysym = 0x10028d4; /* U+28d4 BRAILLE PATTERN DOTS-3578 */
pub const braille_dots_13578: Keysym = 0x10028d5; /* U+28d5 BRAILLE PATTERN DOTS-13578 */
pub const braille_dots_23578: Keysym = 0x10028d6; /* U+28d6 BRAILLE PATTERN DOTS-23578 */
pub const braille_dots_123578: Keysym = 0x10028d7; /* U+28d7 BRAILLE PATTERN DOTS-123578 */
pub const braille_dots_4578: Keysym = 0x10028d8; /* U+28d8 BRAILLE PATTERN DOTS-4578 */
pub const braille_dots_14578: Keysym = 0x10028d9; /* U+28d9 BRAILLE PATTERN DOTS-14578 */
pub const braille_dots_24578: Keysym = 0x10028da; /* U+28da BRAILLE PATTERN DOTS-24578 */
pub const braille_dots_124578: Keysym = 0x10028db; /* U+28db BRAILLE PATTERN DOTS-124578 */
pub const braille_dots_34578: Keysym = 0x10028dc; /* U+28dc BRAILLE PATTERN DOTS-34578 */
pub const braille_dots_134578: Keysym = 0x10028dd; /* U+28dd BRAILLE PATTERN DOTS-134578 */
pub const braille_dots_234578: Keysym = 0x10028de; /* U+28de BRAILLE PATTERN DOTS-234578 */
pub const braille_dots_1234578: Keysym = 0x10028df; /* U+28df BRAILLE PATTERN DOTS-1234578 */
pub const braille_dots_678: Keysym = 0x10028e0; /* U+28e0 BRAILLE PATTERN DOTS-678 */
pub const braille_dots_1678: Keysym = 0x10028e1; /* U+28e1 BRAILLE PATTERN DOTS-1678 */
pub const braille_dots_2678: Keysym = 0x10028e2; /* U+28e2 BRAILLE PATTERN DOTS-2678 */
pub const braille_dots_12678: Keysym = 0x10028e3; /* U+28e3 BRAILLE PATTERN DOTS-12678 */
pub const braille_dots_3678: Keysym = 0x10028e4; /* U+28e4 BRAILLE PATTERN DOTS-3678 */
pub const braille_dots_13678: Keysym = 0x10028e5; /* U+28e5 BRAILLE PATTERN DOTS-13678 */
pub const braille_dots_23678: Keysym = 0x10028e6; /* U+28e6 BRAILLE PATTERN DOTS-23678 */
pub const braille_dots_123678: Keysym = 0x10028e7; /* U+28e7 BRAILLE PATTERN DOTS-123678 */
pub const braille_dots_4678: Keysym = 0x10028e8; /* U+28e8 BRAILLE PATTERN DOTS-4678 */
pub const braille_dots_14678: Keysym = 0x10028e9; /* U+28e9 BRAILLE PATTERN DOTS-14678 */
pub const braille_dots_24678: Keysym = 0x10028ea; /* U+28ea BRAILLE PATTERN DOTS-24678 */
pub const braille_dots_124678: Keysym = 0x10028eb; /* U+28eb BRAILLE PATTERN DOTS-124678 */
pub const braille_dots_34678: Keysym = 0x10028ec; /* U+28ec BRAILLE PATTERN DOTS-34678 */
pub const braille_dots_134678: Keysym = 0x10028ed; /* U+28ed BRAILLE PATTERN DOTS-134678 */
pub const braille_dots_234678: Keysym = 0x10028ee; /* U+28ee BRAILLE PATTERN DOTS-234678 */
pub const braille_dots_1234678: Keysym = 0x10028ef; /* U+28ef BRAILLE PATTERN DOTS-1234678 */
pub const braille_dots_5678: Keysym = 0x10028f0; /* U+28f0 BRAILLE PATTERN DOTS-5678 */
pub const braille_dots_15678: Keysym = 0x10028f1; /* U+28f1 BRAILLE PATTERN DOTS-15678 */
pub const braille_dots_25678: Keysym = 0x10028f2; /* U+28f2 BRAILLE PATTERN DOTS-25678 */
pub const braille_dots_125678: Keysym = 0x10028f3; /* U+28f3 BRAILLE PATTERN DOTS-125678 */
pub const braille_dots_35678: Keysym = 0x10028f4; /* U+28f4 BRAILLE PATTERN DOTS-35678 */
pub const braille_dots_135678: Keysym = 0x10028f5; /* U+28f5 BRAILLE PATTERN DOTS-135678 */
pub const braille_dots_235678: Keysym = 0x10028f6; /* U+28f6 BRAILLE PATTERN DOTS-235678 */
pub const braille_dots_1235678: Keysym = 0x10028f7; /* U+28f7 BRAILLE PATTERN DOTS-1235678 */
pub const braille_dots_45678: Keysym = 0x10028f8; /* U+28f8 BRAILLE PATTERN DOTS-45678 */
pub const braille_dots_145678: Keysym = 0x10028f9; /* U+28f9 BRAILLE PATTERN DOTS-145678 */
pub const braille_dots_245678: Keysym = 0x10028fa; /* U+28fa BRAILLE PATTERN DOTS-245678 */
pub const braille_dots_1245678: Keysym = 0x10028fb; /* U+28fb BRAILLE PATTERN DOTS-1245678 */
pub const braille_dots_345678: Keysym = 0x10028fc; /* U+28fc BRAILLE PATTERN DOTS-345678 */
pub const braille_dots_1345678: Keysym = 0x10028fd; /* U+28fd BRAILLE PATTERN DOTS-1345678 */
pub const braille_dots_2345678: Keysym = 0x10028fe; /* U+28fe BRAILLE PATTERN DOTS-2345678 */
pub const braille_dots_12345678: Keysym = 0x10028ff; /* U+28ff BRAILLE PATTERN DOTS-12345678 */

/*
 * Sinhala (http://unicode.org/charts/PDF/U0D80.pdf)
 * http://www.nongnu.org/sinhala/doc/transliteration/sinhala-transliteration_6.html
 */

pub const Sinh_ng: Keysym = 0x1000d82; /* U+0D82 SINHALA ANUSVARAYA */
pub const Sinh_h2: Keysym = 0x1000d83; /* U+0D83 SINHALA VISARGAYA */
pub const Sinh_a: Keysym = 0x1000d85; /* U+0D85 SINHALA AYANNA */
pub const Sinh_aa: Keysym = 0x1000d86; /* U+0D86 SINHALA AAYANNA */
pub const Sinh_ae: Keysym = 0x1000d87; /* U+0D87 SINHALA AEYANNA */
pub const Sinh_aee: Keysym = 0x1000d88; /* U+0D88 SINHALA AEEYANNA */
pub const Sinh_i: Keysym = 0x1000d89; /* U+0D89 SINHALA IYANNA */
pub const Sinh_ii: Keysym = 0x1000d8a; /* U+0D8A SINHALA IIYANNA */
pub const Sinh_u: Keysym = 0x1000d8b; /* U+0D8B SINHALA UYANNA */
pub const Sinh_uu: Keysym = 0x1000d8c; /* U+0D8C SINHALA UUYANNA */
pub const Sinh_ri: Keysym = 0x1000d8d; /* U+0D8D SINHALA IRUYANNA */
pub const Sinh_rii: Keysym = 0x1000d8e; /* U+0D8E SINHALA IRUUYANNA */
pub const Sinh_lu: Keysym = 0x1000d8f; /* U+0D8F SINHALA ILUYANNA */
pub const Sinh_luu: Keysym = 0x1000d90; /* U+0D90 SINHALA ILUUYANNA */
pub const Sinh_e: Keysym = 0x1000d91; /* U+0D91 SINHALA EYANNA */
pub const Sinh_ee: Keysym = 0x1000d92; /* U+0D92 SINHALA EEYANNA */
pub const Sinh_ai: Keysym = 0x1000d93; /* U+0D93 SINHALA AIYANNA */
pub const Sinh_o: Keysym = 0x1000d94; /* U+0D94 SINHALA OYANNA */
pub const Sinh_oo: Keysym = 0x1000d95; /* U+0D95 SINHALA OOYANNA */
pub const Sinh_au: Keysym = 0x1000d96; /* U+0D96 SINHALA AUYANNA */
pub const Sinh_ka: Keysym = 0x1000d9a; /* U+0D9A SINHALA KAYANNA */
pub const Sinh_kha: Keysym = 0x1000d9b; /* U+0D9B SINHALA MAHA. KAYANNA */
pub const Sinh_ga: Keysym = 0x1000d9c; /* U+0D9C SINHALA GAYANNA */
pub const Sinh_gha: Keysym = 0x1000d9d; /* U+0D9D SINHALA MAHA. GAYANNA */
pub const Sinh_ng2: Keysym = 0x1000d9e; /* U+0D9E SINHALA KANTAJA NAASIKYAYA */
pub const Sinh_nga: Keysym = 0x1000d9f; /* U+0D9F SINHALA SANYAKA GAYANNA */
pub const Sinh_ca: Keysym = 0x1000da0; /* U+0DA0 SINHALA CAYANNA */
pub const Sinh_cha: Keysym = 0x1000da1; /* U+0DA1 SINHALA MAHA. CAYANNA */
pub const Sinh_ja: Keysym = 0x1000da2; /* U+0DA2 SINHALA JAYANNA */
pub const Sinh_jha: Keysym = 0x1000da3; /* U+0DA3 SINHALA MAHA. JAYANNA */
pub const Sinh_nya: Keysym = 0x1000da4; /* U+0DA4 SINHALA TAALUJA NAASIKYAYA */
pub const Sinh_jnya: Keysym = 0x1000da5; /* U+0DA5 SINHALA TAALUJA SANYOOGA NAASIKYAYA */
pub const Sinh_nja: Keysym = 0x1000da6; /* U+0DA6 SINHALA SANYAKA JAYANNA */
pub const Sinh_tta: Keysym = 0x1000da7; /* U+0DA7 SINHALA TTAYANNA */
pub const Sinh_ttha: Keysym = 0x1000da8; /* U+0DA8 SINHALA MAHA. TTAYANNA */
pub const Sinh_dda: Keysym = 0x1000da9; /* U+0DA9 SINHALA DDAYANNA */
pub const Sinh_ddha: Keysym = 0x1000daa; /* U+0DAA SINHALA MAHA. DDAYANNA */
pub const Sinh_nna: Keysym = 0x1000dab; /* U+0DAB SINHALA MUURDHAJA NAYANNA */
pub const Sinh_ndda: Keysym = 0x1000dac; /* U+0DAC SINHALA SANYAKA DDAYANNA */
pub const Sinh_tha: Keysym = 0x1000dad; /* U+0DAD SINHALA TAYANNA */
pub const Sinh_thha: Keysym = 0x1000dae; /* U+0DAE SINHALA MAHA. TAYANNA */
pub const Sinh_dha: Keysym = 0x1000daf; /* U+0DAF SINHALA DAYANNA */
pub const Sinh_dhha: Keysym = 0x1000db0; /* U+0DB0 SINHALA MAHA. DAYANNA */
pub const Sinh_na: Keysym = 0x1000db1; /* U+0DB1 SINHALA DANTAJA NAYANNA */
pub const Sinh_ndha: Keysym = 0x1000db3; /* U+0DB3 SINHALA SANYAKA DAYANNA */
pub const Sinh_pa: Keysym = 0x1000db4; /* U+0DB4 SINHALA PAYANNA */
pub const Sinh_pha: Keysym = 0x1000db5; /* U+0DB5 SINHALA MAHA. PAYANNA */
pub const Sinh_ba: Keysym = 0x1000db6; /* U+0DB6 SINHALA BAYANNA */
pub const Sinh_bha: Keysym = 0x1000db7; /* U+0DB7 SINHALA MAHA. BAYANNA */
pub const Sinh_ma: Keysym = 0x1000db8; /* U+0DB8 SINHALA MAYANNA */
pub const Sinh_mba: Keysym = 0x1000db9; /* U+0DB9 SINHALA AMBA BAYANNA */
pub const Sinh_ya: Keysym = 0x1000dba; /* U+0DBA SINHALA YAYANNA */
pub const Sinh_ra: Keysym = 0x1000dbb; /* U+0DBB SINHALA RAYANNA */
pub const Sinh_la: Keysym = 0x1000dbd; /* U+0DBD SINHALA DANTAJA LAYANNA */
pub const Sinh_va: Keysym = 0x1000dc0; /* U+0DC0 SINHALA VAYANNA */
pub const Sinh_sha: Keysym = 0x1000dc1; /* U+0DC1 SINHALA TAALUJA SAYANNA */
pub const Sinh_ssha: Keysym = 0x1000dc2; /* U+0DC2 SINHALA MUURDHAJA SAYANNA */
pub const Sinh_sa: Keysym = 0x1000dc3; /* U+0DC3 SINHALA DANTAJA SAYANNA */
pub const Sinh_ha: Keysym = 0x1000dc4; /* U+0DC4 SINHALA HAYANNA */
pub const Sinh_lla: Keysym = 0x1000dc5; /* U+0DC5 SINHALA MUURDHAJA LAYANNA */
pub const Sinh_fa: Keysym = 0x1000dc6; /* U+0DC6 SINHALA FAYANNA */
pub const Sinh_al: Keysym = 0x1000dca; /* U+0DCA SINHALA AL-LAKUNA */
pub const Sinh_aa2: Keysym = 0x1000dcf; /* U+0DCF SINHALA AELA-PILLA */
pub const Sinh_ae2: Keysym = 0x1000dd0; /* U+0DD0 SINHALA AEDA-PILLA */
pub const Sinh_aee2: Keysym = 0x1000dd1; /* U+0DD1 SINHALA DIGA AEDA-PILLA */
pub const Sinh_i2: Keysym = 0x1000dd2; /* U+0DD2 SINHALA IS-PILLA */
pub const Sinh_ii2: Keysym = 0x1000dd3; /* U+0DD3 SINHALA DIGA IS-PILLA */
pub const Sinh_u2: Keysym = 0x1000dd4; /* U+0DD4 SINHALA PAA-PILLA */
pub const Sinh_uu2: Keysym = 0x1000dd6; /* U+0DD6 SINHALA DIGA PAA-PILLA */
pub const Sinh_ru2: Keysym = 0x1000dd8; /* U+0DD8 SINHALA GAETTA-PILLA */
pub const Sinh_e2: Keysym = 0x1000dd9; /* U+0DD9 SINHALA KOMBUVA */
pub const Sinh_ee2: Keysym = 0x1000dda; /* U+0DDA SINHALA DIGA KOMBUVA */
pub const Sinh_ai2: Keysym = 0x1000ddb; /* U+0DDB SINHALA KOMBU DEKA */
pub const Sinh_o2: Keysym = 0x1000ddc; /* U+0DDC SINHALA KOMBUVA HAA AELA-PILLA*/
pub const Sinh_oo2: Keysym = 0x1000ddd; /* U+0DDD SINHALA KOMBUVA HAA DIGA AELA-PILLA*/
pub const Sinh_au2: Keysym = 0x1000dde; /* U+0DDE SINHALA KOMBUVA HAA GAYANUKITTA */
pub const Sinh_lu2: Keysym = 0x1000ddf; /* U+0DDF SINHALA GAYANUKITTA */
pub const Sinh_ruu2: Keysym = 0x1000df2; /* U+0DF2 SINHALA DIGA GAETTA-PILLA */
pub const Sinh_luu2: Keysym = 0x1000df3; /* U+0DF3 SINHALA DIGA GAYANUKITTA */
pub const Sinh_kunddaliya: Keysym = 0x1000df4; /* U+0DF4 SINHALA KUNDDALIYA */
