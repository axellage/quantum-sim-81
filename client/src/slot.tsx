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

    return (
    <div ref={setNodeRef} style={style}>
        <PlacedGate name = {props.name} gateType = {props.gateType}/>
    </div>
    );
}

export default Slot;