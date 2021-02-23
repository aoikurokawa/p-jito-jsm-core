// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.9.0;

contract CoinFlip {
    event betWin(bool win);

    modifier isOwner() {
        require(msg.sender == owner, "Caller is not owner");
        _;
    }

    address owner;

    uint256 public balance;

    constructor() public{
        owner = msg.sender;
    }

    function bet() public payable isOwner {
        uint256 randomNumber = _random();
        if (randomNumber == 0) {
            emit betWin(false);
            _lose();
        } else {
            emit betWin(true);
            _win();
        }
    }

    function _random() public view returns (uint256) {
        return block.timestamp % 2;
    }

    function _win() private {
        uint256 betPrice = msg.value;
        balance = balance + betPrice * 2;
    }

    function _lose() private {
        uint256 betPrice = msg.value;
        balance -= betPrice;
    }
}
