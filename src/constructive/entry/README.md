# Entry
An `Entry` is a higher-level construct that groups together one or more `Combinator`. It acts as a container for specific actions, such as calling smart contracts or transferring value, which collectively influence the global state.

## Entry Tree
                                                    
     ┌────────────────────────┐                              ┌────────────────────────┐     
     │ Uppermost Left Branch  │                              │ Uppermost Right Branch │
     │ b:0 => off             │                              │ b:0 => off             │
     │ b:1 => on              │                              │ b:1 => on              │
     └────────────────────────┘                              └────────────────────────┘        
            ┌────┘└────┐                        ┌────────────────────────┘└────────────────────────┐
    ┌────────────┐┌────────────┐    ┌──────────────────────┐                            ┌──────────────────────┐
    │ Liftup     ││ Recharge   │    │ Transactive Branch   │                            │ Upper Right Branch   │  
    │ b:0 => off ││ b:0 => off │    │ b:0                  │                            │ b:1                  │
    │ b:1 => on  ││ b:1 => on  │    └──────────────────────┘                            └──────────────────────┘
    └────────────┘└────────────┘          ┌────┘└────┐                       ┌─────────────────────┘└─────────────────────┐
                                    ┌──────────┐┌──────────┐     ┌──────────────────────┐                     ┌──────────────────────┐
                                    │ Move     ││ Call     │     │ Liquidity Branch     │                     │ Right Branch         │  
                                    │ b:0      ││ b:1      │     │ b:0                  │                     │ b:1                  │
                                    └──────────┘└──────────┘     └──────────────────────┘                     └──────────────────────┘
                                                                       ┌────┘└────┐                    ┌─────────────────┘└─────────────────┐
                                                                 ┌──────────┐┌──────────┐  ┌──────────────────────┐              ┌──────────────────────┐
                                                                 │ Add      ││ Sub      │  │ Lower Left Branch    │              │ Lower Right Branch   │
                                                                 │ b:0      ││ b:1      │  │ b:0                  │              │ b:1                  │
                                                                 └──────────┘└──────────┘  └──────────────────────┘              └──────────────────────┘
                                                                                                 ┌────┘└────┐                   ┌───────────┘└───────────┐         
                                                                                           ┌──────────┐┌──────────┐ ┌──────────────────────┐ ┌──────────────────────┐
                                                                                           │ Deploy   ││ Swapout  │ │ Recovery Branch      │ │ Reserved             │
                                                                                           │ b:0      ││ b:1      │ │ b:0                  │ │ b:1                  │
                                                                                           └──────────┘└──────────┘ └──────────────────────┘ └──────────────────────┘
                                                                                                                          ┌────┘└────┐       
                                                                                                                    ┌──────────┐┌──────────┐ 
                                                                                                                    │ Revive   ││ Claim    │ 
                                                                                                                    │ b:0      ││ b:1      │ 
                                                                                                                    └──────────┘└──────────┘ 



- `Uppermost Left Branch` and `Uppermost Right Branch` can be both set to `on`.
- `Uppermost Left Branch` can be set to `on` and `Uppermost Right Branch` be set to `off`.
- `Uppermost Left Branch` can be set to `off` and `Uppermost Right Branch` be set to `on`.

- If `Uppermost Left Branch` set to `on`;
    - 1. `Liftup` and `Recharge` can be both set to `on`.
    - 2. `Liftup` can be set to `on` and `Recharge` be set to `off`.
    - 3. `Liftup` can be set to `off` and `Recharge` be set to `on`.

- If `Uppermost Right Branch` is set to `on`;
    - 1. `Transactive Branch` can be set to `on` and `Upper Right Branch` be set to `off`.
    - 2. `Transactive Branch` can be set to `off` and `Upper Right Branch` be set to `on`.

    - If `Transactive Branch` is set to `on`;
        - 1. `Move` can be set to `on` and `Call` be set to `off`.
        - 2. `Move` can be set to `off` and `Call` be set to `on`.

    - If `Upper Right Branch` is set to `on`;
        - 1. `Liquidity Branch` can be set to `on` and `Right Branch` be set to `off`.
        - 2. `Liquidity Branch` can be set to `off` and `Right Branch` be set to `on`.

            - If `Liquidity Branch` is set to `on`;
                - 1. `Add` can be set to `on` and `Remove` be set to `off`.
                - 2. `Add` can be set to `off` and `Remove` be set to `on`.

            - If `Right Branch` is set to `on`;
                - 1. `Lower Left Branch` can be set to `on` and `Lower Right Branch` be set to `off`.
                - 2. `Lower Left Branch` can be set to `off` and `Lower Right Branch` be set to `on`.

                - If `Lower Left Branch` is set to `on`;
                    - 1. `Deploy` can be set to `on` and `Swapout` be set to `off`.
                    - 2. `Deploy` can be set to `off` and `Swapout` be set to `on`.

                - If `Lower Right Branch` is set to `on`;
                    - 1. `Recovery Branch` can be set to `on` and `Reserved Branch` be set to `off`.
                    - 2. `Recovery Branch` can be set to `off` and `Reserved Branch` be set to `on`.

                    - If `Recovery Branch` is set to `on`;
                        - 1. `Revive` can be set to `on` and `Claim` be set to `off`.
                        - 2. `Revive` can be set to `off` and `Claim` be set to `on`.

                    - If `Reserved Branch` is set to `on` the entry fails.