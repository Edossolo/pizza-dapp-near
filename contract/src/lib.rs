use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, BorshStorageKey, CryptoHash, PanicOnDefault, Promise,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct PizzaPap {
    user_orders: UnorderedMap<AccountId, UnorderedMap<String, Order>>,
    pizza_pap_address: AccountId,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    Accounts,
    SubAccount { account_hash: CryptoHash },
}

#[near_bindgen]
impl PizzaPap {
    #[init]
    pub fn init(pizza_pap_address: AccountId) -> Self {
        // initialise contract and set pizza pap delivery address
        assert!(!env::state_exists(), "Already initialized");
        Self {
            user_orders: UnorderedMap::new(StorageKeys::Accounts),
            pizza_pap_address,
        }
    }

    #[payable]
    pub fn create_order(&mut self, payload: OrderPayload) {
        // Defind storage cost for contract
        let storage_cost: u128 = 1_000_000_000_000_000_000_000;

        // Get who is calling the method and amount attached
        let user: AccountId = env::predecessor_account_id();
        let mut total: Balance = env::attached_deposit();

        // Get the order from payload
        let order: Order = Order::from_payload(payload);

        // check that the amount sent is equal to the order total amount
        assert_eq!(total, order.total.parse().unwrap(), "Invalid Amount");

        // remove storage cost from total
        total -= storage_cost;

        // look for user's order mapping, if none create a new mapping for user
        let mut user_orders = match self.user_orders.get(&user) {
            Some(x) => x,
            None => UnorderedMap::new(StorageKeys::SubAccount {
                account_hash: env::sha256_array(user.as_bytes()),
            }),
        };

        // add order to user's mapping
        user_orders.insert(&order.id, &order);

        // get pizzapap deposit address
        let deposit_address: &&str = &self.pizza_pap_address.as_str();

        // send funds to the address
        Promise::new(deposit_address.parse().unwrap()).transfer(total);

        // update storage
        self.user_orders.insert(&user, &user_orders);
    }

    pub fn get_user_orders(self, user: &AccountId) -> Vec<Order> {
        // returns user orders
        let user_orders = match self.user_orders.get(&user) {
            Some(x) => x,
            None => UnorderedMap::new(StorageKeys::SubAccount {
                account_hash: env::sha256_array(user.as_bytes()),
            }),
        };

        user_orders.values_as_vector().to_vec()
    }

    pub fn confirm_order(&mut self, order_id: &String) {
        // get caller accountId
        let user: AccountId = env::predecessor_account_id();

        // check if user already has an order mapping, declare error if not
        let mut user_orders = match self.user_orders.get(&user) {
            Some(x) => x,
            None => env::panic_str("Invalid user records"),
        };

        // check for order with id, return error if order does not exist
        let mut order = match user_orders.get(order_id) {
            Some(x) => x,
            None => env::panic_str("Orders not found"),
        };

        // update order status
        order.update_order_status();

        // update storage
        user_orders.insert(&order_id, &order);

        // update storage
        self.user_orders.insert(&user, &user_orders);
    }
}

#[near_bindgen]
#[derive(Serialize, Deserialize, PanicOnDefault)]
pub struct OrderPayload {
    id: String,
    flavor: String,
    size: String,
    crust: String,
    toppings: String,
    name: String,
    location: String,
    phone_number: String,
    total: String,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Serialize, PanicOnDefault)]
pub struct Order {
    id: String,
    flavor: String,
    size: String,
    crust: String,
    toppings: String,
    name: String,
    location: String,
    phone_number: String,
    total: String,
    status: bool,
}

#[near_bindgen]
impl Order {
    pub fn from_payload(payload: OrderPayload) -> Self {
        Self {
            id: payload.id,
            flavor: payload.flavor,
            size: payload.size,
            crust: payload.crust,
            toppings: payload.toppings,
            name: payload.name,
            location: payload.location,
            phone_number: payload.phone_number,
            total: payload.total,
            status: false,
        }
    }

    pub fn update_order_status(&mut self) {
        self.status = true;
    }
}
