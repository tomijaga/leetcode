select distinct l1.account_id
from loginfo l1, loginfo l2
where l1.account_id = l2.account_id
    and l1.ip_address <> l2.ip_address
    and l2.login <= l1.logout
    and l1.login <= l2.logout
