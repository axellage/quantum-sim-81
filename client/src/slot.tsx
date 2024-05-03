import React from 'react';
import './slot.css';
import './toolbar.css';
import PlacedGate from './placedGate';
import {useDroppable} from '@dnd-kit/core';

function Slot(props:any) {
    const {isOver, setNodeRef} = useDroppable({
      id: props.id,
    });

    // TODO: Move to CSS.
    const style = {
      opacity: (isOver ? .8 : 1)
    };

    function removeGate() {
      let qubit = Number(props.id[0]);
      let index = Number(props.id.substring(1));
      let newCircuit = props.circuit;
      newCircuit[qubit][index] = "I";
      
      props.setCircuit(newCircuit);
      props.sendCircuit();
    }

    return (
    <div ref={setNodeRef} style={style}>
        <PlacedGate name = {props.name} gateType = {props.gateType} removeGate = {removeGate} handleDragEnd={props.handleDragEnd}/>
    </div>
    );
}

export default Slot;