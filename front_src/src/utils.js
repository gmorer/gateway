import request from "request-promise"

const API_URL = "http://localhost:8088/"

export const fillEntry = setState => event => setState(event.target.value);

export const removeEvent = cb => event => { event.preventDefault(); typeof cb === "function" && cb(event) }

export const enterSubmit = cb => event => { typeof cb === "function" && event.key === 'Enter' && cb(event) }

export const apiCall = async options => {
	options.uri = `${API_URL}${options.uri}`;
	options.json = true;
	options.method = 'POST';
	// options.simple = false;
	// add the headerfor the return type
	// add the token logic
	try {
		return await request(options);
	} catch (e) {
		if (!!e.error && !!e.error.error) {
			throw e.error.error
		} else {
			throw e.message
		}
	}
}

export const usernameFromToken = token => {
	if (!token || typeof token !== "string") return null;
	const [algo, bclaims, key] = token.split('.');
	if (!algo || !bclaims || !key) return null;
	let claims = JSON.parse(atob(bclaims));
	if (!claims) return null;
	return claims.sub;
}