import React, { useState } from 'react';
import { Container, TextField, Button, Grid } from '@material-ui/core';
import { fillEntry, enterSubmit } from './utils'

const login = (username, password) => _ => {
	console.log(`username: ${username} password: ${password}`);
}

const maxHeight = {
	height: "100%"
}

const margin = {
	margin: "1em"
}

const LoginPage = () => {
	const [username, setUsername] = useState('');
	const [password, setPassword] = useState('');
	return (
		<Container maxWidth="xs" style={maxHeight}>
			<Grid container spacing={3} style={maxHeight} direction="column">
				<Grid item xs={4} />
				<TextField style={margin} required autoFocus label="Username" variant="outlined" value={username} onChange={fillEntry(setUsername)} />
				<TextField style={margin} required label="Passsword" variant="outlined" value={password} onChange={fillEntry(setPassword)} type="password" onKeyPress={enterSubmit(login(username, password))} />
				<Button style={margin} variant="contained" color="primary" onClick={login(username, password)} >Login</Button>
				<Button style={margin} variant="contained" color="secondary" >Register</Button>
				<Grid item xs={4}/>
			</Grid>
		</Container>
	)
}

export default LoginPage;