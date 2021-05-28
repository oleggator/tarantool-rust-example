box.cfg {
    listen = 3301,
    wal_mode = 'none'
}

local users_space = box.schema.create_space('users', { if_not_exists = true })
users_space:format {
    { name = 'user_id', type = 'unsigned' },
    { name = 'company_id',type = 'unsigned' },
    { name = 'username', type = 'string' },
    { name = 'country', type = 'string' },
}

users_space:create_index('pk', {
    if_not_exists = true,
    parts = {
        { 'user_id', 'unsigned'},
    },
})

users_space:create_index('company', {
    if_not_exists = true,
    unique = false,
    parts = {
        { 'company_id', 'unsigned'},
    },
})

users_space:create_index('country', {
    if_not_exists = true,
    unique = false,
    parts = {
        { 'country', 'string'},
    },
})
local data_raw = require('io').open('data.json', 'rb'):read('*all')
local data = require('json').decode(data_raw)

for _, user in ipairs(data) do
    users_space:insert {
        user['user_id'],
        user['company_id'],
        user['username'],
        user['country'],
    }
end

box.schema.func.create('libprocedures.get_company_users', { language = 'C', if_not_exists = true })
function get_company_users(company_id)
    return box.schema.func.call('libprocedures.get_company_users', company_id)
end
