import React, { useState } from 'react';
import './App.css';
import { CssBaseline, ThemeProvider, Button } from '@material-ui/core';
import { createMuiTheme, responsiveFontSizes } from '@material-ui/core/styles';
import { red, grey } from '@material-ui/core/colors';
import LoginPgae from './login'
import { usernameFromToken } from './utils'
import TopBar from './topBar'

const theme = responsiveFontSizes(createMuiTheme({
	palette: {
		primary: red,
		secondary: grey,
	},
	status: {
		danger: 'orange',
	},
}));

const Page = ({ sessionUser, setSessionUser }) => {
	return (
		<div>
			<TopBar sessionUser={sessionUser} setSessionUser={setSessionUser} />
			{sessionUser}
		</div>
	)
}

const App = () => {
	let [sessionUser, setSessionUser] = useState(usernameFromToken(localStorage.getItem("refresh_token")));

	return (
		<div className="App">
			<CssBaseline />
			<ThemeProvider theme={theme}>
				{!!sessionUser ?
					<Page sessionUser={sessionUser} setSessionUser={setSessionUser} />
					:
					<LoginPgae setSessionUser={setSessionUser} />
				}
			</ThemeProvider>
		</div>
	)
}

export default App;
