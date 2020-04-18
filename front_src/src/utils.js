exports.fillEntry = setState => event => setState(event.target.value);
exports.removeEvent = cb => event => { event.preventDefault() ; typeof cb === "function" && cb(event) }
exports.enterSubmit = cb => event => { typeof cb === "function" && event.key === 'Enter' && cb(event) }