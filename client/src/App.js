import React, { useEffect, useState } from 'react';
import Web3 from 'web3';

import {ADDRESS, ABI} from './config';
import GlobalStyle from './component/GlobalStyle';
import Coin from './component/Coin';
import Balance from './component/Balance';

function App() {

  const [account, setAccount] = useState("");

  useEffect(() => {
    loadBlockchain();
  }, []);

  const loadBlockchain = async () => {
    const web3 = new Web3(Web3.givenProvider || "http://localhost:8545");
    const accounts = await web3.eth.getAccounts();
    setAccount(accounts);
    const contractInstance = new web3.eth.Contract(ABI, ADDRESS);
    console.log(contractInstance);
  }

  return (
    <div className="App">
      <GlobalStyle />
      <Balance account={account} />
      <Coin />
    </div>
  );
}

export default App;
