import Crossword from "react-crossword-near";
import { parseSolutionSeedPhrase } from "./utils";
import { createGridData, loadGuess } from "react-crossword-near/dist/es/util";
import sha256 from "js-sha256";
import './App.css';
import { useCallback, useRef, useState } from "react";

function App({ data, solutionHash }) {
  const crossword = useRef();
  const [solutionFound, setSolutionFound] = useState("Not correct yet");

  const onCrosswordComplete = useCallback(async (completeCount) => {
    if (completeCount !== false) {
      let gridData = createGridData(data).gridData;
      loadGuess(gridData, 'guesses');
      await checkSolution(gridData);
    }
  }, []);

  async function checkSolution(gridData) {
    let seedPhrase = parseSolutionSeedPhrase(data, gridData);
    let answerHash = sha256.sha256(seedPhrase);

    if (answerHash === solutionHash) {
      console.log("You're correct!");
      setSolutionFound("Correct!");
    } else {
      console.log("That's not the correct solution. :/");
      setSolutionFound("Not correct yet");
    }
  }

  return (
    <div className="page">
      <h1>NEAR Crossword Puzzle</h1>
      <div id="crossword-wrapper">
        <h3>Status: {solutionFound}</h3>
        <Crossword
          data={data}
          ref={crossword}
          onCrosswordComplete={onCrosswordComplete}
        />
      </div>
      <footer>
        <p>Thank you</p>
      </footer>
    </div>
  );
}

export default App;
