
A quantum computer simulator to learn both rust and Quantum computers in general.

Resources:
https://batistalab.com/classes/v572/Mosca.pdf

To solidify the design
Assumptions being used:
1. Any state A->B->C where '->' is the application of some gates can be split into A->B, B->C
2. Any instruction must belong to a timestamp
3. A gate that changes the state of a qubit can only exist to one qubit per timestamp
4. Instructions will be held as lambda functions, and be ran when the instruction is called
5. Registers will be contained as individual qubits until it is to be measured
6. a timestamp has a set of instructions
7. an instruction is (gate, qubit1 location, qubit2 location)
