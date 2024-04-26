import React from 'react';
import './toolbar.css';
import Gate from './gate';

function Toolbar(){
    return (
    <div className='Toolbar'>
      <div className='gates-container'>
      <h1 style={{color:'white'}}>Gates</h1>
        <div className='gates'>
          <Gate name="X" id = "X"/>
          <Gate name="Y" id = "Y"/>
          <Gate name="Z" id = "Z"/>
          <Gate name="H" id = "H"/>
        </div>
      </div>
      <div className='control'>
      <h1 style={{color:'white'}}>Control</h1>
        <Gate name="." id = "C_down"/>
      </div>
      <div className='swap'>
      <h1 style={{color:'white'}}>Swap</h1>
        <Gate name="SWAP-1" id = "SWAP-1"/>
        <Gate name="SWAP-2" id = "SWAP-2"/>
      </div>
    </div>
    );
  }

export default Toolbar;