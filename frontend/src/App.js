import logo from "./logo.svg";
import "./App.css";
import { useEffect } from "react";
import axios from "axios";

function App() {
  const fetchStudents = () => {
    //
    axios
      .get("http://localhost:8080/api/students")
      .then((res) => console.log(res.data)); // -> error

    // axios.get("http://backend:8080").then((res) => console.log(res.data)); // -> error
  };

  useEffect(() => {
    fetchStudents();
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React?
        </a>
      </header>
    </div>
  );
}

export default App;
