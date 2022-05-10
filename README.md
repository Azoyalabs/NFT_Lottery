# NFT Lottery 

Give the ability to create a lottery with NFTs as prizes.  
Anyone can create a lottery and set an entry fee (or no entry fee), and prizes. Anyone can register, winners are drawn from pool of registered users. 
Lottery organizer gets profits from organizing the lottery, while potentially paying a small cut to the maintainer.  

Uses the Terrand on-chain beacon for randomness. 

Contract is still in development: we are working on adding safety features to safeguard players' interest and safeguard funds.  

## Steps 

1- Create a lottery  
2- Set participation price  
3- Set prizes 
4- Lock prizes and ensure validity (NFTs' ownership is transferred to the lottery contract)  
5- Allow registrations 
6- Lock registrations
7- Draw Lottery winners 
8- Enable Players to withdraw won prizes


## Coming Soon  
- Adding Marketing information  
- Allow payment in CW20 
- Safeguard funds (allow users to un-register)


## Exploits Prevention Remarks  
Organizer can lock up funds if no possibility of refund. Either allow refunding all the way until prize winners are drawn, or set up a max lifetime for the lottery and draw before locking up the lottery state and allowing all users withdrawals.

Need to be careful with safeguarding funds. Lottery admin can modify pricing, so need to track payments done by users to prevent funds stealing with a naive refund process. 
Naïve process: user gets funds back equals to pricing of lottery.   
Problem is if pricing has changed: user can get back more than deposited, possible to drain fund from contracts if not addressed initially. 



