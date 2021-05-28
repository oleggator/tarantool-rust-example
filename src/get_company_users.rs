use tarantool::index::IteratorType;
use tarantool::space::Space;
use tarantool::tuple::{FunctionArgs, FunctionCtx, Tuple};

use crate::models::User;

#[no_mangle]
pub extern "C" fn get_company_users(ctx: FunctionCtx, args: FunctionArgs) -> i32 {
    // decode request body to struct
    let args: Tuple = args.into();
    let (company_id, ): (i32, ) = args.into_struct().unwrap();

    // get space and index by name
    let space = Space::find("users").unwrap();
    let company_index = space.index("company").unwrap();

    // get iterator for company_id
    let iter = company_index.select(IteratorType::GE, &(company_id, )).unwrap();

    let mut company_users = vec![];
    for tuple in iter {
        // decode tuple to struct
        let user: User = tuple.into_struct().unwrap();

        // break when company users ended
        if user.company_id != company_id { break; }
        company_users.push(user);
    }

    // encode respone and return to client
    ctx.return_mp(&(company_id, company_users)).unwrap()
}
