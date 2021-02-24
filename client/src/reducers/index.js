import {combineReducers} from 'redux';

import blockchainReducer from './blockchainReducer';

const rootReducer = combineReducers({
    blockchain: blockchainReducer, 
}); 

export default rootReducer;
