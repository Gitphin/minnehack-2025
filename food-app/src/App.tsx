//component imports
import TopNav from "./components/TopNav";
import FoodGroups from "./components/FoodGroups"
import "bootstrap/dist/css/bootstrap.min.css";
import "bootstrap/dist/js/bootstrap.bundle.min.js";  // Ensure JS is loaded
import './App.css';

//image imports
import HappyPeopleWithFood from './images/happy-people-image.avif';

function App() {
  return (
    <>
    <div className='top-nav-container'>
      <TopNav />
    </div>

    <div className="row-1-container">
      <div className="happy-people-image-container">
        <img src={HappyPeopleWithFood} alt="Happy People With Food" id="happy-people-image"></img>
      </div>
    </div>
    </>
  );
}

export default App;
