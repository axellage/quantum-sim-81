import React, { useState } from 'react';
import './toolbar.css';
import Gate from './gate';
import tooltip from './question-circle-svgrepo-com.svg';

function Toolbar( {setCircuit, setIsOracleVisible, setIsUniVisible}: {setCircuit : (circuit: string[][]) => void, setIsOracleVisible: (isVisible:boolean) => void, setIsUniVisible: (isVisible:boolean) => void}){
  function handleQFT(): void {
    setCircuit([["I","I","Swap","I","Swap","I","C_down","H","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["I","C_down","Swap","C_down","Swap","H","S","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["H","S","I","T","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"]])
    toggleUni(false);
    toggleOracle(false);
  }
  function handleGrover(): void {
    setCircuit([["H","I","I","H","X","C_down","X","H","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["H","I","I","H","X","Z","X","H","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["H","I","I","I","I","I","I","H","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"]]);
    toggleUni(false);
    toggleOracle(true);
  }

  function handleDJ(): void {
    setCircuit([["I","H","I","I","H","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["X","H","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"]]);
    toggleOracle(false);
    toggleUni(true);
  }

  const toggleOracle = (isVisible:boolean) => {
    setIsOracleVisible(isVisible);
  }

  const toggleUni = (isVisible:boolean) => {
    setIsUniVisible(isVisible);
  }
  
  const [gatesStyle, setGatesStyle] = useState({display: 'none'});
  const [controlStyle, setControlStyle] = useState({display: 'none'});
  const [algoStyle, setAlgoStyle] = useState({display: 'none'});

    return (
    <div className='Toolbar'>
      <div className='gates-container'>
        <h2>Gates <img src={tooltip} style={{height: '25px', width: '25px'}} onMouseEnter={e => {
                     setGatesStyle({display: 'flex'});
                 }}
                 onMouseLeave={e => {
                     setGatesStyle({display: 'none'})
                 }}/></h2>
        <div className='gates-tip' style={gatesStyle}>
          <h2>Usage: Drag & Drop. Click to Remove</h2>
          <section><p> <b>X</b>: The Pauli-X gate is the quantum equivalent of the NOT gate for classical computers with respect to the standard basis |0⟩ 
            , |1⟩, which distinguishes the z axis on the Bloch sphere. It is sometimes called 
            a bit-flip as it maps |0⟩ to |1⟩ and |1⟩ to |0⟩. <br/><br/>

            <b>Y</b>: Similarly, the Pauli-Y maps |0⟩  to i|1⟩  and |1⟩ to  -i|0⟩. <br/><br/>

            <b>Z</b>: Pauli-Z leaves the basis state |0⟩ unchanged and maps |1⟩ to  -|1⟩. Due to this nature, Pauli-Z is sometimes called phase-flip.</p>
            <p> <b>H</b>: The Hadamard gate maps the basis states |0⟩ → |0⟩+|1⟩/√2 and |1⟩ → |0⟩−|1⟩/√2 (it creates an equal superposition state if given a computational basis state).<br/><br/>
            <b>T</b>: The T gate, also known as the π/8 gate, introduces a phase shift of π/4 radians around the Z-axis of the Bloch sphere.<br/><br/>
            <b>S</b>: The S gate is also known as the phase gate, represents a 90-degree rotation around the z-axis. The S gate is related to the T gate by the relationship S = T².
            </p></section>
        </div>
        <div className='gates'>
          <Gate name="X" id = "X"/>
          <Gate name="Y" id = "Y"/>
          <Gate name="Z" id = "Z"/>
          <Gate name="H" id = "H"/>
          <Gate name="S" id = "S"/>
          <Gate name="T" id = "T"/>
        </div>
      </div>
      <h2>Control & Swap<img src={tooltip} style={{height: '25px', width: '25px'}} onMouseEnter={e => {
                     setControlStyle({display: 'flex'});
                 }}
                 onMouseLeave={e => {
                     setControlStyle({display: 'none'})
                 }}/></h2>
      <div className='control-tip' style={controlStyle}>
        <h2>Usage: Drag & Drop. Click to Remove</h2>
          <section><p> <b>Control</b>: The control operation selects a qubit as a control bit for another gate. If the control bit is 1 the gate controlled by the control bit is applied. <br></br><b>NOTE</b>: Limited to only controlling a gate directly below the control qubit.</p>

              <p><b>Swap</b>: The SWAP gate swaps the states of two qubits. <br></br>
              <b>NOTE</b>: Limited to only swapping the states of two adjacent qubits.

          </p></section>
        </div>
      <div className='control-swap'>
        <Gate name="." id = "C_down"/>
        <Gate name="Swap" id = "Swap"/>
      </div>
      <h2>Algorithms <img src={tooltip} style={{height: '25px', width: '25px'}} onMouseEnter={e => {
                     setAlgoStyle({display: 'flex'});
                 }}
                 onMouseLeave={e => {
                     setAlgoStyle({display: 'none'})
                 }}/></h2>
      <div className='algo-tip' style={algoStyle}>
        <h2>Usage: Click</h2>
          <section>
          <p> <b>QFT</b>: (3 qubit) quantum fourier transformation. The quantum Fourier transform is a linear transformation on quantum bits, and is the quantum version of the discrete Fourier transform. The QFT is a part of many quantum algorithms, notably Shor's algorithm for factoring and computing the discrete logarithm.<br></br><br></br>
           <b>Grover's</b>: Grover's algorithm, also known as the quantum search algorithm, is a quantum algorithm for unstructured search that finds with high probability the unique input to an oracle function that produces a particular value. Here the oracle is undefined and is more for show, however the oracle function can be constructed in many different ways depending on what function one wants it to be.</p>
          <p> <b>Deutsch-Jozsa</b>: Deutsch-Jozsa algorithm. It is a black box problem that can be solved efficiently by a quantum computer with no error, whereas a deterministic classical computer would need a exponential number of queries to the black box to solve the problem. In the Deutsch–Jozsa problem, we are given an oracle that implements some function: f. The function takes n-bit binary values as input and produces either a 0 or a 1 as output for each such value. We are promised that the function is either constant (0 on all inputs or 1 on all inputs) or balanced (1 for exactly half of the input domain and 0 for the other half). The task then is to determine if f is constant or balanced by using the oracle. </p>
          </section>
        </div>
      <div className='algorithms'>
        <button onClick={handleQFT}>QFT</button>
        <button onClick={handleGrover}>Grover's</button>
        <button onClick={handleDJ}> Deutsch-Jozsa </button>
      </div>
    </div>
    );
  }

export default Toolbar;