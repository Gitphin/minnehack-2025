import React, { useEffect, useState } from "react";
import TopNav from "../components/TopNav";
import { Link } from "react-router-dom";
import axios from "axios";
import "./fooddroplist.css";  // Import the styles

interface FoodDrop {
  id: string;
  name: string;
  food_type: string;
}

const FoodDrops: React.FC = () => {
  const [foodDrops, setFoodDrops] = useState<FoodDrop[]>([]);

  useEffect(() => {
    const fetchFoodDrops = async () => {
      try {
        const response = await axios.get("http://localhost:3001/events");
        setFoodDrops(response.data);
      } catch (error) {
        console.error("Error fetching events:", error);
      }
    };

    fetchFoodDrops();
  }, []);

  return (
    <div className="food-drops-container">
      <TopNav />
      <h2 className="quick">Available Food Drops</h2>
      <ul className="food-list">
        {foodDrops.map((drop) => (
          <li key={drop.id} className="food-item">
            <strong>{drop.name}</strong> ({drop.food_type})
            <div>
              <Link to={`events/claim/${drop.id}`}>Claim</Link> | 
              <Link to={`events/delete/${drop.id}`}>Delete</Link>
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default FoodDrops;

