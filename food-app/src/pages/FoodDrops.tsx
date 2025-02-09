import React, { useEffect, useState } from "react";
import TopNav from "../components/TopNav";
import { Link } from "react-router-dom";
import axios from "axios";

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
        const response = await axios.get("http://verygay.world:3001/events");
        setFoodDrops(response.data);
      } catch (error) {
        console.error("Error fetching events:", error);
      }
    };

    fetchFoodDrops();
  }, []);

  return (
    <div>
      <TopNav />
      <h2>Available Food Drops</h2>
      <ul>
        {foodDrops.map((drop) => (
          <li key={drop.id}>
            <strong>{drop.name}</strong> ({drop.food_type}) -{" "}
            <Link to={`/claim/${drop.id}`}>Claim</Link> |{" "}
            <Link to={`/delete/${drop.id}`}>Delete</Link>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default FoodDrops;
