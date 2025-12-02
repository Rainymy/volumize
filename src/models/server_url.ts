import { proxyStorageAtom } from "$hook/proxyStorageAtom";
import { PORT } from "$type/constant";
import { getNumber } from "$util/generic";

// localStorage.clear();
const __SERVER_URL__ = "server_url" as const;
const __server_url__ = localStorage.getItem(__SERVER_URL__) ?? "192.168.1.115";
export const server_url = proxyStorageAtom(__server_url__, __SERVER_URL__);

const __SERVER_PORT__ = "server_port" as const;
const __server_port__ = getNumber(localStorage.getItem(__SERVER_PORT__)) ?? PORT.DEFAULT;
export const server_port = proxyStorageAtom(__server_port__, __SERVER_PORT__);
