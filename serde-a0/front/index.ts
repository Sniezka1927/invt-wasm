import {
  Liquidity,
  SqrtPrice,
  TokenAmount,
  get_custom_struct,
  get_delta_y,
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
    let tokenAmount: TokenAmount = {
      v: 1,
    };

    get_custom_struct(tokenAmount);
  }
  // Math examples
  {
    // get delta y

    let sqrtPriceA: SqrtPrice = {
      v: 25,
    };
    let sqrtPriceB: SqrtPrice = { v: 30 };
    let liquidity: Liquidity = { v: 1 };
    let delta_y_up = get_delta_y(sqrtPriceA, sqrtPriceB, liquidity, true);
    console.log(delta_y_up);
  }
};

init();
