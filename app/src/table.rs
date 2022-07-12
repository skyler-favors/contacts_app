use yew::prelude::*;
//use sql_project::functions::get_users::get_users;

#[function_component(TableComp)]
pub fn table_comp() -> Html {
    html! {
        <div>
            <table>
                <tr>
                    <th>{"id"}</th>
                    <th>{"first"}</th>
                    <th>{"last"}</th>
                </tr>
            </table>
        </div>
    }
}

//                <tr>{
//                    get_users().iter().map(|user| {
//                        html! {
//                            <>
//                                <td>{user.id}</td>
//                                <td>{user.first.clone()}</td>
//                                <td>{user.last.clone()}</td>
//                            </>
//                        }
//                    }).collect::<Html>()
//                }</tr>
//
