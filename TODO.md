## TP1 - Decoding instructions and implementing our first operation

### Informations

Un squelette de la librairie est fourni, avec des tests unitaire.
Pour les lancer, on utilise `cargo test`. Rien ne vous empêche d'ajouter vos propres tests unitaires!

La **gestion des Erreurs** est faite sommairement via le type [Error] de notre crate. Le but étant d'avoir le moins de dépendences possible. Dans un projet plus sérieux, il est conseillé d'utiliser des crates comme [thiserror](https://docs.rs/thiserror/latest/thiserror/) pour vos librairies, et [anyhow](https://docs.rs/anyhow/latest/anyhow/) pour vos executables.

On va remplir notre squelette pour implementer un émulateur d'une architecture imaginaire décrite dans le fichier README.md

Pour cette première partie, on ne considère que les registres généraux, et on ignorera donc le pointeur d'instruction `RIP` et les flags d'opérations `RFLAGS` mais je vous invite à vous renseigner sur ces notions si elles vous sont nouvelles.

### Exercice 1 - Décodage des instructions

Notre but ici est de parser les instructions de notre architecture en un format qui nous permette ensuite de les executer facilement. Ce format est déjà décrit dans `instruction.rs` via les types suivants :

```rs
pub enum OpCode {
    LD = 0x00,
    ST = 0x01,
    ADD = 0x02,
    XOR = 0x03,
}
pub struct Instruction {
    opcode: OpCode,
    op0: u8,
    op1: u8,
}
```
Remarque: Attention à certains réflèxes de la programmation en `C`, nottament vouloir transmuter des entiers en enum, et à supposer des choses sur la représentation mémoire des enums et des structs. En rust, elle n'est pas toujours guarantie - ou intuitive - et pour en utiliser les détails vous entrez dans le monde épineux du code `unsafe` !

- Dans `instruction.rs` on va implementer la fonction suivante pour `OpCode` :
```rs
pub fn from_u8(opcode: u8) -> Result<OpCode, Error>
```
qui convertit un `u8` en OpCode, et peut faillir avec un `InvalidOpCode(u8)` ou un `UnsupportedOpCode(OpCode)`

- Toujours dans `instruction.rs` on va ensuite implementer la fonction suivante : 
```rs
pub fn disassemble(insn: u16) -> Result<Instruction, Error>
```
Qui décode une instruction complète, et peut faillir de plusieurs manières.

Si tout se passe bien, les tests du fichier s'allument en vert, et on peut passer à la suite !

### Exercice 2

Il s'agit maintenant d'implementer une operation simple. Exécuter une instruction, c'est faire évoluer l'état de notre processeur imaginaire. Il nous faut donc déjà une manière de représenter cet état. On fournit pour cela le type suivant :
```rs
pub struct Core {
    registers: [u16; MAX_REGISTER_INDEX as usize + 1]
}
```
- On va d'abords implementer un constructeur pour ce type. En rust, il n'y a pas de notion de constructeur integrée au langage, mais par convention on va implementer la fonction suivante 
```rs
impl Core {
    /// Instantiate our core
    pub fn new() -> Self {}
}
```
Pour initialiser l'état de notre processeur, on va le remplir arbitrairement, par exemple en initialisant chaque registre `R_i` avec la valeur `i`. Plus tard, on lira une section du programme que l'on execute sur notre processeur pour initialiser ces valeurs.

- On implemente la méthode suivante pour le type `Core`
```rs
pub fn register(&self, index: u8) -> Result<u16, Error> 
```
qui lit simplement la valeur d'un registre, et qui peut faillir..

- On implémente la méthode `execute`, qui lance simplement la méthode correspondant a l'OpCode de l'instruction passée en argument parmis les méthodes situées plus bas dans le fichier

- Enfin, on implemente la fonction `add` de `Core`, qui prends une référence mutable à `self`, et une instruction décodée, de la forme 
```rs
let insn = Instruction {
    opcode : OpCode::ADD,
    op0 : i,
    op1 : j
}
```
et qui met a jour la valeur de `R_i` avec la somme des valeurs des registres `R_i` et `R_j`. Attention, que faire lorsque notre addition cause un `overflow` ? 
On se contentera pour l'instant de renvoyer une erreur `AdditionOverflow`..

- De même, on implémente la fonction `xor`, qui met à jour la valeur de `R_i` avec le xor bit à bit entre les valeurs de `R_i` et `R_j`
