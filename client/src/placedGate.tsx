import React from 'react';
import './gate.css';

function PlacedGate(props:any){
  
    // Display nothing if there is no placed gate (which is the same as the identity gate).
    if(props.gateType !== "I"){
      if (props.gateType === "C_down") {
        return (
          <div className='placed-cnot-container'>
            <button className = "placed-cnot" onClick={props.removeGate}>
            </button>
            <div className='cnot-line'></div>
          </div>
        );
      }else if(props.gateType === "Swap"){
        return(
            <button className = "placed-swap" onClick={props.removeGate}>
              <p>x</p>
              <div className='swap-line'></div>
            </button>
        );

      }else{
        return (
          <button className = "placedGate" onClick={props.removeGate}>
            <h1>{props.name}</h1>
          </button>
        );
      }
    }
    else return null;
    
    
}

export default PlacedGate;
