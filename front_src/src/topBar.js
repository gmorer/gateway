import React, { useState } from 'react';
import { AppBar, Toolbar, IconButton, Typography, Button, Menu, MenuItem, Badge, Breadcrumbs, Link } from '@material-ui/core';
import { makeStyles } from '@material-ui/core/styles';
import {
	Menu as MenuIcon,
	Notifications as NotificationsIcon,
	Home as HomeIcon,
	Whatshot as WhatshotIcon,
	Grain as GrainIcon,
	AccountCircle
} from '@material-ui/icons'

const useStyles = makeStyles((theme) => ({
	grow: {
		flexGrow: 1,
	},
	menuButton: {
		marginRight: theme.spacing(2),
	},
	link: {
		display: 'flex',
	},
	desktopIcon: {
		marginRight: theme.spacing(0.5),
		width: 20,
		height: 20,
		display: 'none',
		[theme.breakpoints.up('md')]: {
			display: 'flex',
		},
	},
	icon: {
		marginRight: theme.spacing(0.5),
		width: 20,
		height: 20,
	},
	breadcrumbs: {
		fontSize: "2em"
	},
	sectionDesktop: {
		display: 'none',
		[theme.breakpoints.up('md')]: {
			display: 'flex',
		},
	},
	sectionMobile: {
		display: 'flex',
		[theme.breakpoints.up('md')]: {
			display: 'none',
		},
	},
}));

const action = () => ({})

const logout = (setSessionUser) => _ => {
	localStorage.removeItem("refresh_token");
	sessionStorage.acces_token = undefined;
	setSessionUser(null);
}

/*
	Actually this part is not mobiel ready
*/
const Navigation = ({ classes }) => {
	return (
		<Breadcrumbs aria-label="breadcrumb" className={classes.breadcrumbs}>
			<Typography color="textPrimary" className={classes.link}>
				<HomeIcon className={classes.desktopIcon} />
				Material-UI
			</Typography>
			<Typography color="textPrimary" className={classes.link}>
				<WhatshotIcon className={classes.desktopIcon} />
			Core
			</Typography>
			<Typography color="textPrimary" className={classes.link}>
				<GrainIcon className={classes.desktopIcon} />
			Breadcrumb
		</Typography>
		</Breadcrumbs>
	)
}

const RenderMenu = ({ anchorEl, setAnchorEl, setSessionUser }) => (
	<Menu
		anchorEl={anchorEl}
		anchorOrigin={{ vertical: 'top', horizontal: 'right' }}
		keepMounted
		transformOrigin={{ vertical: 'top', horizontal: 'right' }}
		open={!!anchorEl}
		onClose={() => setAnchorEl(null)}
	>
		<MenuItem onClick={action}>Change password</MenuItem>
		<MenuItem onClick={logout(setSessionUser)}>Logout</MenuItem>
	</Menu>
);

const setAnchor = setState => e => {
	setState(e.currentTarget);
}

const TopBar = ({ setSessionUser }) => {
	const classes = useStyles();
	const [anchorEl, setAnchorEl] = useState(null);

	return (
		<div>
			<AppBar position="static">
				<Toolbar>
					<IconButton edge="start" className={classes.menuButton} color="inherit" aria-label="menu">
						<MenuIcon />
					</IconButton>
					<Navigation classes={classes} />
					{/* <Typography variant="h6" >
						News
				</Typography> */}
					<div className={classes.grow} />
					{/* <Button color="inherit">notification</Button> */}
					<IconButton aria-label="show 17 new notifications" color="inherit">
						<Badge badgeContent={17} color="secondary">
							<NotificationsIcon />
						</Badge>
					</IconButton>
					<IconButton
						edge="end"
						aria-label="account of current user"
						aria-haspopup="true"
						onClick={setAnchor(setAnchorEl)}
						color="inherit"
					>
						<AccountCircle />
					</IconButton>
					{/* <Button color="inherit">user</Button> */}
				</Toolbar>
			</AppBar>
			<RenderMenu anchorEl={anchorEl} setAnchorEl={setAnchorEl} setSessionUser={setSessionUser} />
		</div>
	)
}

export default TopBar;