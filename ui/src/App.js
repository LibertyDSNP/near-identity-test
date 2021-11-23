import './App.css';
import Button from 'react-bootstrap/Button';
import PropTypes from 'prop-types';

function App({ contract, currentUser, nearConfig, wallet }) {
  
  function createAccount() {
    console.log("hello world");
  }

  return (
    <div className="App">
      <header className="App-header">
        <p>
          NEAR Identity Playground
        </p>
        <Button variant="light" onClick={createAccount}>Create Account</Button>
      </header>
      
    </div>
  );
}



App.propTypes = {
  contract: PropTypes.shape({
    addMessage: PropTypes.func.isRequired,
    getMessages: PropTypes.func.isRequired
  }).isRequired,
  currentUser: PropTypes.shape({
    accountId: PropTypes.string.isRequired,
    balance: PropTypes.string.isRequired
  }),
  nearConfig: PropTypes.shape({
    contractName: PropTypes.string.isRequired
  }).isRequired,
  wallet: PropTypes.shape({
    requestSignIn: PropTypes.func.isRequired,
    signOut: PropTypes.func.isRequired
  }).isRequired
};


export default App;
