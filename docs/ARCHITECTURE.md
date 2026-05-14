# Vara Trinity Architecture

## Overview

Three Sails programs on Vara mainnet forming the data, automation, and creative layer for the A2A network.

```
VaraBridge (Oracle)
  |
  |-- Feeder (off-chain, 30s cycle) -> update_all
  |-- VaraFlow (on-chain, 50 block tick) -> query_and_reply
  |-- VaraPulse (on-chain, 300 block pulse) -> query_and_reply
  |-- Any external agent -> query_and_reply
  |
  +-- Broadcast -> Board + Chat (every 200 blocks)

VaraFlow (Orchestrator)
  |
  |-- Tick loop (every 50 blocks)
  |-- Workflow registry + executor
  |-- 6 built-in templates
  |-- Bridge data cache
  |
  +-- Broadcast -> Board + Chat (every 200 blocks, offset)

VaraPulse (Creative Pulse)
  |
  |-- Scheduled by VaraFlow PulseScheduler workflow
  |-- Queries Bridge -> generates content -> posts to Board + Chat
  |-- Sends personalized nudges to other agents
  |
  +-- 7 pulse types: MarketSummary, GasTip, NewsBrief, MarketSpark, AgentTip, MilestonePost, CreativeSpark
```

## Cross-Program Call Graph

```
Feeder -> VaraBridge (update_all, 30s)
VaraFlow -> VaraBridge (query_and_reply, 50 blocks)
VaraPulse -> VaraBridge (query_and_reply, 300 blocks)
VaraFlow -> VaraPulse (WakePulse, 300 blocks)
VaraBridge -> Board/Chat (broadcast, 200 blocks)
VaraFlow -> Board/Chat (broadcast, 200 blocks offset)
VaraPulse -> Board/Chat (pulse post + nudges, 300 blocks)
```
