// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract DataProofRegistry {
    address public owner;
    
    // Mapping: Batch-ID => Merkle Root (bytes32)
    mapping(uint256 => bytes32) public batchMerkleRoots;
    uint256 public batchCount;

    event BatchRegistered(uint256 indexed batchId, bytes32 merkleRoot, uint256 timestamp);

    modifier onlyOwner() {
        require(msg.sender == owner, "Caller is not the owner");
        _;
    }

    constructor() {
        owner = msg.sender;
    }

    /// @notice Registriert den Merkle Root eines 1.000er-Blocks auf der Blockchain
    function registerBatch(bytes32 _merkleRoot) external onlyOwner {
        batchCount++;
        batchMerkleRoots[batchCount] = _merkleRoot;
        
        emit BatchRegistered(batchCount, _merkleRoot, block.timestamp);
    }

    /// @notice Verifiziert, ob ein spezifischer Datensatz Teil eines registrierten Merkle Roots ist
    function verifyProof(
        bytes32[] calldata proof,
        bytes32 root,
        bytes32 leaf
    ) public pure returns (bool) {
        bytes32 computedHash = leaf;

        for (uint256 i = 0; i < proof.length; i++) {
            bytes32 proofElement = proof[i];

            if (computedHash <= proofElement) {
                computedHash = keccak256(abi.encodePacked(computedHash, proofElement));
            } elseWir können das System so erweitern, dass die Merkle Roots automatisch auf der Blockchain verankert werden!

---

### 1. Der Solidity Smart Contract (`DataProofRegistry.sol`)

Hier ist ein kompakter Smart Contract, den du auf Ethereum, Arbitrum oder Polygon deployen kannst:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract DataProofRegistry {
    address public owner;
    
    mapping(uint256 => bytes32) public batchMerkleRoots;
    uint256 public batchCount;

    event BatchRegistered(uint256 indexed batchId, bytes32 merkleRoot, uint256 timestamp);

    constructor() {
        owner = msg.sender;
    }

    function registerBatch(bytes32 _merkleRoot) external {
        require(msg.sender == owner, "Only owner can register");
        batchCount++;
        batchMerkleRoots[batchCount] = _merkleRoot;
        emit BatchRegistered(batchCount, _merkleRoot, block.timestamp);
    }
}
