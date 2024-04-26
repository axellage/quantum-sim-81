import React, { useState, ReactNode, useEffect } from 'react';
import './circuitboard.css';
import './toolbar.css';
import Slot from './slot';

function Circuitboard(circuit: string[][]){
    const [qubitLines, setQubitLines] = useState<ReactNode[]>([]);

    useEffect(() => {
      setQubitLines([
        <div>
          <QubitLine id="0"/>
        </div>,
        <div>
          <QubitLine id="1"/>
        </div>,
        <div>
          <QubitLine id="2"/>
        </div>,
        <div>
          <QubitLine id="3"/>
        </div>,
        <div>
          <QubitLine id="4"/>
        </div>,
        <div>
          <QubitLine id="5"/>
        </div>
      ]);
    }, [circuit]); // Circuit dependency array to make it only update when circuit is changed
  
  {/*  const addQubit = () => {
      if (qubitLines.length < 6) {
        setQubitLines(prevQubitLines => [
          ...prevQubitLines,
          <div>
            <QubitLine id={JSON.stringify(qubitLines.length)}/>
          </div>
        ]);
      }
      else {
        alert("No more qubits can be added");
        console.log("No more qubits can be added");
      }
    };
  
    const removeQubit = () => {
      if (qubitLines.length > 1) {
        setQubitLines(prevQubitLines => prevQubitLines.slice(0, -1));
      }
      else {
        //TODO make this a visible error
        console.log("Already 0 qubits");
      }
    };*/}

    function QubitLine(props:any) {
        const qubitLineId = Number(props.id);
        const circuitLine = circuit[qubitLineId] || []; // Fallback to an empty array if circuit[qubitLineId] is undefined
      
        return (
            <div className='qubitLine'>
              <h2>|0⟩</h2>
              <hr/>
              <div className='slot-container'>
                {//TODO create records for gateTypes and their corresponding names
                }
                {circuitLine.map((gate, index) => <Slot name={gate} gateType={gate} id={`${qubitLineId}${index}`} key={`${qubitLineId}${index}`} />)}
              </div>
            </div>
        );
      }

    return(
    <div>
      <section className="circuit">
        {qubitLines}
      </section>
      {/*<button onClick={addQubit}>+</button>
      <button onClick={removeQubit}>-</button>*/}
    </div>)
  }
  export default Circuitboard;