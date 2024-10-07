import { createPromiseClient } from "@connectrpc/connect";
import { createGrpcWebTransport } from "@connectrpc/connect-web";

// Import the ViewService from the Penumbra package
import { ViewService } from "@buf/penumbra-zone_penumbra.connectrpc_es/penumbra/view/v1/view_connect";

// Create a transport for gRPC-web
const transport = createGrpcWebTransport({
  baseUrl: "http://localhost:3333", // Adjust this if your server is on a different port
});

// Create and export the client
export const penumbraClient = createPromiseClient(ViewService, transport);
