import React from "react";
import PropTypes from "prop-types";
import { Button } from "react-bootstrap";
import Loader from "../utils/Loader";
import { utils } from "near-api-js";

const PrevOrders = ({ orders, confirmDelivery, loading }) => {
  const startConfirmTxn = async (order) => {
    await confirmDelivery(order.id);
  };

  return (
    <>
      <div className="order" id="order-table">
        <div className="order-overlay prev-orders table-responsive">
          <div className="controls-top text-center" id="Previous-orders">
            <h2>Previous Orders</h2>
          </div>
          {!loading ? (
            <table id="order-table" className="table-edit table align-middle">
              <thead>
                <tr>
                  <th scope="col">#</th>
                  <th scope="col">Flavor</th>
                  <th scope="col">Size</th>
                  <th scope="col">Crust</th>
                  <th scope="col">Toppings</th>
                  <th scope="col">Total (ALGO)</th>
                  <th scope="col">Status</th>
                </tr>
              </thead>
              <tbody id="userorders">
                {orders ? (
                  orders.map((order, index) => (
                    <tr key={index} className="align-middle">
                      <td id="id">{order.id}</td>
                      <td id="pizzaname">{order.flavor}</td>
                      <td id="pizzasize">{order.size}</td>
                      <td id="pizzacrust">{order.crust}</td>
                      <td id="pizzatopping">{order.toppings}</td>
                      <td id="totals">
                        {Number(
                          utils.format.formatNearAmount(order.total)
                        ).toFixed(2)}
                      </td>
                      <td id="status">
                        {order.status ? (
                          "Delivery Confirmed"
                        ) : (
                          <Button
                            onClick={(e) => startConfirmTxn(order)}
                            variant="success"
                          >
                            Confirm Delivery
                          </Button>
                        )}
                      </td>
                    </tr>
                  ))
                ) : (
                  <></>
                )}
              </tbody>
            </table>
          ) : (
            <Loader />
          )}
        </div>
      </div>
    </>
  );
};

PrevOrders.propTypes = {
  orders: PropTypes.instanceOf(Array),
  confirmDelivery: PropTypes.func.isRequired,
  loading: PropTypes.bool.isRequired,
};

export default PrevOrders;
