import React, { useEffect, useState } from "react";
import Web3 from "web3";
import { Loader } from "rimble-ui";

import { ADDRESS, ABI } from "./config";
import GlobalStyle from "./component/GlobalStyle";
import Nav from "./component/Nav";
import DisplayModal from "./component/Modal";
import DepositSection from "./section/DepositSection";
import BetSection from "./section/BetSection";
import WithdrawSection from "./section/WithdrawSection";
import styled from "styled-components";

function App() {
  const [account, setAccount] = useState("");
  const [contractBalance, setContractBalance] = useState();
  const [gameBalance, setGameBalance] = useState();
  const [minimumAmount, setMinimumAmount] = useState();
  const [contractInstance, setContractInstance] = useState();
  const [isModalVisible, setIsModalVisible] = useState(false);
  const [title, setTitle] = useState("");
  const [type, setType] = useState("");
  const [hash, setHash] = useState("");
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    loadBlockchain();
  }, []);

  const loadBlockchain = async () => {
    const web3 = new Web3(Web3.givenProvider || "http://localhost:8545");
    const accounts = await web3.eth.getAccounts();
    setAccount(accounts);
    const contractInstance = new web3.eth.Contract(ABI, ADDRESS, {
      from: accounts[0],
    });
    console.log(contractInstance);
    setContractInstance(contractInstance);
    contractInstance.methods
      .balance()
      .call()
      .then((result) => {
        setContractBalance(Web3.utils.fromWei(result, "ether"));
      });
    contractInstance.methods
      .getPlayerBalance(window.ethereum.selectedAddress)
      .call()
      .then((res) => {
        setGameBalance(Web3.utils.fromWei(res, "ether"));
      });
    contractInstance.methods
      .minimumBetNumber()
      .call()
      .then((res) => {
        setMinimumAmount(Web3.utils.fromWei(res, "ether"));
        setIsLoading(false);
      });
  };

  const depositHandler = (depositAmount) => {
    if (depositAmount === 0) {
      alert("should be more than zero!");
    } else {
      let config = {
        value: Web3.utils.toWei(depositAmount, "ether"),
        from: account[0],
      };
      contractInstance.methods
        .deposit()
        .send(config)
        .on("transactionHash", (hash) => {
          showModal("deposit");
          setHash(hash);
          setTitle(`You deposit ${depositAmount} ETH`);
        });
    }
  };

  const betHandler = (choice, price) => {
    if (price <= 0) {
      alert("should be more than zero!");
    } else {
      let config = {
        value: Web3.utils.toWei(price, "ether"),
        from: account[0],
      };
      contractInstance.methods
        .bet(choice)
        .send(config)
        .on("transactionHash", (hash) => {
          console.log(hash);
        })
        .on("receipt", (receipt) => {
          console.log(receipt);
          alert("You betted!");
        })
    }
  };

  const withdrawHandler = () => {
    console.log("withdraw");
    contractInstance.methods
      .withdrawFunds()
      .send()
      .on("transactionHash", (hash) => {
        console.log(hash);
      })
      .on("receipt", (receipt) => {
        console.log(receipt);
        let amt = receipt.events.WithdrawnFundsFromPlayer.returnValues(
          "amount"
        );
        alert(
          `You received ${Web3.utils.fromWei(
            amt,
            "ether"
          )} eth! Check your Metamask wallet`
        );
      });
  };

  const showModal = (type) => {
    setIsModalVisible(true);
    setType(type);
  };

  const handleOk = () => {
    if (type === "withdraw") {
      withdrawHandler();
    }

    setIsModalVisible(false);
    setTitle("");
    setHash("");
    setType("");
  };

  const handleCancel = () => {
    setIsModalVisible(false);
    setTitle("");
    setHash("");
    setType("");
  };

  return (
    <div className="App">
      <GlobalStyle />
      {isLoading ? (
        <LoadingStyled>
          <Loader color="primary" size="80px" />
        </LoadingStyled>
      ) : (
        <>
          <Nav
            account={account}
            contractBalance={contractBalance}
            gameBalance={gameBalance}
          />
          <DepositSection depositHandler={depositHandler} />
          <BetSection minimumAmount={minimumAmount} betHandler={betHandler} />
          <WithdrawSection
            showModal={showModal}
            setTitle={setTitle}
            setType={setType}
          />
          <DisplayModal
            isModalVisible={isModalVisible}
            handleOk={handleOk}
            handleCancel={handleCancel}
            title={title}
            hash={hash}
          />
        </>
      )}
    </div>
  );
}

const LoadingStyled = styled.div`
  display: flex;
  justify-content: space-around;
  height: 100vh;
  align-items: center;
`;

export default App;
