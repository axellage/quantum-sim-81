import React, { useState, useEffect } from 'react';
import './circuitboard.css';
import './toolbar.css';
import Toolbar from './toolbar';
import {DndContext} from '@dnd-kit/core';
import axios from 'axios';
import Circuitboard from './circuitboard';
import './slider.css';

import { BarChart, barElementClasses } from '@mui/x-charts/BarChart';
import { axisClasses } from '@mui/x-charts/ChartsAxis';
import { legendClasses } from '@mui/x-charts';


function App() {
  // This matrix doesn't contain actual elements, just information about what the circuit looks like.
  const [circuit, setCircuit] = useState([["I","I","I","I"], ["I","I","I","I"], ["I","I","I","I"], ["I","I","I","I"], ["I","I","I","I"], ["I","I","I","I"]]);
  // Initializing this because it complains about type otherwise, there is probably a better way to do it.
  const [states, setStates] = useState([{"step":0, "state":[]}]);

  const [stepNumber, setStepNumber] = useState(4);
  const [displayedGraph, setDisplayedGraph] = useState("Probabilities");

  const changeGraph = (e:any) => {
    setDisplayedGraph(e.target!.value);
    
  }
  const onChange = (e:any) => {
    setStepNumber(e.target!.value)
  }
  useEffect(() => {
    // This effect will be triggered whenever the circuit state changes
    sendCircuit();
  }, [circuit]);

  // TODO implement setCircuit (aka add + and - buttons).

  return (
    <div className="App">
      <DndContext onDragEnd={handleDragEnd}>
        <Toolbar />
        <Circuitboard {...circuit}/> {/*shallow copy of circuit to circuitboard, solve for it to be in circuitboard later*/}
        {/*<button onClick={sendCircuit}>send circuit</button>*/}
        <div className='slider-container'>
          <input
            type='range'
            min={1}
            max={4}
            defaultValue={4}
            step={1}
            className='range'
            onChange={onChange}
          />
          <div className='step-numbers'>
            <p>1</p>
            <p>2</p>
            <p>3</p>
            <p>4</p>
          </div>
        </div>
        <select className="dropdown"  onChange={changeGraph}>
            <option className='option' id="0" >Probabilities</option>
            <option className='option' id="1" >State vectors</option>
        </select>
        <States dispGraph={displayedGraph}/>
      </DndContext>
    </div>
  );
  
  

function handleDragEnd(event:any){
    const {active, over} = event;
    console.log(over.id[0]);
    if(active.id === "C_down"){
      if(over.id[0] === 5){
        alert("No gate to control.");
        return;
      }
      if(circuit[parseInt(over.id[0]) + 1][parseInt(over.id[1])] === "I"){
        alert("No gate to control.");
        return;
      }
    }

    console.log("Placed gate on position " + over.id[1] + " on qubit line " + over.id[0]);

    // These nested maps replace the gate at the given position.
    const newCircuit = circuit.map((line, i) => {
      if(i === (Number(over.id[0]))) {
        return (line.map((gate, j) => {
          if(j === (Number(over.id[1]))){
            return (active.id);
          } else{
            return (gate);
          }
        }));
      } else {
        return line;
      } 
    });
    setCircuit(newCircuit);
    
  }
  

  async function sendCircuit() {
    console.log("Sending circuit: " + convertToOldVersion(circuit));
    const response = await axios.post('http://localhost:8000/simulate',
        {circuit_matrix: convertToOldVersion(circuit)})
  .then(function(response: any){
    console.log(response);
    setStates(response.data.state_list);
  })}

  function convertToOldVersion(newCircuit:string[][]){
    for(let i = 0; i < newCircuit.length - 1; i++){
      for(let j = 0; j < newCircuit[0].length; j++){
        if(newCircuit[i][j] === "C_down"){
          newCircuit[i][j] = "CNOT-1";
          newCircuit[i + 1][j] = "CNOT-2";
          //newCircuit = swapMatrixItem(newCircuit, i + 1, j, "CNOT-2")
        }
      }
    }
    return newCircuit;
  }

  function getState(step: number): string {
    let allStates: string[] = [];

    states.map((timeStep) => (
      allStates.push(JSON.stringify(timeStep.state))
    ))

    return allStates[step];
  }

  function States({ dispGraph } : {dispGraph: string}) {
    let state = getState(stepNumber) ? JSON.parse(getState(stepNumber)) : null
    console.log("hejsan" + dispGraph)

    let seriesLabel: string;
    let seriesDatakey: string;
    let dataColor: string;

    let dataset = [{}];

    if(dispGraph === "Probabilities") {
      seriesLabel = 'Probability';
      seriesDatakey = 'probability';
      dataColor = '#08c49f';
      if (state !== null){
        dataset = getStatesOrProbabilities(true, state);
      }
    }else {
      seriesLabel = 'Amplitude';
      seriesDatakey = 'amplitude';
      dataColor = '#a208c4'
      if (state !== null){
        dataset = getStatesOrProbabilities(false, state);
      }
    }

    const valueFormatter = (value:any) => `${value}`;



    const chartSetting = {
      yAxis: [
        {
         min: 0, max: 1,
        },
      ],
      series: [{ dataKey: `${seriesDatakey}`, valueFormatter, label: `${seriesLabel}`}],
      height: 300,
      sx: {
        [`& .${axisClasses.directionY} .${axisClasses.label} `]: {
          transform: 'translateX(-10px)',
          fill: '#ffffff'
        },
        [`& .${axisClasses.left} .${axisClasses.tickLabel} `]: {
          fill: '#ffffff'
        },
        [`& .${axisClasses.directionY} .${axisClasses.line}`]: {
          stroke: '#ffffff'
        },
        [`& .${axisClasses.directionX} .${axisClasses.line}`]: {
          stroke: '#ffffff'
        },
        [`& .${axisClasses.directionY} .${axisClasses.tick}`]: {
          stroke: '#ffffff'
        },
        [`& .${axisClasses.directionX} .${axisClasses.tick}`]: {
          stroke: '#ffffff',
        },
        [`& .${axisClasses.directionX} .${axisClasses.tickLabel}`]: {
          transform: 'rotate(-90deg) translateX(-35px) translateY(-13px)',
          fill: '#ffffff'
        },
        [`& .${legendClasses.mark}`]: {
          fill: `${dataColor}`
        },
        [`& .${barElementClasses.root}`]: {
          fill: `${dataColor}`
        }
      }
    };

    const tickPlacement = 'middle';
    const tickLabelPlacement = 'middle';

    

    
  
    return (
      <section className="states">
        {/*<h2>{getState(stepNumber)}</h2>*/}
        <BarChart
        dataset={dataset}
        xAxis={[
          { scaleType: 'band', dataKey: 'bitstring', tickPlacement, tickLabelPlacement, tickLabelInterval: () => true},
        ]}
        margin={{
          top: 10,
          bottom: 60,
        }}
        slotProps={{
          legend : {
            labelStyle : {
              fill: '#ffffff'
            }
          }
        }}
        grid={{ horizontal: true }}
        tooltip={{trigger: 'item'}}
        {...chartSetting}
      />
      </section>
    );
}


  /*function swapMatrixItem(matrix:string[][], y:number, x:number, newItem:string){
    const newMatrix = matrix.map((line, i) => {
      if(i === y) {
        return (line.map((gate, j) => {
          if(j === x){
            return (newItem);
          } else{
            return (gate);
          }
        }));
      } else {
        return line;
      } 
    });
  }*/
}



