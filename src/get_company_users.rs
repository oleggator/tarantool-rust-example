use tarantool::index::IteratorType;
use tarantool::space::Space;
use tarantool::tuple::{FunctionArgs, FunctionCtx, Tuple};

use crate::models::User;

#[no_mangle]
pub extern "C" fn get_company_users(ctx: FunctionCtx, args: FunctionArgs) -> i32 {
    // decode request body to struct
    let args: Tuple = args.into();
    let (company_id,): (i32,) = match args.into_struct() {
        Ok(args) => args,
        Err(_) => return -1,
    };

    // get space and index by name
    let space = match Space::find("users") {
        Some(space) => space,
        None => return -1,
    };
    let company_index = match space.index("company") {
        Some(index) => index,
        None => return -1,
    };

    // get iterator for company_id
    let iter = match company_index.select(IteratorType::GE, &(company_id,)) {
        Ok(iter) => iter,
        Err(_) => return -1,
    };

    let mut company_users = vec![];
    for tuple in iter {
        // decode tuple to struct
        let user: User = match tuple.into_struct() {
            Ok(user) => user,
            Err(_) => return -1,
        };

        // break when company users ended
        if user.company_id != company_id {
            break;
        }
        company_users.push(user);
    }

    match ctx.return_mp(&(company_id, company_users)) {
        Ok(status) => status,
        Err(_) => return -1,
    }
}
