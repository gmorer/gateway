import React, { useState } from 'react';
import { Container, TextField, Button, Grid, Dialog, DialogTitle, DialogContent, DialogActions } from '@material-ui/core';
import { fillEntry, enterSubmit, apiCall } from './utils'

const login = (username, password, setSessionUser) => async _ => {
	try {
		let { acces_token, refresh_token } = await apiCall({ uri: "auth/auth", body: { username, password } });
		localStorage.setItem("refresh_token", refresh_token);
		sessionStorage.acces_token = acces_token;
		setSessionUser(username);
	} catch (e) {
		console.error(e)
	}
}

const toggleModal = (setModal) => _ => setModal(state => !state);

const register = (username, password, mentoring, setModal) => async _ => {
	try {
		let res = await apiCall({ uri: "auth/join", body: { username, password, mentoring } })
		console.log(res);
	} catch (e) {
		console.log(e)
	}
	toggleModal(setModal)();
}


const LoginPage = ({ setSessionUser }) => {
	const [username, setUsername] = useState('');
	const [password, setPassword] = useState('');
	const [mentoring, setMentoring] = useState('');
	const [modal, setModal] = useState(false);

	return (
		<div>
			<Dialog open={modal} fullWidth={true} onClose={toggleModal(setModal)} >
				<DialogTitle>Subscribe</DialogTitle>
				<DialogContent>
					<TextField required autoFocus label="Mentoring code" margin="dense" fullWidth value={mentoring} onChange={fillEntry(setMentoring)} />
				</DialogContent>
				<DialogActions>
					<Button onClick={toggleModal(setModal)} color="primary">
						Cancel
					</Button>
					<Button onClick={register(username, password, mentoring, setModal)} color="primary">
						Register
					</Button>
				</DialogActions>
			</Dialog>
			<Container maxWidth="xs">
				<Grid container justify="center" direction="column" spacing={3} style={{ marginTop: "50%" }}>
					<Grid item>
						<TextField required autoFocus label="Username" variant="outlined" value={username} onChange={fillEntry(setUsername)} />
					</Grid>
					<Grid item>
						<TextField required label="Passsword" variant="outlined" value={password} onChange={fillEntry(setPassword)} type="password" onKeyPress={enterSubmit(login(username, password))} />
					</Grid>
					<Grid item>
						<Button variant="contained" color="primary" onClick={login(username, password, setSessionUser)} >Login</Button>
					</Grid>
					<Grid item>
						<Button variant="contained" color="secondary" onClick={toggleModal(setModal)} >Register</Button>
					</Grid>
				</Grid>
			</Container>
		</div>
	)
}

export default LoginPage;