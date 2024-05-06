import React, { useState, useEffect } from 'react';
import './circuitboard.css';
import './toolbar.css';
import Toolbar from './toolbar';
import {DndContext} from '@dnd-kit/core';
import axios from 'axios';
import Circuitboard from './circuitboard';
import './slider.css';
import './app.css';
import { BarChart, barElementClasses } from '@mui/x-charts/BarChart';
import { axisClasses } from '@mui/x-charts/ChartsAxis';
import { chartsTooltipClasses, legendClasses } from '@mui/x-charts';


function App() {
  type Circuit = string[][];

  // Function to initialize the circuit with a variable number of "I"s
  const initializeCircuit = (rows: number, columns: number, initialValue: string): Circuit => {
      const circuit: Circuit = [];
      for (let i = 0; i < rows; i++) {
          const row: string[] = [];
          for (let j = 0; j < columns; j++) {
              row.push(initialValue);
          }
          circuit.push(row);
      }
      return circuit;
  }

  // This matrix doesn't contain actual elements, just information about what the circuit looks like.
  const [circuit, setCircuit] = useState<Circuit>(() => initializeCircuit(6, 25, "I"));  // Initializing this because it complains about type otherwise, there is probably a better way to do it.
  const [states, setStates] = useState([{"0":1, "1":0}]);
  const [stepNumber, setStepNumber] = useState(25);
  const [displayedGraph, setDisplayedGraph] = useState("Probabilities");

  //Oracle visibility work around
  const [isOracleVisible, setIsOracleVisible] = useState(false);
  const [isUniVisible, setIsUniVisible] = useState(false);


  const changeGraph = (e:any) => {
    setDisplayedGraph(e.target!.value);
  }
  const onChange = (e:any) => {
    sendCircuit();
    setStepNumber(e.target!.value)
  }

  useEffect(() => {
    // This effect will be triggered whenever the circuit state changes
    sendCircuit();
  }, [circuit,stepNumber]);


  return (
    <div className="App">
      <DndContext onDragEnd={handleDragEnd}>
        <div className='circuit-tools'>
          <Toolbar setCircuit={setCircuit} setIsOracleVisible={setIsOracleVisible} setIsUniVisible={setIsUniVisible} />
          <div className='circuit-slider'>
            <Circuitboard circuit={circuit} setCircuit={setCircuit} sendCircuit={sendCircuit} isOracleVisible={isOracleVisible} setIsOracleVisible={setIsOracleVisible} isUniVisible={isUniVisible} setIsUniVisible={setIsUniVisible}/>
            <div className='slider-container'>
              <input
                type='range'
                min={1}
                max={25}
                defaultValue={25}
                step={1}
                className='range'
                onChange={onChange}
              />
              <div className='step-numbers'>
                <p>1</p>
                <p>2</p>
                <p>3</p>
                <p>4</p>
                <p>5</p>
                <p>6</p>
                <p>7</p>
                <p>8</p>
                <p>9</p>
                <p>10</p>
                <p>11</p>
                <p>12</p>
                <p>13</p>
                <p>14</p>
                <p>15</p>
                <p>16</p>
                <p>17</p>
                <p>18</p>
                <p>19</p>
                <p>20</p>
                <p>21</p>
                <p>22</p>
                <p>23</p>
                <p>24</p>
                <p>25</p>
              </div>
            </div>
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
    if(over === null){
      return;
    }
    if(active.id === "C_down"){
      if(over.id[0] === 5){
        alert("No gate to control.");
        return;
      }
      if(circuit[parseInt(over.id[0]) + 1][parseInt(over.id.substring(1))] === "I"){
        alert("No gate to control.");
        return;
      }
    }

    /*if(active.id === "Swap"){
      let placable = true;
      for (let i = 0; i < circuit.length; i++) {
        if (circuit[i].includes("Swap")) {
          if (over.id.substring(1) !== JSON.stringify(circuit[i].indexOf("Swap")) || (Math.abs(over.id[0]) - i) > 1) {
            alert("Second swap must be placed on the qubit directly below the first");
            placable = false;
          }
        }
        
      }
      if(!placable){
        return;
      }
    }*/

    console.log("Placed gate on position " + over.id.substring(1) + " on qubit line " + over.id[0]);

    // These nested maps replace the gate at the given position.
    const newCircuit = circuit.map((line, i) => {
      if(i === (Number(over.id[0]))) {
        return (line.map((gate, j) => {
          if(j === (Number(over.id.substring(1)))){
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
    console.log("Circuit:")
    console.log(JSON.stringify(circuit));
    const response = await axios.post('http://localhost:8000/simulate',
        {circuit_matrix: circuit})
  .then(function(response: any){
    console.log("statelist")
    console.log(response.data.state_list);
    setStates(response.data.state_list[stepNumber].col.data);
  })}

  function States({ dispGraph } : {dispGraph: string}) {

    let seriesLabel: string;
    let seriesDatakey: string;
    let dataColor: string;

    let dataset = [{}];

    if(dispGraph === "Probabilities") {
      seriesLabel = 'Probability';
      seriesDatakey = 'probability';
      dataColor = '#08c49f';
      if (states !== null){
        dataset = getStatesOrProbabilities(true, states);
      }
    }else {
      seriesLabel = 'Amplitude';
      seriesDatakey = 'amplitude';
      dataColor = '#a208c4'
      if (states !== null){
        dataset = getStatesOrProbabilities(false, states);
      }
    }

    const valueFormatter = (value:any) => `${value}`;



    const chartSetting = {
      yAxis: [
        {
         min: 0, max: 1, label: `${seriesLabel}`
        },
      ],
      series: [{ dataKey: `${seriesDatakey}`, valueFormatter, label: `${seriesLabel}`}],
      height: 415,
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
        [`& .${axisClasses.directionX} .${axisClasses.label}`]: {
          fill: '#ffffff',
          transform: 'translateY(40px)'
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
        },
        [`& .${chartsTooltipClasses.mark}`]: {
          stroke: `${dataColor}`,
          display: 'none'
        }
      }
    };

    const tickPlacement = 'middle';
    const tickLabelPlacement = 'middle';
  
    return (
      <section className="states">
        <BarChart
        dataset={dataset}
        xAxis={[
          { scaleType: 'band', dataKey: 'bitstring', label: 'Computational basis states', tickPlacement, tickLabelPlacement, tickLabelInterval: () => true},
        ]}
        margin={{
          top: 10,
          bottom: 90,
        }}
        slotProps={{
          legend : {
            labelStyle : {
              fill: '#ffffff'
            }
          },
        }}
        grid={{ horizontal: true }}
        tooltip={{trigger: 'item' }}
        {...chartSetting}
      />
      </section>
    );
}
}



export default App;

function getStatesOrProbabilities(returnProb: boolean, stateList: {0:number, 1:number}[]): {}[] {
  let probabilities: {probability: number, bitstring: string}[] = [];
  let statevecs: {amplitude: number, bitstring: string}[] = [];
  let amplitude: number;
  let bitstring: string;
  let probability: number;

  if(returnProb) {
    for (let i = 0; i < stateList.length; i++) {
      bitstring = toBitString(i);
      if (stateList[i][1] !== 0) {
        probability = Math.round(((stateList[i][1])*(stateList[i][1]) + Number.EPSILON) * 1000000) / 1000000;
      }else{
        probability = Math.round(((stateList[i][0])*(stateList[i][0]) + Number.EPSILON) * 1000000) / 1000000;
      }
      probabilities.push({probability: probability, bitstring: `${bitstring}`}) 
    }
    return probabilities;
  } else {
    for (let i = 0; i < stateList.length; i++) {
      bitstring = toBitString(i);
      if (stateList[i][1] !== 0) {
        amplitude = Math.round(((Math.abs(stateList[i][1])) + Number.EPSILON) * 1000000) / 1000000;
      }else {
        amplitude = Math.round(((Math.abs(stateList[i][0])) + Number.EPSILON) * 1000000) / 1000000;
      }
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





