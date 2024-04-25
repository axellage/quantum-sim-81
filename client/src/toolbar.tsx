import React from 'react';
import './toolbar.css';
import Gate from './gate';

function Toolbar(){
    return (
    <div className='Toolbar'>
      <Gate name="X" id = "X"/>
      <Gate name="Y" id = "Y"/>
      <Gate name="." id = "C_down"/>
      <Gate name="Z" id = "Z"/>
      <Gate name="H" id = "H"/>
      <Gate name="SWAP-1" id = "SWAP-1"/>
      <Gate name="SWAP-2" id = "SWAP-2"/>
    </div>
    );
  }

export default Toolbar;