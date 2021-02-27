import React, { useState } from "react";
import styled from "styled-components";

const InputSection = ({ betHandler }) => {
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
      <InputSectionStyled>
        <input
          type="number"
          placeholder="Type the price"
          value={price}
          onChange={(e) => setPrice(e.target.value)}
        />
        <label>ETH</label>
        <ButtonStyled onClick={coinTossHandler}>Bet</ButtonStyled>
      </InputSectionStyled>
    </div>
  );
};

const InputSectionStyled = styled.div`
  padding: 1rem;
  input {
    padding: 1rem 0rem;
    font-size: 2rem;
    border-radius: 1rem;
    text-align: center;
  }
  input:focus {
    outline: none;
  }
  label {
    color: white;
    font-size: 2rem;
    margin-left: 1rem;
  }
`;

const ButtonStyled = styled.div`
  color: white;
  border: 3px solid #23d997;
  padding: 3rem;
  border-radius: 3rem;
  transition: 0.5s;
  cursor: pointer;
  margin-top: 5rem;
  font-size: 2rem;
  text-align: center;
  :hover {
    background-color: #23d997;
    color: white;
}
`;

export default InputSection;
