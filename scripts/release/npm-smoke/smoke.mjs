import { Gram, GraphClass, init } from "@relateby/pattern";

await init();

if (GraphClass.NODE !== "node") {
  throw new Error("GraphClass constants not available");
}

if (typeof Gram?.parse !== "function" || typeof Gram?.stringify !== "function") {
  throw new Error("Gram API not available from @relateby/pattern");
}

console.log("npm smoke test passed");
