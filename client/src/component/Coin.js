import React from "react";
import styled from "styled-components";

const Coin = () => {
  let heads = 0;
  let tails = 0;
  let win = true;

  const coinTossHandler = () => {
    let x = Math.floor(Math.random() * 2);
    if (x === 0) {
        win = true;
        heads += 1;
    } else {
        win = false;
        tails += 1;
    }
  };

  return (
    <div>
      <CoinStyled className="coin">
        {win ? (
          <AnimateCoin
            class="heads animate-coin"
            src="https://lenadesignorg.files.wordpress.com/2020/06/head.png?w=100"
          />
        ) : (
          <AnimateCoin
            class="tails animate-coin"
            src="https://lenadesignorg.files.wordpress.com/2020/06/tail.png?w=100"
          />
        )}
      </CoinStyled>
      <ButtonStyled onClick={coinTossHandler}>Toss coin</ButtonStyled>
    </div>
  );
};

const CoinStyled = styled.div`
  position: relative;
`;

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

const AnimateCoin = styled.img`
    animation: flip 2s 1;
    @keyframes flip {
        0% {
          transform: scale3d(1,1,1) rotateX(0deg);
          
        }
        50% {
          transform: scale3d(1,1,1) rotateX(3600deg);
          
        }
        100% {
          transform: scale3d(1,1,1) rotateX(7200deg);
          
        }
      }
`

export default Coin;
