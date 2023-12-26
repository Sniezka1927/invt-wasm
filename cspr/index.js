import { Primitive } from "./pkg/invt.js";

let primitiveInstance = new Primitive(BigInt(42));
let value = primitiveInstance.v;

console.log(`Primitive Instance = ${primitiveInstance}`);
console.log(`Primitive value = ${value}`);
console.table(primitiveInstance);
