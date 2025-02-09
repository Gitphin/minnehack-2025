// import React from "react";

const TopNav = () => {
    return (
        <nav className="navbar navbar-expand-lg">
            <div className="container-fluid">
                <a className="navbar-brand" href="#">Yum Yum Yummy</a>
                <button 
                    className="navbar-toggler" 
                    type="button" 
                    data-bs-toggle="collapse" 
                    data-bs-target="#navbarSupportedContent" 
                    aria-controls="navbarSupportedContent" 
                    aria-expanded="false" 
                    aria-label="Toggle navigation"
                >
                    <span className="navbar-toggler-icon"></span>
                </button>
                <div className="collapse navbar-collapse" id="navbarSupportedContent">
                    <ul className="navbar-nav me-auto mb-2 mb-lg-0">
                        <li className="nav-item">
                            <a className="nav-link active" aria-current="page" href="#">Home</a>
                        </li>
                        <li className="nav-item">
                            <a className="nav-link" href="#">Filtered Map</a>
                        </li>
                        <li className="nav-item dropdown">
                            <a className="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                                Food Shelters
                            </a>
                            <ul className="dropdown-menu">
                                <li><a className="dropdown-item" href="#">List of Food Shelters</a></li>
                                <li><a className="dropdown-item" href="#">Be a Volunteer</a></li>
                                {/* <li><hr className="dropdown-divider" /></li> */}
                                {/* <li><a className="dropdown-item" href="#">Something else here</a></li> */}
                            </ul>
                        </li>
                        <li className="nav-item dropdown">
                            <a className="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                                Food Drops
                            </a>
                            <ul className="dropdown-menu">
                                <li><a className="dropdown-item" href="#">List of Food Drops</a></li>
                                <li><a className="dropdown-item" href="#">Be a Volunteer</a></li>
                            </ul>
                        </li>
                        {/* <li className="nav-item">
                            <a className="nav-link disabled" aria-disabled="true">Disabled</a>
                        </li> */}
                    </ul>
                    {/* <form className="d-flex" role="search">
                        <input className="form-control me-2" type="search" placeholder="Search" aria-label="Search" />
                        <button className="btn btn-outline-success" type="submit">Search</button>
                    </form> */}
                </div>
            </div>
        </nav>
    );
};

export default TopNav;
