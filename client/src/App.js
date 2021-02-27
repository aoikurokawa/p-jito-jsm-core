import React, { useEffect, useState } from "react";
import Web3 from "web3";

import { ADDRESS, ABI } from "./config";
import GlobalStyle from "./component/GlobalStyle";
import Nav from "./component/Nav";
import DisplayModal from "./component/Modal";
import DepositSection from "./section/DepositSection";
import BetSection from './section/BetSection';
import WithdrawSection from './section/WithdrawSection';

import "./styles/app.scss";

function App() {
  const [account, setAccount] = useState("");
  const [balance, setBalance] = useState();
  const [contractInstance, setContractInstance] = useState();
  const [isModalVisible, setIsModalVisible] = useState(false);
  const [title, setTitle] = useState("");
  const [hash, setHash] = useState("");
  const [deposit, setDeposit] = useState(0);

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
        setBalance(Web3.utils.fromWei(result, "ether"));
      });
    contractInstance.methods
      .getPlayerBalance(window.ethereum.selectedAddress)
      .call()
      .then((res) => {
        console.log(res);
      });
  };

  const betHandler = async (price) => {
    if (price === 0) {
      alert("should be more than zero!!");
    } else {
      let config = {
        value: Web3.utils.toWei(price, "ether"),
        from: account[0],
      };
      contractInstance.methods
        .bet()
        .send(config)
        .on("transactionHash", (hash) => {
          setHash(hash);
        })
        .on("receipt", (receipt) => {
          contractInstance.methods
            .balance()
            .call()
            .then((result) => {
              if (result > balance) {
                setTitle("You Win");
                showModal();
              } else {
                setTitle("You Lose");
                showModal();
              }
              setBalance(Web3.utils.fromWei(result, "ether"));
            });
        });
    }
  };

  const showModal = () => {
    setIsModalVisible(true);
  };

  const handleOk = () => {
    setIsModalVisible(false);
  };

  return (
    <div className="App">
      <GlobalStyle />
      <Nav account={account} balance={balance} />
      <DepositSection deposit={deposit} />
      <BetSection />
      <WithdrawSection />
      <DisplayModal
        isModalVisible={isModalVisible}
        handleOk={handleOk}
        title={title}
        hash={hash}
      />
    </div>
  );
}

export default App;
