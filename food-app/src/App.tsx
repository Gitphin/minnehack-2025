import './App.css';
//component imports
import TopNav from "./components/TopNav";
import FoodGroups from "./components/FoodGroups"
import "bootstrap/dist/css/bootstrap.min.css";
import "bootstrap/dist/js/bootstrap.bundle.min.js";  // Ensure JS is loaded

//image imports
import HappyPeopleWithFood from './images/happy-people-with-food.jpg';

function App() {
  return (
    <>
    <div className='top-nav-container'>
      <TopNav />
    </div>

    <img src={HappyPeopleWithFood} alt="Happy People With Food" id="happy-people-image"></img>
    </>
  );
}

export default App;
