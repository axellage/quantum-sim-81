import React, { useState, ReactNode, useEffect } from 'react';
import './circuitboard.css';
import './toolbar.css';
import Slot from './slot';



function Circuitboard( { circuit, setCircuit, sendCircuit, isOracleVisible, setIsOracleVisible, isUniVisible, setIsUniVisible} :{circuit: string[][], setCircuit : (circuit: string[][]) => void, sendCircuit: () => void, isOracleVisible: boolean, setIsOracleVisible: (isVisible: boolean) => void, isUniVisible: boolean, setIsUniVisible: (isVisible: boolean) => void}){
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
    }, [circuit, sendCircuit]); // Circuit dependency array to make it only update when circuit is changed

    function QubitLine(props:any) {
        const qubitLineId = Number(props.id);
        const circuitLine = circuit[qubitLineId] || []; // Fallback to an empty array if circuit[qubitLineId] is undefined
      
        return (
            <div className='qubitLine'>
              <h2>|0⟩</h2>
              <hr/>
              <div className='slot-container'>
                {circuitLine.map((gate, index) => <Slot name={gate} gateType={gate} id={`${qubitLineId}${index}`} key={`${qubitLineId}${index}`} circuit={circuit} setCircuit={setCircuit} sendCircuit={sendCircuit}/>)}
              </div>
            </div>
        );
      }

    const hideOracle = () => {
      setIsOracleVisible(false);
    }

    const hideUni = () => {
      setIsUniVisible(false);
    }

    return(
    <div>
      <section className="circuit">
        {isOracleVisible && <button className='oracle' onClick={hideOracle}>Oracle</button>}
        {isUniVisible && <button className='unitary' onClick={hideUni}>U</button>}
        {qubitLines}
      </section>
    </div>)
  }
  export default Circuitboard;