if (!window.fhqconfig) window.fhqconfig = {};

fhqconfig.base_url = 'ws://' + window.location.hostname + ":1234/api-ws/";

if (window.location.protocol == "https:") {
	fhqconfig.base_url = "wss://" + window.location.hostname + ":4613/api-wss/";
}

if (window.location.hostname == "freehackquest.com") {
	fhqconfig.base_url = 'wss://freehackquest.com/api-wss/';
}
