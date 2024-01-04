.open "main.dol"

.org @NextFreeSpace
.global custom_main_additions

.org 0x80062e60
bl custom_main_additions

.close