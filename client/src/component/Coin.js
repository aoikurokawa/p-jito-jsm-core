import React, { useState } from "react";
import styled from "styled-components";

const Coin = ({betHandler}) => {

  const [price, setPrice] = useState(0);

  let head = true;

  const coinTossHandler = () => {
    // console.log(price);
    betHandler(price);
    let flipResult = Math.random();
    let coinCLass = document.getElementById("coin");
    coinCLass.classList.remove();
    setTimeout(() => {
      if(flipResult <= 0.5) {
        head = true;
        coinCLass.classList.add("heads");
        console.log("head");
      } else {
        head = false;
        coinCLass.classList.add("tails");
        console.log("tails");
      }
    }, 100);
  };

  return (
    <div>
      <div id="coin" className={`${head ? "heads" : "tails"}`} onClick={coinTossHandler}>
        <div className="side-a"></div>
        <div className="side-b"></div>
      </div>
      <input type="number" placeholder="Type the price" value={price} onChange={(e) => setPrice(e.target.value)} />
      <ButtonStyled onClick={coinTossHandler}>Bet</ButtonStyled>
    </div>
  );
};



const ButtonStyled = styled.div`
  background-color: #e9c46a;
  color: #fff;
  padding: 15px 15px;
  border-radius: 30px;
  display: inline-block;
  transition: 0.5s;
  cursor: pointer;

  :hover {
    opacity: 0.5;
    background-color: #ef233c;
  }
`;



export default Coin;
