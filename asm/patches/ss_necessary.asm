.open "main.dol"

.org @NextFreeSpace
.global custom_main_additions

.org 0x80062e40
mr r3, r31 ; tell main to use this
bl custom_main_additions
mr r31, r3 ; Force the game to stop
b 0x80064ca0


.close