import './foodgroups.css'
import TopNav from "../components/TopNav";

const items = [
  { id: 1, name: 'The Food Group' , href: "https://www.thefoodgroupmn.org/"},
  { id: 2, name: 'Greater Twin Cities United Way', href: "https://www.gtcuw.org/" },
  { id: 3, name: 'Hunger Solutions' , href: "https://www.hungersolutions.org/"},
  { id: 4, name: 'Salvation Army' , href: "https://www.salvationarmyusa.org/usn/"},
  { id: 5, name: 'Second Harvest Heartland', href: "https://www.2harvest.org/" },
  { id: 6, name: 'Supplemental Nutrition Assistance Program (SNAP)' , href: "https://www.fns.usda.gov/snap/supplemental-nutrition-assistance-program"},
];

function FoodGroups() {
  return (
    <div className="container">
      <TopNav />
      <ul className="list-group">
        {items.map((item) => (
          <a href={item.href}>
          <li key={item.id} className="list-item">
            {item.name}
          </li>
          </a>
        ))}
      </ul>
    </div>
  );
}

export default FoodGroups;

