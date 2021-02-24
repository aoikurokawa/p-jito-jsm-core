
const initialState = {
  account: "",
  balance: 0,
  contractInstance: null,
  hash: "",
};

const blockchainReducer = (state = initialState, action) => {
  switch (action.type) {
    case "GET_CONTRACTDATA":
      return {
        ...state,
        account: action.payload.account,
        balance: action.payload.balance,
        contractInstance: action.payload.contractInstance,
      };

    default:
      return {
        ...state,
      };
  }
};

export default blockchainReducer;
