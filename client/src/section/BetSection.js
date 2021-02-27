import React from "react";
import SelectSelection from "../component/SelectSelection";
import InputSection from "../component/InputSection";
import styled from "styled-components";

const BetSection = () => {
  return (
    <>
      <h1>Minimum amount: {}</h1>
      <BetSectionStyled>
        <SelectSelection />
        <InputSection />
      </BetSectionStyled>
    </>
  );
};

const BetSectionStyled = styled.div`
  display: flex;
  align-items: center;
  flex-wrap: nowrap;
  justify-content: space-around;
`;

export default BetSection;
