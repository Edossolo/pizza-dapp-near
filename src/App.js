import React, { useState, useCallback, useEffect } from "react";
import Cover from "./components/Cover";
import "./App.css";
import "./css/style.css";
import "./css/wickedcss.min.css";
import { Container } from "react-bootstrap";
import { login, logout as destroy, accountBalance } from "./utils/near";
import Header from "./components/pizzapap/Header";
import PizzaSection from "./components/pizzapap/PizzaSection";
import Footer from "./components/pizzapap/Footer";
import { Notification } from "./components/utils/Notifications";
import coverImg from "./assets/img/cover.jpg";

const App = function AppWrapper() {
  const account = window.walletConnection.account();

  const [balance, setBalance] = useState("0");

  const getBalance = useCallback(async () => {
    if (account.accountId) {
      setBalance(await accountBalance());
    }
  }, [account]);

  useEffect(() => {
    getBalance();
  }, [getBalance]);

  return (
    <>
      <Notification />
      {account.accountId ? (
        <Container fluid className="main-header">
          <Header
            address={account.accountId}
            amount={balance}
            destroy={destroy}
          />
          <main>
            <PizzaSection
              address={account.accountId}
              fetchBalance={getBalance}
            />
          </main>
          <Footer />
        </Container>
      ) : (
        <Cover name={"PIZZAPAP"} coverImg={coverImg} connect={login} />
      )}
    </>
  );
};

export default App;
