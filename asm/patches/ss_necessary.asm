.open "main.dol"

.org @NextFreeSpace
.global custom_main_additions

; 0x80062f40 in JP 1.0
; 0x80062e60 in US 1.0
.org @MainInjection
bl custom_main_additions

.close