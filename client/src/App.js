import React, { useEffect, useState } from "react";
import Web3 from "web3";

import { ADDRESS, ABI } from "./config";
import GlobalStyle from "./component/GlobalStyle";
import Nav from "./component/Nav";
import Coin from "./component/Coin";
import Balance from "./component/Balance";

import "./styles/app.scss";

function App() {
  const [account, setAccount] = useState("");
  const [balance, setBalance] = useState();
  const [contractInstance, setContractInstance] = useState();

  useEffect(() => {
    loadBlockchain();
  }, []);

  const loadBlockchain = async () => {
    const web3 = new Web3(Web3.givenProvider || "http://localhost:8545");
    const accounts = await web3.eth.getAccounts();
    setAccount(accounts);
    const contractInstance = new web3.eth.Contract(ABI, ADDRESS);
    setContractInstance(contractInstance);
    contractInstance.methods
      .balance()
      .call()
      .then((result) => {
        setBalance(Web3.utils.fromWei(result, "ether"));
      });
  };

  const betHandler = async (price) => {
    // console.log(price);
    let config = {
      value: Web3.utils.toWei(price, "ether"),
      from: account[0],
    };
    contractInstance.methods
      .bet()
      .send(config)
      .on("transactionHash", (hash) => {
        console.log(hash);
      })
      .on("confirmation", (confirmationNr) => {
        console.log(confirmationNr);
      })
      .on("receipt", (receipt) => {
        console.log(receipt);
        alert("Done!");
      });
      contractInstance.methods
      .balance()
      .call()
      .then((result) => {
        setBalance(Web3.utils.fromWei(result, "ether"));
      });
  };

  return (
    <div className="App">
      <GlobalStyle />
      <Nav account={account} balance={balance} />
      <Balance balance={balance} />
      <Coin betHandler={betHandler} />
    </div>
  );
}

export default App;
