import TopNav from "../components/TopNav";
import FoodGroup from "../components/FoodGroups"
import "bootstrap/dist/css/bootstrap.min.css";
import "bootstrap/dist/js/bootstrap.bundle.min.js";  // Ensure JS is loaded
// import './FilteredMap.css';

function FoodSheltersVolunteer() {  
  return (
    <>
    <div className='top-nav-container'>
      <TopNav />
      <FoodGroup />
    </div>
    </>
  );
};

export default FoodSheltersVolunteer;
