// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title Mallen DataProofRegistry — Anclaje Diario de Árboles de Merkle para Fecha Cierta (SCJN)
contract DataProofRegistry {
    address public immutable owner;
    mapping(bytes32 => uint256) public batchTimestamps;

    event RootCommitted(bytes32 indexed merkleRoot, uint256 timestamp, address indexed relayer);

    modifier onlyOwner() {
        require(msg.sender == owner, "No autorizado");
        _;
    }

    constructor() {
        owner = msg.sender;
    }

    function registerBatch(bytes32 _merkleRoot) external onlyOwner {
        require(_merkleRoot != bytes32(0), "Root invalido");
        require(batchTimestamps[_merkleRoot] == 0, "Root ya registrado");
        batchTimestamps[_merkleRoot] = block.timestamp;
        emit RootCommitted(_merkleRoot, block.timestamp, msg.sender);
    }
}
