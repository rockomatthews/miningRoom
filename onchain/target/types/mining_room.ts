/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/mining_room.json`.
 */
export type MiningRoom = {
  "address": "61zXhvyP5jZhnj2X8XjT6S6Q1HJZk1wt4MPf9vVg89fq",
  "metadata": {
    "name": "miningRoom",
    "version": "0.1.0",
    "spec": "0.1.0"
  },
  "instructions": [
    {
      "name": "claimRewards",
      "discriminator": [
        4,
        144,
        132,
        71,
        116,
        23,
        151,
        80
      ],
      "accounts": [
        {
          "name": "owner",
          "writable": true,
          "signer": true,
          "relations": [
            "room"
          ]
        },
        {
          "name": "room",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  111,
                  111,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "miningPool",
          "writable": true
        },
        {
          "name": "globalConfig"
        },
        {
          "name": "stake",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "pendingHash",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  101,
                  110,
                  100,
                  105,
                  110,
                  103,
                  45,
                  104,
                  97,
                  115,
                  104
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "autoCompound",
          "type": "bool"
        }
      ]
    },
    {
      "name": "emergencyPause",
      "discriminator": [
        21,
        143,
        27,
        142,
        200,
        181,
        210,
        255
      ],
      "accounts": [
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "globalConfig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "paused",
          "type": "bool"
        }
      ]
    },
    {
      "name": "initialize",
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "globalConfig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "miningPool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  105,
                  110,
                  105,
                  110,
                  103,
                  45,
                  112,
                  111,
                  111,
                  108
                ]
              }
            ]
          }
        },
        {
          "name": "insuranceVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  110,
                  115,
                  117,
                  114,
                  97,
                  110,
                  99,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": {
              "name": "initializeParams"
            }
          }
        }
      ]
    },
    {
      "name": "purchaseMiner",
      "discriminator": [
        245,
        204,
        186,
        107,
        90,
        41,
        85,
        223
      ],
      "accounts": [
        {
          "name": "owner",
          "writable": true,
          "signer": true,
          "relations": [
            "room"
          ]
        },
        {
          "name": "room",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  111,
                  111,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "miner",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  105,
                  110,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "room"
              },
              {
                "kind": "account",
                "path": "room.next_miner_index",
                "account": "room"
              }
            ]
          }
        },
        {
          "name": "miningPool",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "model",
          "type": {
            "defined": {
              "name": "minerModel"
            }
          }
        }
      ]
    },
    {
      "name": "sellMiner",
      "discriminator": [
        150,
        235,
        200,
        199,
        249,
        41,
        103,
        8
      ],
      "accounts": [
        {
          "name": "owner",
          "signer": true,
          "relations": [
            "miner",
            "room"
          ]
        },
        {
          "name": "miner",
          "writable": true
        },
        {
          "name": "room",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  111,
                  111,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "stakeMrc",
      "discriminator": [
        125,
        178,
        187,
        122,
        49,
        236,
        146,
        33
      ],
      "accounts": [
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "mrcMint"
        },
        {
          "name": "payerMrcAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "owner"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "mrcMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "stake",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "stakeMrcAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "stake"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "mrcMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "lockDays",
          "type": "u16"
        }
      ]
    },
    {
      "name": "unlockSlot",
      "discriminator": [
        4,
        14,
        75,
        199,
        96,
        245,
        215,
        58
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "room",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  111,
                  111,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "miningPool",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "slotIndex",
          "type": "u8"
        }
      ]
    },
    {
      "name": "unstakeMrc",
      "discriminator": [
        111,
        184,
        242,
        68,
        241,
        238,
        33,
        254
      ],
      "accounts": [
        {
          "name": "owner",
          "signer": true
        },
        {
          "name": "stake",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  116,
                  97,
                  107,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "stakeMrcAccount",
          "writable": true
        },
        {
          "name": "ownerMrcAccount",
          "writable": true
        },
        {
          "name": "mrcMint"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": []
    },
    {
      "name": "updateParams",
      "discriminator": [
        108,
        178,
        190,
        95,
        94,
        203,
        116,
        20
      ],
      "accounts": [
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "globalConfig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": {
              "name": "updateParamsInput"
            }
          }
        }
      ]
    },
    {
      "name": "upgradeMiner",
      "discriminator": [
        93,
        174,
        185,
        203,
        165,
        148,
        94,
        13
      ],
      "accounts": [
        {
          "name": "owner",
          "signer": true,
          "relations": [
            "room",
            "miner"
          ]
        },
        {
          "name": "room",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  111,
                  111,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "miner",
          "writable": true
        }
      ],
      "args": [
        {
          "name": "upgrade",
          "type": {
            "defined": {
              "name": "upgradeKind"
            }
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "globalConfig",
      "discriminator": [
        149,
        8,
        156,
        202,
        160,
        252,
        176,
        217
      ]
    },
    {
      "name": "insuranceVault",
      "discriminator": [
        131,
        200,
        252,
        180,
        131,
        202,
        30,
        144
      ]
    },
    {
      "name": "minerAccount",
      "discriminator": [
        232,
        196,
        79,
        139,
        222,
        213,
        161,
        99
      ]
    },
    {
      "name": "miningPool",
      "discriminator": [
        134,
        149,
        193,
        160,
        11,
        139,
        229,
        253
      ]
    },
    {
      "name": "pendingHash",
      "discriminator": [
        123,
        216,
        82,
        151,
        159,
        99,
        3,
        7
      ]
    },
    {
      "name": "room",
      "discriminator": [
        156,
        199,
        67,
        27,
        222,
        23,
        185,
        94
      ]
    },
    {
      "name": "stakeAccount",
      "discriminator": [
        80,
        158,
        67,
        124,
        50,
        189,
        192,
        255
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "unauthorized",
      "msg": "Unauthorized action for caller"
    },
    {
      "code": 6001,
      "name": "insufficientRewards",
      "msg": "Insufficient rewards to claim"
    },
    {
      "code": 6002,
      "name": "invalidTokenAccount",
      "msg": "Token account does not match expected mint or owner"
    },
    {
      "code": 6003,
      "name": "invalidMint",
      "msg": "Mint does not match expected account"
    },
    {
      "code": 6004,
      "name": "invalidAmount",
      "msg": "Amount must be greater than zero"
    },
    {
      "code": 6005,
      "name": "invalidLockPeriod",
      "msg": "Lock period must be greater than zero"
    },
    {
      "code": 6006,
      "name": "stakeLocked",
      "msg": "Stake is still locked"
    },
    {
      "code": 6007,
      "name": "slotLimitReached",
      "msg": "No additional slots available"
    },
    {
      "code": 6008,
      "name": "invalidSlotIndex",
      "msg": "Slot index is invalid for this room"
    },
    {
      "code": 6009,
      "name": "noAvailableSlots",
      "msg": "No unlocked slots are free for another miner"
    },
    {
      "code": 6010,
      "name": "invalidRoomReference",
      "msg": "Miner does not belong to the provided room"
    },
    {
      "code": 6011,
      "name": "exceedsEmissionCap",
      "msg": "Requested amount exceeds the room's emission allowance"
    }
  ],
  "types": [
    {
      "name": "globalConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "mrcMint",
            "type": "pubkey"
          },
          {
            "name": "hashMint",
            "type": "pubkey"
          },
          {
            "name": "emissionRate",
            "type": "u64"
          },
          {
            "name": "coverTargetBps",
            "type": "u16"
          },
          {
            "name": "paused",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "initializeParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mrcMint",
            "type": "pubkey"
          },
          {
            "name": "hashMint",
            "type": "pubkey"
          },
          {
            "name": "emissionRate",
            "type": "u64"
          },
          {
            "name": "coverTargetBps",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "insuranceVault",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "reserveSol",
            "type": "u64"
          },
          {
            "name": "coverRatioBps",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "minerAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "room",
            "type": "pubkey"
          },
          {
            "name": "model",
            "type": {
              "defined": {
                "name": "minerModel"
              }
            }
          },
          {
            "name": "powerKw",
            "type": "u16"
          },
          {
            "name": "coolingPct",
            "type": "u16"
          },
          {
            "name": "riskBps",
            "type": "u16"
          },
          {
            "name": "status",
            "type": {
              "defined": {
                "name": "minerStatus"
              }
            }
          }
        ]
      }
    },
    {
      "name": "minerModel",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "basic"
          },
          {
            "name": "advanced"
          },
          {
            "name": "hyperscale"
          }
        ]
      }
    },
    {
      "name": "minerStatus",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "idle"
          },
          {
            "name": "mining"
          },
          {
            "name": "cooling"
          },
          {
            "name": "fried"
          }
        ]
      }
    },
    {
      "name": "miningPool",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "totalSol",
            "type": "u64"
          },
          {
            "name": "totalMrc",
            "type": "u64"
          },
          {
            "name": "hashLiability",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "pendingHash",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "room",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "slotsUnlocked",
            "type": "u8"
          },
          {
            "name": "nextMinerIndex",
            "type": "u8"
          },
          {
            "name": "totalPowerKw",
            "type": "u64"
          },
          {
            "name": "totalCoolingBps",
            "type": "u64"
          },
          {
            "name": "totalRiskBps",
            "type": "u64"
          },
          {
            "name": "minerCount",
            "type": "u8"
          },
          {
            "name": "accruedHash",
            "type": "u64"
          },
          {
            "name": "lastHarvestTs",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "stakeAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "lockEndTs",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "updateParamsInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "emissionRate",
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "coverTargetBps",
            "type": {
              "option": "u16"
            }
          },
          {
            "name": "paused",
            "type": {
              "option": "bool"
            }
          }
        ]
      }
    },
    {
      "name": "upgradeKind",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "power"
          },
          {
            "name": "cooling"
          },
          {
            "name": "firmware"
          }
        ]
      }
    }
  ]
};
