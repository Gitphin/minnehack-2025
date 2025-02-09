//component imports
// import TopNav from "./components/TopNav";
// import FoodGroups from "./components/FoodGroups"
// import React from "react";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import { Container } from "react-bootstrap";
import 'bootstrap/dist/css/bootstrap.min.css';
import 'bootstrap/dist/js/bootstrap.bundle.min.js';
import Home from "./pages/Home";
import RestaurantMap from "./pages/RestaurantMap";
import FoodShelters from "./pages/FoodShelters";
import FoodGroups from "./components/FoodGroups";
import FoodDrops from "./pages/FoodDrops.tsx";
import CreateFoodDrop from "./pages/CreateFoodDrop.tsx";
import ClaimFoodDrop from "./pages/ClaimFoodDrop.tsx";
import DeleteFoodDrop from "./pages/DeleteFoodDrop.tsx";

import './App.css';

function App() {
  return (
    <Router>
      <Container>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/restaurant-map" element={<RestaurantMap />} />
          <Route path="/list-of-food-shelters" element={<FoodShelters />} />
          <Route path="/be-a-volunteer-food-shelters" element={<FoodGroups />} />
          <Route path="/list-of-food-drops" element={<FoodDrops />} />
          <Route path="/create-food-drops" element={<CreateFoodDrop />} />
          <Route path="/claim-food-drops" element={<ClaimFoodDrop />} />
          <Route path="/claim-food-drops" element={<DeleteFoodDrop />} />
        </Routes>
      </Container>
    </Router>
  );
}
export default App;
