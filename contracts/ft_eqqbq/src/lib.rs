use near_sdk::{env, near, AccountId, PromiseOrValue, Promise, PanicOnDefault, NearToken, require};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::{U128, U64};

pub const STORAGE_COST: NearToken = NearToken::from_millinear(1);
const TIMELOCK_SECONDS: u64 = 60 * 60 * 24; // 24 horas

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    /// Almacena los depósitos vinculados a un hash(secreto)
    pub deposits: UnorderedMap<String, DepositInfo>,
    pub deposit_number: U128,
}

#[near(serializers = [json, borsh])]
#[derive(Clone)]
//#[derive(Debug)]
pub struct DepositInfo {
    pub sender: AccountId,
    pub amount: U128,
    pub timestamp: u64,
    pub claimed: bool,
}

#[near]
impl Contract {
    #[init]
    pub fn init(
        deposit_number: U128,
    ) -> Self {
        Self {
            deposit_number,
            deposits: UnorderedMap::new(0),
        }
    }
    /// Función de callback cuando se reciben tokens (NEP-141)
    #[payable]
    pub fn ft_on_transfer(
        &mut self,
        //falta el adress del sender
        //me tengo que guardar que token es
        sender_id: AccountId,
        amount: U128,
        msg: String, // msg debería ser el hash(secreto)
    ) -> PromiseOrValue<U128> {
        let hash = msg;
        assert!(
            self.deposits.get(&hash).is_none(),
            "Ya existe un depósito con ese hash"
        );

        //Falta mirar como guardar este ft en el contrato
        //let ft = predecessor_account_id();
        
        //require!(ft == self.ft, "The token is not supported");

        let deposit = DepositInfo {
            sender: sender_id,
            amount,
            timestamp: env::block_timestamp(),
            claimed: false,
        };

        self.deposits.insert(&hash, &deposit);
        PromiseOrValue::Value(U128(0))
    }

    /// Reclamar fondos proporcionando el secreto que genera el hash
    pub fn claim_tokens(&mut self, secret: String) {
        
        let result = secret;
        let hash = result;

        let mut deposit = self
            .deposits
            .get(&hash)
            .expect("No hay fondos asociados a ese hash");

        assert!(!deposit.claimed, "Ya fueron reclamados");

        deposit.claimed = true;
        self.deposits.insert(&hash, &deposit);

        Promise::new(env::predecessor_account_id()).transfer(NearToken::from_yoctonear(deposit.amount.0));
    }

    /// Recuperar fondos después del timelock
    pub fn retrieve_tokens(&mut self, hash: String) {
        let deposit = self
            .deposits
            .get(&hash)
            .expect("No hay depósito para ese hash");

        assert!(
            env::block_timestamp() > deposit.timestamp + TIMELOCK_SECONDS * 1_000_000_000,
            "El tiempo de espera aún no ha pasado"
        );
        assert!(!deposit.claimed, "Ya fueron reclamados");

        self.deposits.remove(&hash);
        Promise::new(deposit.sender).transfer(NearToken::from_yoctonear(deposit.amount.0));
    }

    #[payable]
    pub fn recive_near(&mut self,
        //falta el adress del sender ## ya se está guardando con el sender_id, no?
        //me tengo que guardar que token es ## en near no se puede saber qué token se está recibiendo,
        // al enviar near, por ejemplo, se llama a ft_transfer_call desde el token nep-141
        msg: String,) -> PromiseOrValue<U128> {
        
        //probablemente habría que poner en algún momento la función de yoctonear por temas de seguridad
        let hash = msg; //sigue faltando hacer el hash de la string
        let sender_id: AccountId = env::predecessor_account_id();
        let amount_near = env::attached_deposit();

        //el require se ejecuta todas las veces, no se si es necesario que sólo se haga la primera vez
        require!(
            amount_near > STORAGE_COST,
            format!(
                "Attach at least {} yoctoNEAR to cover for the storage cost",
                STORAGE_COST
            )
        );
        //tendría que restar el storage_cost a lo que se envía ##done en la siguiente línea

        let amount: U128 = U128(amount_near.as_yoctonear() - STORAGE_COST.as_yoctonear());

        let deposit = DepositInfo {
            sender: sender_id,
            amount,
            timestamp: env::block_timestamp(),
            claimed: false,
        };

        self.deposits.insert(&hash, &deposit);

        PromiseOrValue::Value(U128(0))
    }

