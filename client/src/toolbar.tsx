import React from 'react';
import './toolbar.css';
import Gate from './gate';

function Toolbar( {setCircuit}: {setCircuit : (circuit: string[][]) => void}){
  function handleQFT(): void {
    setCircuit([["I","I","Swap","I","Swap","I","C_down","H","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["I","C_down","Swap","C_down","Swap","H","S","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["H","S","I","T","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"]])
  }
  function handleGrover(): void {
    setCircuit([["H","C_down","X","H","C_down","Z","H","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["H","Z","X","H","Z","Z","H","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"],["I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I","I"]]);
  }

    return (
    <div className='Toolbar'>
      <div className='gates-container'>
        <div className='gates'>
          <Gate name="X" id = "X"/>
          <Gate name="Y" id = "Y"/>
          <Gate name="Z" id = "Z"/>
          <Gate name="H" id = "H"/>
          <Gate name="S" id = "S"/>
          <Gate name="T" id = "T"/>
        </div>
      </div>
      <div className='control'>
        <Gate name="." id = "C_down"/>
      </div>
      <div className='swap'>
        <Gate name="Swap" id = "Swap"/>
      </div>
      <div className='algorithms'>
        <button onClick={handleQFT}>QFT</button>
        <button onClick={handleGrover}>Grover's</button>
      </div>
    </div>
    );
  }

export default Toolbar;