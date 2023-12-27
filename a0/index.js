import { D, add, big_mul, from_integer, mul_up, multiply } from "./pkg/invt.js";

const init = async () => {
  {
    let decimalInstance = new D(BigInt(10));
    console.log(`Decimal Instance = ${decimalInstance}`);
    let previousValue = decimalInstance.v;
    console.log(`Decimal previous value = ${previousValue}`);
    decimalInstance.v = BigInt(20);
    let updatedValue = decimalInstance.v;
    console.log(`Decimal current value = ${updatedValue}`);
  }

  {
    {
      let a = new D(BigInt(10));
      let b = new D(BigInt(5));
      let addition = add(a, b);
      console.log(`Decimal add result = ${addition.v}`);
    }
    {
      let a = new D(BigInt(10));
      let b = new D(BigInt(5));
      let result = multiply(a, b);
      console.log(`Decimal multiply result = ${result.v}`);
    }
    {
      let a = new D(BigInt(10));
      let b = new D(BigInt(5));
      let big_mul_result = big_mul(a, b);
      console.log(`Decimal big_mul result = ${big_mul_result.v}`);
    }
    {
      let a = new D(BigInt(10));
      let b = new D(BigInt(5));
      let mul_up_result = mul_up(a, b);
      console.log(`Decimal mul_up result = ${mul_up_result.v}`);
    }
    {
      let from_integer_result = from_integer(BigInt(10));
      console.log(`Decimal from_integer result = ${from_integer_result.v}`);
    }
  }
};

init();
