# nelabai pabaigtas chess engine naudojimui kaip sperai naujam chess engine
neisivaizduoju ka as prirasiau pries puse metu

### Files
* `deprecated_eval` pries tai buvusi eval versija db nenaudojama
* `deprecated_move_gen` pries tai buvusi move_gen versija db nenaudojama
* `eval` naudoja `evaluation_function` ir pagal tai randa geriausia ejima
* `evaluation_function` file skirtas funkcijai kuri nusprendzia ant kiek gera yra tam tikra pozicija
* `hashing` file kuriame yra definined hashing algorithm
* `main` root file kuris dabar paleidzia tests::main 
* `move_gen` generuoja pseudolegal moves
* `tests` naudojama norint paziureti kaip veikia programa kad `main` liktu clean
* `utils` defininta viskas ko gali prireikti bet kur kitur
