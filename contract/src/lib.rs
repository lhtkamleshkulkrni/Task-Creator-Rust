#![allow(non_snake_case)]
#![allow(unused_variables)]

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{log, env, near_bindgen, AccountId, Promise};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TaskCreator {
    memo: LookupMap<AccountId, Vec<String>>,
}
impl Default for TaskCreator {
    fn default() -> Self {
        Self {
            memo: LookupMap::new(b"memo".to_vec()),
        }
    }
}
#[near_bindgen]
impl TaskCreator {

    // Change Method
    pub fn addTasks(&mut self, assigne: String, taskNumber: String, Description: String, Task_heading: String) {
  log!("Adding New Task");

        let account_id = env::signer_account_id();
        let contains_user = self.memo.contains_key(&account_id);
        if contains_user {
            let mut temp_list = match self.memo.get(&account_id) {
                Some(x) => x,
                None => vec![],
            };
            temp_list.push( taskNumber + "  ___  Assignee : " + &assigne  + "  ___  Task Heading : " + &Task_heading + "  ___  Description : " + &Description);
            self.memo.insert(&account_id, &temp_list);
        } else {
            let fresh_vec = vec![taskNumber + "  ___  Assignee : " + &assigne  + "  ___  Task Heading : " + &Task_heading + "  ___  Description : " + &Description];
            self.memo.insert(&account_id, &fresh_vec);
        }
    }

    // send Task
    pub fn Task_Allocated(&mut self,
        account_id: AccountId,
        taskNumber: f64,
        Taskheading: String,
        Description: String,) 
        {
        Promise::new(account_id).transfer(taskNumber as u128);
        }

    // View Methods
    pub fn getTasks(self, user: AccountId) -> Vec<String> {
        match self.memo.get(&user) {
            Some(x) => x,
            None => vec![],
        }
    }
}
