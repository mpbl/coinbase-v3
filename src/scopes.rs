//! Valid scopes, as listed on [Coinbase documentation](https://docs.cloud.coinbase.com/sign-in-with-coinbase/docs/permissions-scopes)

/// List of valid scopes.
pub const VALID_SCOPES: [&str; 25] = [
    "wallet:accounts:read",          //List user's accounts and their balances
    "wallet:accounts:update",        //Update account (e.g. change name)
    "wallet:accounts:create",        //Create a new account (e.g. BTC wallet)
    "wallet:accounts:delete",        //Delete existing account
    "wallet:addresses:read",         //List account's bitcoin or ethereum addresses
    "wallet:addresses:create",       //Create new bitcoin or ethereum addresses for wallets
    "wallet:buys:read",              //List account's buys
    "wallet:buys:create",            //Buy bitcoin or ethereum
    "wallet:deposits:read",          //List account's deposits
    "wallet:deposits:create",        //Create a new deposit
    "wallet:notifications:read",     //List user's notifications
    "wallet:payment-methods:read",   //List user's payment methods (e.g. bank accounts)
    "wallet:payment-methods:delete", //Remove existing payment methods
    "wallet:payment-methods:limits", //Get detailed limits for payment methods (useful for performing buys and sells). This permission is to be used together with wallet:payment-methods:read
    "wallet:sells:read",             //List account's sells
    "wallet:sells:create",           //Sell bitcoin or ethereum
    "wallet:transactions:read",      //List account's transactions
    "wallet:transactions:send",      //Send bitcoin or ethereum
    "wallet:transactions:request",   //Request bitcoin or ethereum from a Coinbase user
    "wallet:transactions:transfer", //Transfer funds between user's two bitcoin or ethereum accounts
    "wallet:user:read", //List detailed user information (public information is available without this permission)
    "wallet:user:update", //Update current user
    "wallet:user:email", //Read current user's email address
    "wallet:withdrawals:read", //List account's withdrawals
    "wallet:withdrawals:create", //Create a new withdrawal
];
