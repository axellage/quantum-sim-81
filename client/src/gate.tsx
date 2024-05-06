import React from 'react';
import './gate.css';
import {useDraggable} from '@dnd-kit/core';
import {CSS} from '@dnd-kit/utilities';

function Gate(props:any) {
    const {attributes, listeners, setNodeRef, transform} = useDraggable({
        id: props.id,
      });
      const style = {
        transform: CSS.Translate.toString(transform),
        width: 50,
        height: 50
        
      };
      
      if(props.name === "."){
        return (
          <button className="gate" ref={setNodeRef} style={style} {...listeners} {...attributes}>
            <div className='control-icon'></div>
          </button>
        );
      }else if(props.id === "Swap") {
        return (
          <button className="gate" ref={setNodeRef} style={style} {...listeners} {...attributes}>
            <div className='swap-icon'>×</div>
            <div className='icon-line'></div>
          </button>
        );
      }else {
        return (
          <button className="gate" ref={setNodeRef} style={style} {...listeners} {...attributes}>
            <h1>{props.name}</h1>
          </button>
        );
      }
  }

export default Gate;