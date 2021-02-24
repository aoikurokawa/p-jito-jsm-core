import Web3 from "web3";
import { ABI, ADDRESS } from "../config";

export const loadBlockchain = async (dispatch) => {
  const web3 = new Web3(Web3.givenProvider || "http://localhost:8545");
  const accountData = await web3.eth.getAccounts();
  const contractData = new web3.eth.Contract(ABI, ADDRESS);
  const balanceData = await contractData.methods.balance().call();

  dispatch({
    type: "GET_CONTRACTDATA",
    payload: {
      account: accountData,
      balance: balanceData,
      contractInstance: contractData,
    },
  });
};
