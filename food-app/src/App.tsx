//component imports
import TopNav from "./components/TopNav";
// import FoodGroups from "./components/FoodGroups"
// import React from "react";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import { Container } from "react-bootstrap";
import Home from "./pages/Home";
// import About from "./pages/About";
// import Contact from "./pages/Contact";

import './App.css';



function App() {
  return (
    <Router>
      <div className='top-nav-container'>
        <TopNav />
      </div>

      <Container>
        <Routes>
          <Route path="/" element={<Home />} />
          {/* <Route path="/about" element={<About />} /> */}
          {/* <Route path="/contact" element={<Contact />} /> */}
        </Routes>
      </Container>
    </Router>
  );
}
export default App;