export default App;

function getStatesOrProbabilities(returnProb: boolean, stateList: {re:number, im:number}[]): {}[] {
  let probabilities: {probability: number, bitstring: string}[] = [];
  let statevecs: {amplitude: number, bitstring: string}[] = [];
  let amplitude: number;
  let bitstring: string;
  let probability: number;

  if(returnProb) {
    for (let i = 0; i < stateList.length; i++) {
      bitstring = toBitString(i);
      probability = Math.round(((stateList[i].re)*(stateList[i].re) + Number.EPSILON) * 1000000) / 1000000;
      probabilities.push({probability: probability, bitstring: `${bitstring}`}) 
    }
    return probabilities;
  } else {
    for (let i = 0; i < stateList.length; i++) {
      bitstring = toBitString(i);
      amplitude = Math.round(((stateList[i].re) + Number.EPSILON) * 1000000) / 1000000;
      statevecs.push({amplitude: amplitude, bitstring: `${bitstring}`})
    }
    return statevecs;
  }

  
}

function toBitString(num: number): string {

  if (num === 0) {
      return "000000";
  }

  let result: string = "";

  while (num > 0) {
      result = (num & 1) + result;
      num >>= 1;
  }

  while (result.length < 6) {
      result = "0" + result;
  }

  return result;
}


