      #include p16core.inc

        udata   0x20
vA      res     1    ; rezerwacja pami?ci dla zmiennych
cnt     res     1

        org     0x0000 ; pocz?tek programu
        bcf     INTCON,GIE ; zablokowanie przerwa?
        goto    Main ; skok do programu g?ównego

        ;org     0x0004  ;wektor przerwania

        org     0x0006 ;
Main:
        call    TMR1Setup ; ustawienia dla Timer 1
        clrf    vA ; vA <- 0

MainLoop:
        movlw   0x01
        xorwf   vA,f   ; zanegowanie
        movf    vA,w   ; w <- vA
        movwf   PORTB  ; PORTB <- w
        call    Delay200ms  ; opó?nienie 200ms
        goto    MainLoop

TMR1Setup:
       	bsf		T1CON,TMR1ON
        bcf		T1CON,4
        bsf		T1CON,5
        return
Delay10ms:     ; podprogram realizuj?cy opó?nienie 10ms
        movlw	d'11'
        movwf	T1H
        movlw	d'219'
        movwf	T1L
        bcf		PIR1,TMR1IF
WaitForTMR1IF:
        btfsc	PIR1,TMR1IF
        return
        goto	WaitForTMR1IF

Delay200ms:    ; podprogram realizuj?cy opó?nienie 200ms
        movlw     d'20'
        movwf     cnt
D200msLoop:
        call      Delay10ms
        decfsz    cnt,f
        goto      D200msLoop
        return

end