import { useEffect, useState } from "react";
import TopNav from "../components/TopNav";
import "bootstrap/dist/css/bootstrap.min.css";
import "bootstrap/dist/js/bootstrap.bundle.min.js";

// Define TypeScript interface for restaurant data
interface Restaurant {
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
  price_level: number;
  photo_url: string;
}

const RestaurantMap: React.FC = () => {
  const [restaurants, setRestaurants] = useState<Restaurant[]>([]); // Type the state with Restaurant[]

useEffect(() => {
  fetch("http://localhost:3001/find_restaurants")
    .then((response) => response.json())
    .then((data) => {
      console.log("Fetched Data:", data); // Debugging Step 1
      if (!Array.isArray(data)) {
        console.error("Unexpected data format:", data);
        return;
      }

      const updatedData = data.map((restaurant: any) => ({
        name: restaurant.name,
        address: restaurant.vicinity, 
        business_status: restaurant.business_status,
        geometry: restaurant.geometry,
        rating: restaurant.rating,
        user_ratings_total: restaurant.user_ratings_total,
        price_level: restaurant.price_level ?? "N/A", // Handle missing price_level
        photo_url: restaurant.photos?.[0]?.photo_reference 
          ? `https://maps.googleapis.com/maps/api/place/photo?maxwidth=400&photoreference=${restaurant.photos[0].photo_reference}&key=YOUR_GOOGLE_API_KEY`
          : "", 
      }));

      console.log("Processed Data:", updatedData); // Debugging Step 2
      setRestaurants(updatedData);
    })
    .catch((error) => {
      console.error("Error fetching data:", error);
    });
}, []);



  return (
    <>
      <div className="top-nav-container">
        <TopNav />
      </div>
      <div className="container mt-4">
        <h2>Nearby Restaurants</h2>
        {/* Map through the restaurants and display them */}
        <ul className="list-group">
          {restaurants.length === 0 ? (
            <li className="list-group-item">No restaurants found.</li>
          ) : (
            restaurants.map((restaurant, index) => (
              <li key={index} className="list-group-item">
                <div className="d-flex">
                  <div className="flex-shrink-0">
                    {restaurant.photo_url ? (
                      <img
                        src={restaurant.photo_url}
                        alt={restaurant.name}
                        className="img-thumbnail"
                        width={100}
                      />
                    ) : (
                      <div style={{ width: 100, height: 100, backgroundColor: "#f0f0f0" }} />
                    )}
                  </div>
                  <div className="ms-3">
                    <strong>{restaurant.name}</strong> - {restaurant.address}
                    <p>Rating: {restaurant.rating} ({restaurant.user_ratings_total} ratings)</p>
                    <p>Price Level: {restaurant.price_level}</p>
                    <p>Status: {restaurant.business_status}</p>
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

export default RestaurantMap;
