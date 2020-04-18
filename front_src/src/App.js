import React from 'react';
import './App.css';
import { CssBaseline, ThemeProvider, Button } from '@material-ui/core';
import { createMuiTheme, responsiveFontSizes } from '@material-ui/core/styles';
import { red, grey } from '@material-ui/core/colors';
import LoginPgae from './login'

const theme = responsiveFontSizes(createMuiTheme({
	palette: {
	  primary: red,
	  secondary: grey,
	},
	status: {
	  danger: 'orange',
	},
}));

const App = () => (
	<div className="App">
		<CssBaseline />
		<ThemeProvider theme={theme}>
	  	{/* <header className="App-header">
		<p>
			Edit <code>src/App.js</code> and save to reload.
		</p>
		<Button variant="contained">Default</Button>
		<Button variant="contained" color="primary">Primary</Button>
	  </header> */}
	  <LoginPgae />
	</ThemeProvider>
	</div>
)

export default App;
