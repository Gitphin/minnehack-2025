import { useEffect, useState } from "react";
import TopNav from "../components/TopNav";
import "bootstrap/dist/css/bootstrap.min.css";
import "bootstrap/dist/js/bootstrap.bundle.min.js";

// Define TypeScript interface for food shelter data
interface FoodShelter {
  name: string;
  address: string;
  business_status: string;
  geometry: {
    location: {
      lat: number;
      lng: number;
    };
  };
  rating: number;
  user_ratings_total: number;
  photo_url: string;
}

const FoodShelterMap: React.FC = () => {
  const [foodShelters, setFoodShelters] = useState<FoodShelter[]>([]);

  useEffect(() => {
    fetch("http://localhost:3001/find_food_shelters") // Adjust API endpoint
      .then((response) => response.json())
      .then((data) => {
        console.log("Fetched Food Shelters:", data); // Debugging

        if (!Array.isArray(data)) {
          console.error("Unexpected data format:", data);
          return;
        }

        const formattedData = data.map((shelter: any) => ({
          name: shelter.name,
          address: shelter.vicinity,
          business_status: shelter.business_status,
          geometry: shelter.geometry,
          rating: shelter.rating ?? "N/A",
          user_ratings_total: shelter.user_ratings_total ?? 0,
          photo_url: shelter.photos?.[0]?.photo_reference
            ? `https://maps.googleapis.com/maps/api/place/photo?maxwidth=400&photoreference=${shelter.photos[0].photo_reference}&key=YOUR_GOOGLE_API_KEY`
            : "",
        }));

        console.log("Processed Food Shelters:", formattedData);
        setFoodShelters(formattedData);
      })
      .catch((error) => {
        console.error("Error fetching food shelters:", error);
      });
  }, []);

  return (
    <>
      <div className="top-nav-container">
        <TopNav />
      </div>
      <div className="container mt-4">
        <h2>Nearby Food Shelters</h2>
        <ul className="list-group">
          {foodShelters.length === 0 ? (
            <li className="list-group-item">No food shelters found.</li>
          ) : (
            foodShelters.map((shelter, index) => (
              <li key={index} className="list-group-item">
                <div className="d-flex">
                  <div className="flex-shrink-0">
                    {shelter.photo_url ? (
                      <img
                        src={shelter.photo_url}
                        alt={shelter.name}
                        className="img-thumbnail"
                        width={100}
                      />
                    ) : (
                      <div style={{ width: 100, height: 100, backgroundColor: "#f0f0f0" }} />
                    )}
                  </div>
                  <div className="ms-3">
                    <strong>{shelter.name}</strong> - {shelter.address}
                    <p>Rating: {shelter.rating} ({shelter.user_ratings_total} reviews)</p>
                    <p>Status: {shelter.business_status}</p>
                  </div>
                </div>
              </li>
            ))
          )}
        </ul>
      </div>
    </>
  );
};

export default FoodShelterMap;
