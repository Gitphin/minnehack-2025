import React, { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import axios from "axios";
import TopNav from "../components/TopNav";

const DeleteFoodDrop: React.FC = () => {
  const { eventId } = useParams<{ eventId: string }>();
  const [deleteId, setDeleteId] = useState("");

  useEffect(() => {
    const fetchEvent = async () => {
      try {
        const response = await axios.get(`http://verygay.world:3001/events/${eventId}`);
        setDeleteId(response.data.delete_id);
      } catch (error) {
        console.error("Error fetching event:", error);
      }
    };

    fetchEvent();
  }, [eventId]);

  const handleDelete = async () => {
    try {
      await axios.delete(`http://verygay.world:3001/events/${eventId}/${deleteId}`);
      alert("Event deleted successfully.");
    } catch (error) {
      alert("Error deleting event.");
      console.error(error);
    }
  };

  return (
    <div>
      <TopNav />
      <h2>Delete Food Drop</h2>
      <button onClick={handleDelete}>Confirm Delete</button>
    </div>
  );
};

export default DeleteFoodDrop;
