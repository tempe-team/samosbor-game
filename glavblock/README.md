# Glavblock

Game in [Samosbor](https://samosb.org/) setting. Bastard of economical strategy and visual novel.

# Docs

Various game design digits in [spreadsheet](https://docs.google.com/spreadsheets/d/1PA18gcbbeIUVYdINowk_PRhOiLDzaMh0UOmgDVoPoxM/edit#gid=0)

# Roadmap

- [ ] Economical strategy element
  - [ ] Colony with stats
    - [ ] People
      - [ ] Profession
        - [ ] Profession model
        - [ ] Build power of particular human
        - [ ] Change profession logic
        - [ ] Experience (build power incrementaion)
      - [ ] Equip
        - [ ] Respirator
        - [ ] Clothes
        - [ ] Inhaler
        - [ ] HogweedSuit
        - [ ] ConcreteSuit
        - [ ] Bulletproof
        - [ ] Helmet
        - [ ] ExoskeletonU
        - [ ] ExoskeletonZ
      - [ ] Weapon
        - [ ] BattleRake
        - [ ] Flamethrower
        - [ ] Granit
        - [ ] PG6
        - [ ] Grinder
        - [ ] RPL
        - [ ] AE
        - [ ] PPS
        - [ ] PM
    - [ ] Resources
    - [ ] Area
      - [ ] Undistributed
      - [ ] Living
      - [ ] Infrastructure
      - [ ] Science
      - [ ] Military
      - [ ] Party
      - [ ] Medical
    - [ ] Stationary objects (like buildings in regular strategy games)
      - [ ] Bench
      - [ ] TurnMill
      - [ ] FormatFurnace
      - [ ] ChemLab
      - [ ] BioLab
      - [ ] Barrel
      - [ ] ElectronicsLab
      - [ ] MolecularPrinter
      - [ ] NeuroTerminal
      - [ ] OperatingRoom
      - [ ] GermT1
      - [ ] GermT2
      - [ ] GermT3
      - [ ] Generator
      - [ ] AeroPump
      - [ ] WaterPump
      - [ ] VoidScanner
    - [ ] Ecology
      - [ ] Air imputity
      - [ ] Water impurity
    - [ ] Colony init
  - [ ] Update-per-turn logic
    - [ ] Stationary objects degradation(run-out)
    - [ ] Resources
      - [ ] Production and consumption
        - [ ] Income from stalkers
        - [ ] Resources consumption
        - [ ] Production tree (buildings)
        - [ ] Resources degradation
  - [ ] [Macroquad](https://github.com/not-fl3/macroquad) interface
    - [ ] "next turn" button and colony stats
    - [ ] Control elements
      - [ ] Tasks - what to build/make
      - [ ] People list
        - [ ] Profession control
    - [ ] Human info interface
        - [ ] Equip
        - [ ] Statuses
          - [ ] Hunger
          - [ ] Morality
          - [ ] Fatigue
          - [ ] Sickness
- [ ] Events (Visual novel element)
- [ ] Content server
  - [ ] Colony perks
  - [ ] Quests
  - [ ] Events
    - [ ] Interface
  - [ ] Script language integration ([gluon](https://github.com/gluon-lang/gluon)? [rhai](https://github.com/jonathandturner/rhai)? lua? [Something else](https://github.com/ruse-lang/langs-in-rust)?)
  - [ ] Events api in core library
  - [ ] Http (or maybe graphql?) server whitch serves events sets. Contextually.
- [ ] CI/CD
