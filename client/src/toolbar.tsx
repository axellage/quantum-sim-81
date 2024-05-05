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
  
  const [style, setStyle] = useState({display: 'none'});

    return (
    <div className='Toolbar'>
      <div className='gates-container'>
        <h1>Gates <img src={tooltip} style={{height: '25px', width: '25px'}} onMouseEnter={e => {
                     setStyle({display: 'flex'});
                 }}
                 onMouseLeave={e => {
                     setStyle({display: 'none'})
                 }}/></h1>
        <div className='gates-tip' style={style}>
          <p> <b>X</b>: The Pauli-X gate is the quantum equivalent of the NOT gate for classical computers with respect to the standard basis |0⟩ 
            , |1⟩, which distinguishes the z axis on the Bloch sphere. It is sometimes called 
            a bit-flip as it maps |0⟩ to |1⟩ and |1⟩ to |0⟩. <br/><br/>

            <b>Y</b>: Similarly, the Pauli-Y maps |0⟩  to i|1⟩  and |1⟩ to  -i|0⟩. <br/><br/>

            <b>Z</b>: Pauli-Z leaves the basis state |0⟩ unchanged and maps |1⟩ to  -|1⟩. Due to this nature, Pauli-Z is sometimes called phase-flip.</p>
            <p> <b>H</b>: The Hadamard gate maps the basis states |0⟩ → |0⟩+|1⟩/√2 and |1⟩ → |0⟩−|1⟩/√2 (it creates an equal superposition state if given a computational basis state).<br/><br/>
            <b>T</b>: The T gate, also known as the π/8 gate, introduces a phase shift of π/4 radians around the Z-axis of the Bloch sphere.<br/><br/>
            <b>S</b>: The S gate is also known as the phase gate, represents a 90-degree rotation around the z-axis. The S gate is related to the T gate by the relationship S = T².
            </p>
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
      <h1>Control <img src={tooltip} style={{height: '25px', width: '25px'}}/></h1>
      <div className='control-swap'>
        <Gate name="." id = "C_down"/>
        <Gate name="W" id = "Swap"/>
      </div>
      <h1>Algorithms <img src={tooltip} style={{height: '25px', width: '25px'}}/></h1>
      <div className='algorithms'>
        <button onClick={handleQFT}>QFT</button>
        <button onClick={handleGrover}>Grover's</button>
        <button onClick={handleDJ}>Deutsch-Joza</button>

      </div>
    </div>
    );
  }

export default Toolbar;