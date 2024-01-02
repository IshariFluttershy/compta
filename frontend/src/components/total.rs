use common::PaymentTotal;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TotalProps {
    pub total: PaymentTotal,
}

#[function_component(Total)]
pub fn total(TotalProps { total }: &TotalProps) -> Html {
    html!(
        <div >
        <table >
            <caption>{"Totaux"}</caption>
            <tr>
                <td></td>
                <th scope="col">{"Carte bleue"}</th>
                <th scope="col">{"Especes"}</th>
                <th scope="col">{"Total"}</th>
            </tr>
            <tr>
                <th scope="row">{"Nourriture"}</th>
                <td>{total.cb_food}</td>
                <td>{total.cash_food}</td>
                <td>{total.food}</td>
            </tr>
            <tr>
                <th scope="row">{"Charges"}</th>
                <td>{total.cb_charges}</td>
                <td>{total.cash_charges}</td>
                <td>{total.charges}</td>
            </tr>
            <tr>
                <th scope="row">{"Autres"}</th>
                <td>{total.cb_miscellaneous}</td>
                <td>{total.cash_miscellaneous}</td>
                <td>{total.miscellaneous}</td>
            </tr>
            <tr>
            <th scope="row">{"Total"}</th>
                <td>{total.cb}</td>
                <td>{total.cash}</td>
                <td>{total.total}</td>
            </tr>
        </table>
        </div>
    )
}
