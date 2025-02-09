import React, { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import axios from "axios";
import TopNav from "../components/TopNav";

const ClaimFoodDrop: React.FC = () => {
  const { eventId } = useParams<{ eventId: string }>();
  const [qrCode, setQrCode] = useState("");

  useEffect(() => {
    const claimEvent = async () => {
      try {
        const response = await axios.delete(`http://verygay.world:3001/events/${eventId}/claim`);
        setQrCode(response.data.code);
      } catch (error) {
        console.error("Error claiming event:", error);
      }
    };

    claimEvent();
  }, [eventId]);

  return (
    <div>
      <TopNav />
      <h2>Claim Food Drop</h2>
      {qrCode ? <img src={`http://verygay.world:3001${qrCode}`} alt="QR Code" /> : <p>Claiming food drop...</p>}
    </div>
  );
};

export default ClaimFoodDrop;
