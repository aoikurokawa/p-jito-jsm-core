import React from 'react';
import {Button} from 'rimble-ui';
import styled from 'styled-components';

const WithdrawSection = ({ showModal, setTitle}) => {

    const withdrawClickHandler = () => {
        showModal("withdraw");
        setTitle("Are you sure you want to withdraw?");
    }

    return(
        <WithdrawStyled>
            <Button size="large" onClick={withdrawClickHandler}>Withdraw your funds</Button>
        </WithdrawStyled>
    );
}

const WithdrawStyled = styled.div`
    text-align: center;
    margin-top: 8rem;
`

export default WithdrawSection;
