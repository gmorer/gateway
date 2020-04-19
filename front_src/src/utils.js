import request from "request-promise"

const API_URL = "http://localhost:8088/"

export const fillEntry = setState => event => setState(event.target.value);

export const removeEvent = cb => event => { event.preventDefault(); typeof cb === "function" && cb(event) }

export const enterSubmit = cb => event => { typeof cb === "function" && event.key === 'Enter' && cb(event) }

export const apiCall = options => {
	options.uri = `${API_URL}${options.uri}`;
	options.json = true;
	options.method = 'POST';
	// add the headerfor the return type
	// add the token logic
	return request(options);
}