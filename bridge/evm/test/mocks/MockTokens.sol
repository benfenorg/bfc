// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockWBTC is ERC20 {
    constructor() ERC20("Wrapped Bitcoin", "wBTC") {}

    function mint(address to, uint256 amount) public virtual {
        _mint(to, amount);
    }

    function burn(address form, uint256 amount) public virtual {
        _burn(form, amount);
    }

    function decimals() public view virtual override returns (uint8) {
        return 8;
    }

    function testSkip() public {}
}

contract MockBNBUSDC is ERC20 {
    constructor() ERC20("USD Coin", "USDC") {}

    function mint(address to, uint256 amount) public virtual {
        _mint(to, amount);
    }

    function burn(address form, uint256 amount) public virtual {
        _burn(form, amount);
    }

    function decimals() public view virtual override returns (uint8) {
        return 18;
    }

    function testSkip() public {}
}

contract MockBNBUSDT is ERC20 {
    constructor() ERC20("Tether", "USDT") {}

    function mint(address to, uint256 amount) public virtual {
        _mint(to, amount);
    }

    function burn(address form, uint256 amount) public virtual {
        _burn(form, amount);
    }

    function decimals() public view virtual override returns (uint8) {
        return 18;
    }

    function testSkip() public {}
}


contract MockUSDC is ERC20 {
    constructor() ERC20("USD Coin", "USDC") {}

    function mint(address to, uint256 amount) public virtual {
        _mint(to, amount);
    }

    function burn(address form, uint256 amount) public virtual {
        _burn(form, amount);
    }

    function decimals() public view virtual override returns (uint8) {
        return 6;
    }

    function testSkip() public {}
}


contract MockAAVELPToken is ERC20 {
    constructor() ERC20("AAVE LP Token", "AAVE-LP") {}

    function mint(address to, uint256 amount) public virtual {
        _mint(to, amount);
    }

    function burn(address form, uint256 amount) public virtual {
        _burn(form, amount);
    }

    function decimals() public view virtual override returns (uint8) {
        return 6;
    }

    function testSkip() public {}
}


contract MockLPToken is ERC20 {
    constructor() ERC20("LP Token", "LP") {}

    function mint(address to, uint256 amount) public virtual {
        _mint(to, amount);
    }

    function burn(address from, uint256 amount) public virtual {
        _burn(from, amount);
    }

    function decimals() public view virtual override returns (uint8) {
        return 6;
    }

    function testSkip() public {}
}

contract MockUSDT is ERC20 {
    constructor() ERC20("Tether", "USDT") {}

    function mint(address to, uint256 amount) public virtual {
        _mint(to, amount);
    }

    function burn(address form, uint256 amount) public virtual {
        _burn(form, amount);
    }

    function decimals() public view virtual override returns (uint8) {
        return 6;
    }

    function testSkip() public {}
}

contract MockKA is ERC20 {
    constructor() ERC20("Ka Coin", "KA") {}

    function mint(address to, uint256 amount) public virtual {
        _mint(to, amount);
    }

    function burn(address form, uint256 amount) public virtual {
        _burn(form, amount);
    }

    function decimals() public view virtual override returns (uint8) {
        return 9;
    }

    function testSkip() public {}
}

contract MockBUSD is ERC20 {
    constructor() ERC20("BUSD", "BUSD") {}

    function mint(address to, uint256 amount) public virtual {
        _mint(to, amount);
    }

    function burn(address form, uint256 amount) public virtual {
        _burn(form, amount);
    }

    function decimals() public view virtual override returns (uint8) {
        return 9;
    }

    function testSkip() public {}
}

contract MockBNB is ERC20 {
    constructor() ERC20("BNB", "BNB") {}

    function mint(address to, uint256 amount) public virtual {
        _mint(to, amount);
    }

    function burn(address form, uint256 amount) public virtual {
        _burn(form, amount);
    }

    function decimals() public view virtual override returns (uint8) {
        return 18;
    }

    function testSkip() public {}
}

contract WETH {
    string public name = "Wrapped Ether";
    string public symbol = "WETH";
    uint8 public decimals = 18;

    event Approval(address indexed src, address indexed guy, uint256 wad);
    event Transfer(address indexed src, address indexed dst, uint256 wad);
    event Deposit(address indexed dst, uint256 wad);
    event Withdrawal(address indexed src, uint256 wad);

    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    function deposit() public payable {
        balanceOf[msg.sender] += msg.value;
        emit Deposit(msg.sender, msg.value);
    }

    function withdraw(uint256 wad) public {
        require(balanceOf[msg.sender] >= wad);
        balanceOf[msg.sender] -= wad;
        payable(msg.sender).transfer(wad);
        emit Withdrawal(msg.sender, wad);
    }

    function totalSupply() public view returns (uint256) {
        return address(this).balance;
    }

    function approve(address guy, uint256 wad) public returns (bool) {
        allowance[msg.sender][guy] = wad;
        emit Approval(msg.sender, guy, wad);
        return true;
    }

    function transfer(address dst, uint256 wad) public returns (bool) {
        return transferFrom(msg.sender, dst, wad);
    }

    function transferFrom(address src, address dst, uint256 wad) public returns (bool) {
        require(balanceOf[src] >= wad);

        if (src != msg.sender) {
            require(allowance[src][msg.sender] >= wad);
            allowance[src][msg.sender] -= wad;
        }

        balanceOf[src] -= wad;
        balanceOf[dst] += wad;

        emit Transfer(src, dst, wad);

        return true;
    }

    function testSkip() public {}
}


contract WBNB {
    string public name     = "Wrapped BNB";
    string public symbol   = "WBNB";
    uint8  public decimals = 18;

   event Approval(address indexed src, address indexed guy, uint256 wad);
    event Transfer(address indexed src, address indexed dst, uint256 wad);
    event Deposit(address indexed dst, uint256 wad);
    event Withdrawal(address indexed src, uint256 wad);

    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    function deposit() public payable {
        balanceOf[msg.sender] += msg.value;
        emit Deposit(msg.sender, msg.value);
    }

    function withdraw(uint256 wad) public {
        require(balanceOf[msg.sender] >= wad);
        balanceOf[msg.sender] -= wad;
        payable(msg.sender).transfer(wad);
        emit Withdrawal(msg.sender, wad);
    }

    function totalSupply() public view returns (uint256) {
        return address(this).balance;
    }

    function approve(address guy, uint256 wad) public returns (bool) {
        allowance[msg.sender][guy] = wad;
        emit Approval(msg.sender, guy, wad);
        return true;
    }

    function transfer(address dst, uint256 wad) public returns (bool) {
        return transferFrom(msg.sender, dst, wad);
    }

    function transferFrom(address src, address dst, uint256 wad) public returns (bool) {
        require(balanceOf[src] >= wad);

        if (src != msg.sender) {
            require(allowance[src][msg.sender] >= wad);
            allowance[src][msg.sender] -= wad;
        }

        balanceOf[src] -= wad;
        balanceOf[dst] += wad;

        emit Transfer(src, dst, wad);

        return true;
    }
    function testSkip() public {}
}
