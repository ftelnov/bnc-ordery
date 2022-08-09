What's here?
---

Simple rust application that provides casual binance order management.

Just Rust exercise!

Plan
---

1) Fetch current active orders of the provided account - display it in UI. Table will be enough;
2) Fetch current balances of all the balances - display it in UI. Table will be enough;
3) Provide a way of spreading whole balance of specific symbol into N amount of orders in the depth of symbol. 
   So this function should create N(specified by user) orders of the symbol S(specified by user) in the depth of symbol(somewhere in the end of order book). Display this function in UI with a simple popup - would be enough I suppose.
4) Provide a way of canceling for those accounts.
    It should of course allow some sort of logging, so user will determine whether his orders were really canceled.  
    I'm planning to introduce UI popup with the result of canceling.
5) That's it - UI with several tables, internal REST api fetching, weight balancing, then, as bonus, balance with websockets endpoints.