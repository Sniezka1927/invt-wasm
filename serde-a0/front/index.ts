import {
  Liquidity,
  SqrtPrice,
  TokenAmount,
  get_custom_struct,
  get_delta_y,
  receive_example_from_js,
  send_example_to_js,
} from "../pkg/invt.js";

import { default as BN } from "bn.js";

const init = async () => {
  // let decimalInstance = new D(125);
  // Set serializer config
  let test = new BN(1000);
  console.log(typeof test);
  let received = send_example_to_js();
  console.log(received);
  let result = receive_example_from_js(received);
  console.log(result);
  // Handling structs
  {
    let tokenAmount: TokenAmount = {
      v: BigInt(1),
    };

    get_custom_struct(tokenAmount);
  }
  // Math examples
  {
    // get delta y
    let sqrtPriceA: SqrtPrice = {
      v: BigInt(234878324943782000000000000),
    };
    let sqrtPriceB: SqrtPrice = { v: BigInt(87854456421658000000000000) };
    let liquidity: Liquidity = { v: BigInt(983983249092) };
    let delta_y_up = get_delta_y(sqrtPriceA, sqrtPriceB, liquidity, true);
    let delta_y_down = get_delta_y(sqrtPriceA, sqrtPriceB, liquidity, false);
    console.log(delta_y_up);
    console.log(delta_y_down);
    // // 144669023.842474597804911408
    // assert_eq!(result_down, TokenAmount(144669023));
    // assert_eq!(result_up, TokenAmount(144669024));
  }
};

init();
