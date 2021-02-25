import React, { useState } from "react";
import styled from "styled-components";

const Coin = ({ betHandler }) => {
  const [price, setPrice] = useState(0);

  let head = true;

  const coinTossHandler = () => {
    // console.log(price);
    betHandler(price);
    let flipResult = Math.random();
    let coinCLass = document.getElementById("coin");
    coinCLass.classList.remove();
    setTimeout(() => {
      if (flipResult <= 0.5) {
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
      <div id="coin" className={`${head ? "heads" : "tails"}`}>
        <div className="side-a"></div>
        <div className="side-b"></div>
      </div>
      <InputSection>
        <input
          type="number"
          placeholder="Type the price"
          value={price}
          onChange={(e) => setPrice(e.target.value)}
        />
        <ButtonStyled onClick={coinTossHandler}>Bet</ButtonStyled>
      </InputSection>
    </div>
  );
};

const InputSection = styled.div`
  text-align: center;
  padding: 1rem;
  input {
    padding: 2rem 3rem;
    font-size: 2rem;
    width: 100%;
    border-radius: 2rem;
  }
  input:focus {
    outline: none;
  }
`;

const ButtonStyled = styled.div`
  background-color: #e9c46a;
  color: #fff;
  padding: 3rem;
  border-radius: 20rem;
  transition: 0.5s;
  cursor: pointer;
  margin-top: 5rem;
  font-size: 2rem;

  :hover {
    opacity: 0.5;
    background-color: #ef233c;
  }
`;

export default Coin;
