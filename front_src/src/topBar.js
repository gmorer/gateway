import React, { useState } from 'react';
import {
	AppBar,
	Toolbar,
	IconButton,
	Typography,
	Menu,
	MenuItem,
	Badge,
	Breadcrumbs,
	Dialog,
	DialogTitle,
	List,
	ListItem,
	ListItemAvatar,
	Avatar,
	ListItemText
} from '@material-ui/core';
import { makeStyles } from '@material-ui/core/styles';
import {
	Menu as MenuIcon,
	Notifications as NotificationsIcon,
	Home as HomeIcon,
	MoreVert as MoreIcon,
	Person as PersonIcon,
	Storage as StorageIcon,
	Note as NoteIcon,
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
		cursor: 'pointer'
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
		fontSize: "2em",
		display: 'none',
		[theme.breakpoints.up('md')]: {
			display: 'flex',
		},
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

/* To do: change password */
const action = () => ({})

const services = [
	"Home",
	"Contacts",
	"Drive",
	"Notes"
]

const servicesIcons = {
	"Home": HomeIcon,
	"Contacts": PersonIcon,
	"Drive": StorageIcon,
	"Notes": NoteIcon
}

const renderIcon = (name, props = {}) => {
	let Component = servicesIcons[name]
	return <Component {...props} />
}

const logout = (setSessionUser) => _ => {
	localStorage.removeItem("refresh_token");
	sessionStorage.acces_token = undefined;
	setSessionUser(null);
}

const Navigation = ({ classes, service, setService }) => {
	return (
		<Breadcrumbs aria-label="breadcrumb" separator="|" className={classes.breadcrumbs}>
			{services.map((name, index) => (
				<Typography color={name === service ? "textPrimary" : "textSecondary"} className={classes.link} key={index} onClick={() => setService(name)}>
					{renderIcon(name, { className: classes.desktopIcon })}
					{name === service ? "[ " + name + " ]" : name}
				</Typography>
			))
			}
		</Breadcrumbs >
	)
}

const MobileServiceList = ({ isOpen, setIsOpen, setService, service, classes }) => {
	const clickOnService = name => _ => {
		if (service !== name) {
			setService(name);
			setIsOpen(false);
		}
	}

	return (
		<Dialog onClose={() => setIsOpen(false)} aria-labelledby="simple-dialog-title" open={isOpen}>
			<DialogTitle id="simple-dialog-title">Select your service</DialogTitle>
			<List>
				{services.map((name, index) => (
					<ListItem selected={service === name} button onClick={clickOnService(name)} key={index}>
						<ListItemAvatar>
							<Avatar className={classes.avatar}>
								{renderIcon(name)}
							</Avatar>
						</ListItemAvatar>
						<ListItemText primary={name} />
					</ListItem>
				))
				}
			</List>
		</Dialog>
	)
}

const RenderMenu = ({ anchorEl, setAnchorEl, setSessionUser, classes, service, setService }) => {
	const [dialog, setDialog] = useState(false);

	return (
		<div>
			<Menu
				anchorEl={anchorEl}
				anchorOrigin={{ vertical: 'top', horizontal: 'right' }}
				keepMounted
				transformOrigin={{ vertical: 'top', horizontal: 'right' }}
				open={!!anchorEl}
				onClose={() => setAnchorEl(null)}
			>
				<MenuItem className={classes.sectionMobile} onClick={() => { setDialog(true); setAnchorEl(null) }}>Services</MenuItem>
				<MenuItem onClick={action}>Change password</MenuItem>
				<MenuItem onClick={logout(setSessionUser)}>Logout</MenuItem>
			</Menu>
			<div className={classes.sectionMobile}>
				<MobileServiceList isOpen={dialog} setIsOpen={setDialog} setService={setService} service={service} classes={classes} />
			</div>
		</div>
	);
}

const setAnchor = setState => e => {
	setState(e.currentTarget);
}

const TopBar = ({ setSessionUser }) => {
	const [anchorEl, setAnchorEl] = useState(null);
	const [service, setService] = useState(services[0])
	const classes = useStyles();

	return (
		<div>
			<AppBar position="static">
				<Toolbar>
					<IconButton edge="start" className={classes.menuButton} color="inherit" aria-label="menu">
						<MenuIcon />
					</IconButton>
					<Navigation classes={classes} service={service} setService={setService} />
					<Typography variant="h6" className={classes.sectionMobile}>
						{service}
					</Typography>
					<div className={classes.grow} />
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
						<AccountCircle className={classes.sectionDesktop} />
						<MoreIcon className={classes.sectionMobile} />
					</IconButton>
				</Toolbar>
			</AppBar>
			<RenderMenu service={service} setService={setService} anchorEl={anchorEl} setAnchorEl={setAnchorEl} setSessionUser={setSessionUser} classes={classes} />
		</div>
	)
}

export default TopBar;