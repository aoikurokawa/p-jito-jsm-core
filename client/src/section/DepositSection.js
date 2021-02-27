import React from "react";
import styled from "styled-components";

const DepositSection = ({ deposit }) => {
  return (
    <DepositStyled>
      <input type="number" value={deposit} placeholder="Type the price you want to deposit" />
      <label>ETH</label>
      <button type="button">Deposit</button>
    </DepositStyled>
  );
};

const DepositStyled = styled.div`
  color: white;
  text-align: center;
  padding: 2rem;
  input {
    font-size: 2rem;
    padding: 0.5rem;
  }
  input:focus {
    outline: none;
  }
  label {
    font-size: 2rem;
    margin-left: 1rem;
  }
  button {
    margin-left: 10rem;
    border-radius: 1rem;
  }
`;

export default DepositSection;
