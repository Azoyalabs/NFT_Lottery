

TokenInfo is only a response, no need to store it 

need admin(s) --- good?  

metadata for platforms:  
from opensea
https://docs.opensea.io/docs/metadata-standards
{
  "description": "Friendly OpenSea Creature that enjoys long swims in the ocean.", 
  "external_url": "https://openseacreatures.io/3", 
  "image": "https://storage.googleapis.com/opensea-prod.appspot.com/puffs/3.png", 
  "name": "Dave Starbelly",
  "attributes": [ ... ], 
}


terrarity description:  
Terrarity Heroes is a collection of characters for Terrarity, the blockchain game on Terra. Created by AzoyaLabs, having your own heroes allows you to play a variety of games and more!

character: 
add level


Terrarity Heroes 
so what for terrarity?  
{
    "description": "Terrarity is a  
    "external_url": "https://terrarity.azoyalabs.com/characters" + id
    "image": url image class,   //  We recommend using a 350 x 350 image.
    "name": character name  ? ,

    "attributes": [
        {
            "trait_type": "Class",
            "value": "Warrior"
        }, {
            "trait_type": "Level", 
            "value": 1
        }, {
        "display_type": "date", 
        "trait_type": "birthday", 
        "value": 1546360800
        }
      
    }, 
    ]
}


implémenter test queries   
- revoir structs et response renvoyées   
- link up token info / character info et class info sur réponse de query          
- revoir tests: éviter répétition créer classe de base                          --- DONE
- add pricing update                                                            --- DONE
- add pricing query                                                             --- DONE
- add update admin                                                              --- DONE 
- add update class  
- add URI file to class for image query 


revoir les imports de type: confusion entre state, cw721 et cw_utils            --- DONE?


Still to implement:  
PAYMENT          --- DONE
WITHDRAW         --- DONE
gotta add tests  --- DONE for execute?  

execute:  
    Mint         --- DONE
    TransferNft  --- DONE
    SendNft      --- DONE
    Approve      --- DONE
    Revoke       --- DONE
    ApproveAll   --- DONE
    RevokeAll    --- DONE
    AddClass     --- DONE


query: 
    OwnerOf         --- DONE
    Approved        --- DONE
    ApprovedForAll  --- DONE?
    NumTokens       --- DONE
    ContractInfo    --- DONE
    NftInfo         --- DONE
    AllNftInfo      --- DONE??? Vérifier les infos renvoyés, je crois que j'ai pas le bon struct en sortie
    Tokens          --- DONE
    AllTokens       --- DONE


tests: 
    Mint                                    --- DONE
    Transfer                                --- DONE
    Allow + transfer                        --- DONE
    Allow + revoke + transfer               --- DONE
    ApproveAll + transfer                   --- DONE
    ApproveAll + RevokeAll + transfer       --- DONE

