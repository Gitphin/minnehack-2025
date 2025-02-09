import React, { useState } from "react";
import TopNav from "../components/TopNav";
import axios from "axios";
import "react-datepicker/dist/react-datepicker.css";
import "./createpage.css"; // Import CSS

const CreateFoodDrop: React.FC = () => {
  const [name, setName] = useState("");
  const [address, setAddress] = useState("");
  const [foodType, setFoodType] = useState("");
  const [maxClaims, setMaxClaims] = useState(1);
  const [deadline, _setDeadline] = useState<Date | null>(null);

  const handleCreate = async () => {
    try {
      const response = await axios.post("http://verygay.world:3001/events/create", {
        name,
        address,
        food_type: foodType,
        max_claims: maxClaims,
        deadline: deadline ? Math.floor(deadline.getTime() / 1000) : null,
      });

      alert(`Food drop created! Event ID: ${response.data.id}`);
    } catch (error) {
      alert("Error creating event");
      console.error(error);
    }
  };

  return (
    <div className="create-page-container">
      <TopNav />
      <h2>Create a Food Drop</h2>
      <input
        placeholder="Restaurant Name"
        value={name}
        onChange={(e) => setName(e.target.value)}
      />
      <input
        placeholder="Address"
        value={address}
        onChange={(e) => setAddress(e.target.value)}
      />
      <input
        placeholder="Food Type"
        value={foodType}
        onChange={(e) => setFoodType(e.target.value)}
      />
      <input
        type="number"
        placeholder="Max Claims"
        value={maxClaims}
        onChange={(e) => setMaxClaims(Number(e.target.value))}
      />
      <button onClick={handleCreate}>Create</button>
    </div>
  );
};

export default CreateFoodDrop;

