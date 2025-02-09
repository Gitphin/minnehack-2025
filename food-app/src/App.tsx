import './App.css';
//component imports
import TopNav from "./components/TopNav";
import FoodGroups from "./components/FoodGroups"

//image imports
import HappyPeopleWithFood from './images/happy-people-with-food.jpg';

function App() {
  return (
    <>
    <div>
      <TopNav />
    </div>

    <img src={HappyPeopleWithFood} alt="Happy People With Food" id="happy-people-image"></img>
    </>
  );
}

export default App;
