pragma circom 2.2.1;

// Adapted from:
// https://gist.github.com/simon-something/b705500d534f25289e9f5db1ffb275d6

include "poseidon.circom";
include "comparators.circom";
include "mux1.circom";

// Based on a lead, its index and siblings in a lean incremental
// Merkle tree, re-compute the root of the tree
template LeanIMTInclusionProof(maxDepth) {
    signal input leaf;
    signal input leafIndex;
    signal input siblings[maxDepth];

    signal output out;

    signal nodes[maxDepth + 1]; 
    signal indices[maxDepth];

    component siblingIsEmpty[maxDepth]; 
    component hashInCorrectOrder[maxDepth];
    component latestValidHash[maxDepth];
    component poseidons[maxDepth];

    // Convert leaf index to their path
    component indexToPath = Num2Bits(maxDepth);
    indexToPath.in <== leafIndex;
    indices <== indexToPath.out;

    nodes[0] <== leaf;

    for (var i = 0; i < maxDepth; i++) {
        var childrenToSort[2][2] = [ [nodes[i], siblings[i]], [siblings[i], nodes[i]] ];
        hashInCorrectOrder[i] = MultiMux1(2);
        hashInCorrectOrder[i].c <== childrenToSort;
        hashInCorrectOrder[i].s <== indices[i];

        poseidons[i] = Poseidon(2);
        poseidons[i].inputs <== hashInCorrectOrder[i].out;

        // difference with an IMT is the proof/siblings array has a
        // variable length, as a single node is propagated to the
        // level above (and not hashed with a placeholder value or 0).
        siblingIsEmpty[i] = IsZero();
        siblingIsEmpty[i].in <== siblings[i];

        // Either keep the previous hash (no more siblings) or the new one.
        nodes[i + 1] <== (nodes[i] - poseidons[i].out) * siblingIsEmpty[i].out + poseidons[i].out;
    }

    out <== nodes[maxDepth];
}

component main {public [leaf]} = LeanIMTInclusionProof(14);