    //TODO: hacer una función que recibe near y que lo guarde en depositinfo (amount)
    //hacer hash
    //testear todo bien
    //revisar las funciones que ha hecho el chatgpt

    pub fn claim_near(){

    }

    pub fn get_deposit_info(&self, string: String) -> Option<DepositInfo>{
        self.deposits.get(&string)
    }

    pub fn get_deposit_number(&self) -> U128{
        self.deposit_number
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::testing_env;
    use near_sdk::test_utils::VMContextBuilder;

    use super::*;

    #[test]
    fn init_contract() {
        let contract = Contract::init(
            U128(3),
        );

        let deposit_number = contract.get_deposit_number();

        assert_eq!(deposit_number, U128(3));
    }

      #[test]
    fn test_on_transfer() {
        let mut contract = Contract::init(
            U128(3),
        );

        let alice: AccountId = "alice.near".parse().unwrap();

        contract.ft_on_transfer(alice.clone(), U128(23), "asdasd".to_string());

        let value = contract.deposits.get(&"asdasd".to_string()).unwrap();

        assert_eq!(value.sender, alice);
        assert_eq!(value.amount, U128(23));
        assert_eq!(value.claimed, false);
        assert_eq!(value.timestamp, env::block_timestamp());
    }

    //este test no va aquí, hay que hacer un test de integración
     #[test]
    fn recive_near() {
        let mut contract = Contract::init(
            U128(3),
        );

        let alice: AccountId = "alice.near".parse().unwrap();

        let mut builder = VMContextBuilder::new();
        builder
            .attached_deposit(NearToken::from_near(1))
            .predecessor_account_id(alice.clone());

        testing_env!(builder.build());

        contract.recive_near("asdasd".to_string());
  
        let value = contract.deposits.get(&"asdasd".to_string()).unwrap();

        //println!("{:?}", value);
        let attached_deposit = NearToken::from_near(1).checked_sub(STORAGE_COST).unwrap().as_yoctonear();

        assert_eq!(value.sender, alice);
        assert_eq!(value.amount, attached_deposit.into());
        assert_eq!(value.claimed, false);
        assert_eq!(value.timestamp, env::block_timestamp());
    }

    #[test]
    fn claim_tokens(){
        let mut contract = Contract::init(
            U128(3),
        );

        let alice: AccountId = "alice.near".parse().unwrap();
        let hash = "hash123".to_string();

        let mut builder = VMContextBuilder::new();
        builder
            .attached_deposit(NearToken::from_near(1))
            .predecessor_account_id(alice.clone());

        testing_env!(builder.build());

        let deposit_info = DepositInfo{
            sender: alice.clone(),
            amount: U128::from(1_000_000_000_000_000_000_000_000),
            claimed: false,
            timestamp: env::block_timestamp(),
        };

        contract.deposits.insert(&hash, &deposit_info);
        contract.claim_tokens(hash.clone());

        let updated_deposit: DepositInfo = contract.deposits.get(&hash).unwrap();

        assert!(updated_deposit.claimed, "Deposit has not been claimed yet");
    }

    #[test]
    fn retrieve_tokens() {
         let mut contract = Contract::init(
            U128(3),
        );

        let alice: AccountId = "alice.near".parse().unwrap();
        let hash = "hash123".to_string();

        let deposit_info = DepositInfo{
            sender: alice.clone(),
            amount: U128::from(1_000_000_000_000_000_000_000_000),
            claimed: false,
            timestamp: env::block_timestamp() + 25 * 3600 * 1_000_000_000,
        };

        let mut builder = VMContextBuilder::new();
        builder
            .attached_deposit(NearToken::from_near(1))
            .predecessor_account_id(alice.clone());

        testing_env!(builder.build());

        contract.deposits.insert(&hash, &deposit_info);

        contract.retrieve_tokens(hash.clone());

        assert!(contract.deposits.get(&hash).is_none(),"Deposit was not deleted after retrieving the tokens");
    }

    //TODO: hacer el test del flow del contrato
}