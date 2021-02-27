import React from 'react';
import {Button} from 'rimble-ui';
import styled from 'styled-components';

const WithdrawSection = () => {
    return(
        <WithdrawStyled>
            <Button size="large">Withdraw your funds</Button>
        </WithdrawStyled>
    );
}

const WithdrawStyled = styled.div`
    text-align: center;
`

export default WithdrawSection;
