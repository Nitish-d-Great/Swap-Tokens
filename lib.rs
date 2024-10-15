#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, token, IntoVal};

#[contract]
pub struct AtomicSwapContract;

#[contractimpl]
impl AtomicSwapContract {
    pub fn swap (
        env: Env, // it is helping us to interact with the blockchain which is in the end an environment only
        a:Address, b: Address,
        token_a: Address, token_b: Address,
        amount_a: i128, min_b_for_a: i128,
        amount_b: i128, min_a_for_b: i128, 
    )
    {
        if amount_b<min_b_for_a // basically agar minimum x amount of tokens swap kr rhe ho to hi mein swap krne me interested hu warna mujhse swap nahi krna.. just like.. to swap, you need to swap atleast x tokens to perform swapping
        {
            panic!("Not enough token b for exchanging with token a");
        }
        if amount_a<min_a_for_b
        {
            panic!("Not enough token a for exchanging with token b");
        }

        a.require_auth_for_args((token_a.clone(),token_b.clone(),amount_a,min_b_for_a).into_val(&env));
        b.require_auth_for_args((token_b.clone(),token_a.clone(),amount_b,min_a_for_b).into_val(&env));
        // basically.. hm direct swap ni kr re.. clone banake fir swap krenge.. kitna swap krna hai wo amount_a ya amount_b mein aega.. aur fir check krna hai ki agar a ko swap kr re hain to kya wo minimum a needed for swapping with b ki condition ko satisfy kr rha hai ya ni.. fir usko val me convert krenge taki wo soroban env me chal sake
    
        move_token(&env, &token_a, &a, &b, amount_a, min_a_for_b);    // from a to b, we are transfering amount_a number of tokens which must satisfy the condition ki minimum kitna token a ka dalna padega to be eligible for transfer to b..
        move_token(&env, &token_b, &b, &a, amount_b, min_b_for_a);    // same as above (bas a ka b and b ka a)
        // move function bana hai neeche  
    }
}

    fn move_token       // move is a reserve keyword
    (
        env: &Env,
        token: &Address, from: &Address, to: &Address,
        max_spend_amount: i128, transfer_amount: i128
    )
    // yaha pe And (&) isliye laga rhe hain since we dont want to transfer ownership of tokens.. just swap krna hai and khatam.. ownership transfer nahi krni hai

    {
        let token = token::Client::new(env,token);  // instance create kia hai (object)
        let contract_address = env.current_contract_address();  // to get contract address of the current contract, hence we are using env
        token.transfer(from, &contract_address, &max_spend_amount); // mein max itna amount spend kr sakta hu during swapping
        token.transfer(&contract_address, to, &transfer_amount);    // itna amount transfer kia hai jo ki swap hona hai
        token.transfer(&contract_address, &from, &(max_spend_amount - transfer_amount));   // kitna mene already bhej dia tha and kitna mein swap krna chahta hu.. ab isme jo extra amount mene bhej dia tha pehle wo mein wapas le rha hu jo ki hoga difference bw jo mene pehle extra dedia tha minus jo me actual me swap krna chahta hu
    }


mod test;