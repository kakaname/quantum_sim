
A quantum computer simulator to learn both rust and Quantum computers in general.

Resources:
https://batistalab.com/classes/v572/Mosca.pdf


The way it works:

A circuit is made out of individual timestamps.
Individual timestamps have multiple gates which are applied to specific qubits.
  - Timestamps cannot have more than one gate apply to a singular qubit.
Gates and timestamps will handle the changes to the state vector of the quantum register in and arbitrary order.

Importantly: All calculations are done on pure quantum states rather than mixed quantum states ( pure quantum states are much easier to simulate as they only require matrix multiplication )



Currently, the application of a register to a single qubit gate works, The goal is to have both a CX, and CCX gate working, then to order all of these within a quantum circuit.

The tests under each quantum gate shows how each piece of code interacts with eachother.


'''
  let register = QuantumRegister::new_with_size(4); // creates a quantum register with size four in state |0000>
  let cnot = XGate::x(0); // creates a Pauli_X gate for qubit 0 
  cnot.apply(&mut register) // applies the gate to the register, here the state of the register is |0001>
'''


More will be added with time, the next steps are:
 1. implement both CXGates and CCXGates
 2. implement circuit, and the running of circuits
 3. implmenet multiple gates to a timestamp
 4. implement a method printing the state of the circuit
 5. implementing a working UI to make the organization of the circuit easier