import React from 'react';
import styled from 'styled-components';

const Nav = ({account, balance}) => {
    return(
        <NavSstyled>
            <h1>Dapps - Coin Flip</h1>
            <ul>
                {/* <li>Your Account: {account}</li> */}
                <li>Contract Balance: {balance} ETH</li>
                <li>Your balance in game: ETH</li>
            </ul>
        </NavSstyled>
    );
};

const NavSstyled = styled.nav`
    min-height: 10vh;
    margin: auto;
    // display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 10rem;
    background: #282828;
    position: sticky;
    top: 0;
    z-index: 10;
    h1 {
        color: white;
        font-size: 1.5rem;
        font-family: "Lobster", cursive;
        font-weight: lighter;
    }
    ul {
        display: flex;
        list-style: none;
    }
    li {
        padding-left: 7rem;
        position: relative;
        color: white;
        text-decoration: none;
        font-size: 1.5rem;
    }
    @media (max-width: 1300px) {
        flex-direction: column;
        padding: 1rem;
        #Logo {
            display: inline-block;
            margin: 2rem;
        }
        ul {
            padding: 2rem;
            justify-content: space-around;
            width: 100%;
            li {
                padding: 0;
            }
        }
    }
`;

export default Nav;
