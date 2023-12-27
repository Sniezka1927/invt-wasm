import {
  get_delta_y,
  Liquidity,
  receive_example_from_js,
  send_example_to_js,
  SqrtPrice,
} from "../pkg/invt.js";

// todo start it

const init = async () => {
  // let decimalInstance = new D(125);
  let received = send_example_to_js();
  console.log(received);
  let result = receive_example_from_js(received);
  console.log(result);
  // Math examples
  {
    // get delta y

    let sqrtPriceA: SqrtPrice = {
      v: 234_878_324_943_782_000_000_000_000,
    };
    let sqrtPriceB: SqrtPrice = { v: 87_854_456_421_658_000_000_000_000 };
    let liquidity: Liquidity = { v: 983_983_249_092 };
    let delta_y_up = get_delta_y(sqrtPriceA, sqrtPriceB, liquidity, true);
  }
};

init();
