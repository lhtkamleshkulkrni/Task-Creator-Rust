# Near Task Creator

- Near Task Creator is a NEAR blockchain based dApp which which includes creation of Task for users
- This Near Task Creator dApp it consists of a Smart Contract and it is witten in Rust.
- Ultimately, the purpose of this project was to build a simple contract to explore how contract calls interact when building on the NEAR ecosystem.

### Functionality

#### This Near Task Creator includes following functionality :

- Manager login with Near Wallet
- Manager create task for users
- Task Dashboard
- User get the Task Allocated message in their Near wallet
- User get account-ID, Task Heading, Task Description in message
- Logout


### Smart Contracts Used In Project

#### Change Method

```bash
addTasks
```

#### Send Method

```bash
Task_Allocated
```

#### View Method

```bash
getTasks
```

```bash
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

```

### Quick Start

#### To run this project locally:

- Prerequisites: Make sure you've installed [Node.js] ≥ 12
- Clone this repository
- Go to the project folder
- Install dependencies: `yarn install`
- Run the local development server: `yarn dev` (see `package.json` for a
  full list of `scripts` you can run with `yarn`)

Now you'll have a local development environment backed by the NEAR TestNet!

Go ahead and play with the app and the code. As you make code changes, the app will automatically reload.

### Exploring The Code

- The "backend" code lives in the `/contract` folder.
- The frontend code lives in the `/frontend` folder. `/frontend/index.html` is a great
  place to start exploring. Note that it loads in `/frontend/assets/js/index.js`, where you
  can connects frontend to the NEAR blockchain.
- Tests: there are different kinds of tests for the frontend and the smart
  contract. See `contract/README` for info about how it's tested. The frontend
  code gets tested with [jest]. You can run both of these at once with `yarn run test`.

### Installation -

Every smart contract in NEAR has its [own associated account][near accounts]. When you run `yarn dev`, your smart contract gets deployed to the live NEAR TestNet with a throwaway account. When you're ready to make it permanent, here's how.

### Step 1: Install near-cli (optional)

[near-cli] is a command line interface (CLI) for interacting with the NEAR blockchain. It was installed to the local `node_modules` folder when you ran `yarn install`, but for best ergonomics you may want to install it globally:

    yarn install --global near-cli

Or, if you'd rather use the locally-installed version, you can prefix all `near` commands with `npx`

Ensure that it's installed with `near --version` (or `npx near --version`)

### Step 2: Create an account for the contract

Each account on NEAR can have at most one contract deployed to it. If you've already created an account such as `your-name.testnet`, you can deploy your contract to `near-blank-project.your-name.testnet`. Assuming you've already created an account on [NEAR Wallet], here's how to create `near-blank-project.your-name.testnet`:

- Authorize NEAR CLI, following the commands it gives you:
```bash
    near login
```
- Create a subaccount (replace `YOUR-NAME` below with your actual account name):
```bash
     near create-account near-blank-project.YOUR-NAME.testnet --masterAccount YOUR-NAME.testnet
```
