import React from 'react';
import './toolbar.css';
import Gate from './gate';

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
  

    return (
    <div className='Toolbar'>
      <div className='gates-container'>
        <h1>Gates</h1>
        <div className='gates'>
          <Gate name="X" id = "X"/>
          <Gate name="Y" id = "Y"/>
          <Gate name="Z" id = "Z"/>
          <Gate name="H" id = "H"/>
          <Gate name="S" id = "S"/>
          <Gate name="T" id = "T"/>
        </div>
      </div>
      <h1>Control</h1>
      <div className='control-swap'>
        <Gate name="." id = "C_down"/>
        <Gate name="W" id = "Swap"/>
      </div>
      <h1>Algorithms</h1>
      <div className='algorithms'>
        <button onClick={handleQFT}>QFT</button>
        <button onClick={handleGrover}>Grover's</button>
        <button onClick={handleDJ}>Deutsch-Joza</button>

      </div>
    </div>
    );
  }

export default Toolbar;