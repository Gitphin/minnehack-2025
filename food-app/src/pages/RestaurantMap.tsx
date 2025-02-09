import TopNav from "../components/TopNav";
import "bootstrap/dist/css/bootstrap.min.css";
import "bootstrap/dist/js/bootstrap.bundle.min.js";  // Ensure JS is loaded
// import './FilteredMap.css';

function RestaurantMap() {  
  return (
    <>
    <div className='top-nav-container'>
      <TopNav />
    </div>
    </>
  );
};

export default RestaurantMap;