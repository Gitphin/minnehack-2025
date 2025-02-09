import './foodgroups.css'
import TopNav from "../components/TopNav";

const items = [
  { id: 1, name: 'The Food Group' },
  { id: 2, name: 'Greater Twin Cities United Way' },
  { id: 3, name: 'Hunger Solutions' },
  { id: 4, name: 'Salvation Army' },
  { id: 5, name: 'Second Harvest Heartland' },
  { id: 6, name: 'Supplemental Nutrition Assistance Program (SNAP)' },
];

function FoodGroups() {
  return (
    <div className="container">
      <TopNav />
      <ul className="list-group">
        {items.map((item) => (
          <li key={item.id} className="list-item">
            {item.name}
          </li>
        ))}
      </ul>
    </div>
  );
}

export default FoodGroups;

