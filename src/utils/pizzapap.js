import { v4 as uuid4 } from "uuid";
import { parseNearAmount } from "near-api-js/lib/utils/format";

const GAS = 100000000000000;

export function createOrder(order) {
  order.id = uuid4();
  order.total = parseNearAmount(order.total + "");
  return window.contract.create_order({ payload: order }, GAS, order.total);
}

export function getUserOrders(user) {
  return window.contract.get_user_orders({ user });
}

export async function confirmOrder(id) {
  console.log(id);
  await window.contract.confirm_order({ order_id: id });
}
