import React from 'react';
import './toolbar.css';
import Gate from './gate';

function Toolbar(){
    return (
    <div className='Toolbar'>
      <div className='gates-container'>
        <div className='gates'>
          <Gate name="X" id = "X"/>
          <Gate name="Y" id = "Y"/>
          <Gate name="Z" id = "Z"/>
          <Gate name="H" id = "H"/>
        </div>
      </div>
      <div className='control'>
        <Gate name="." id = "C_down"/>
      </div>
      <div className='swap'>
        <Gate name="Swap" id = "Swap"/>
      </div>
    </div>
    );
  }

export default Toolbar;