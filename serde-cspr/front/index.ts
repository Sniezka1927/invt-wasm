import {
  SqrtPrice,
  get_custom_struct,
  receive_example_from_js,
  send_example_to_js,
} from "../pkg/invt.js";

const init = async () => {
  // let decimalInstance = new D(125);
  let received = send_example_to_js();
  console.log(received);
  let result = receive_example_from_js(received);
  console.log(result);
  // Handling structs
  {
    let sqrtPrice: SqrtPrice = {
      v: BigInt(1),
    };
    get_custom_struct(sqrtPrice);
  }
  // // Math examples
  // {
  //   // get delta y

  //   let sqrtPriceA: SqrtPrice = {
  //     v: 234_878_324_943_782_000_000_000_000,
  //   };
  //   let sqrtPriceB: SqrtPrice = { v: 87_854_456_421_658_000_000_000_000 };
  //   let liquidity: Liquidity = { v: 983_983_249_092 };
  //   let delta_y_up = get_delta_y(sqrtPriceA, sqrtPriceB, liquidity, true);
  //   console.log(delta_y_up);
  // }
};

init();
