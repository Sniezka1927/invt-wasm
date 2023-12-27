import { D, multiply } from "./pkg/invt.js";

{
  let decimalInstance = new D(BigInt(10));
  console.log(`Decimal Instance = ${decimalInstance}`);
  let anotherDecimalInstance = new D(BigInt(5));

  let previousValue = decimalInstance.v;
  console.log(`Decimal previous value = ${previousValue}`);

  decimalInstance.v = BigInt(20);

  let updatedValue = decimalInstance.v;
  console.log(`Decimal current value = ${updatedValue}`);

  console.log(anotherDecimalInstance.v, updatedValue);

  let product = multiply(anotherDecimalInstance.v, updatedValue);
  console.log(`Product = ${product}`);
  decimalInstance.v = product;
  console.log(`Decimal current value = ${decimalInstance.v}`);

  console.table(decimalInstance);
}
