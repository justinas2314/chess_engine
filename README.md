# chess_engine
Nekeistas ktug chess bot konkurse dalyvavusio chess engine kodas  
### Naudojimas
Sukompiliuoti su rust, žaidimui naudoti [šį interface](https://github.com/HollaFoil/KTUG-ChessBot)  
Norint parinkti ar žaisti su baltomis, ar juodomis figūromis, galima arba keisti pusę nustatančias konstantas (`main.rs`), arba command line arguments parašyti white/black
### Pastabos
* Tyčia nenaudojau magic bitboardų ir panašių ėjimų generavimo optimizacijų, nes užsibrėžiau tikslą reusinti savo seną, prieš kelis metus rašytą, kodą
* Variklis nuvertina toli nuėjusius pėstininkus ir yra kitų vietų, kur galima variklį padaryti stipresniu pakeičiant konstantas `evaluation_function.rs`, tačiau koda palieku tokį, koks dalyvavo konkurse
* Turnyre iš 9 variklių pelnyta (subjektyvu ką reiškia pelnyta) 2 vieta
* Nesistengiau, kad kodas būtų aiškus skaitytojui, kuriam nepaaiškinau ką kuri vieta daro
* Pozicijos įvertinimo euristiką ir svorius, sudariau naudodamasis sveiku protu bei taisydamas, kai variklis padarydavo keistą ėjimą (ne taip, kaip tam tikras 1 vietos laimėjojas >:c)
