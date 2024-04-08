import React from 'react';
import './gate.css';

function PlacedGate(props:any){
  
    // Display nothing if there is no placed gate (which is the same as the identity gate).
    if(props.gateType !== "I"){
      return (
        <button className = "placedGate">
          <h1>{props.name}</h1>
        </button>
      );
    } 
    else return null;
    
    
}

export default PlacedGate;