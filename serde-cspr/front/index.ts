import {
  Liquidity,
  SqrtPrice,
  get_custom_struct,
  get_delta_y,
} from "../pkg/invt.js";

const init = async () => {
  // let decimalInstance = new D(125);
  // let received = send_example_to_js();
  // console.log(received);
  // let result = receive_example_from_js(received);
  // console.log(result);
  // Handling structs
  {
    let sqrtPrice: SqrtPrice = {
      v: "1",
    };
    get_custom_struct(sqrtPrice);
  }
  // Math examples
  {
    // get delta y
    // 1073741824
    let sqrtPriceA: SqrtPrice = {
      v: "234878324943782000000000000",
    };
    let sqrtPriceB: SqrtPrice = { v: "87854456421658000000000000" };
    let liquidity: Liquidity = { v: "983983249092" };
    let delta_y_up = get_delta_y(sqrtPriceA, sqrtPriceB, liquidity, true);
    let delta_y_down = get_delta_y(sqrtPriceA, sqrtPriceB, liquidity, false);
    console.log(delta_y_up);
    console.log(delta_y_down);
    // // 144669023.842474597804911408
    // assert_eq!(result_down, TokenAmount::new(U256::from(1446690238)));
    // assert_eq!(result_up, TokenAmount::new(U256::from(1446690239)));
  }
};

init();
