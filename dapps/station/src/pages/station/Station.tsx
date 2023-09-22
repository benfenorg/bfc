import SwapObcToStablecoin from '../../components/Station/SwapObcToStablecoin';
import SwapStablecoinToObc from '../../components/Station/SwapStablecoinToObc';
import CoinSwap from '../../components/Station/CoinSwap'
function Station() {
    return (<div>
        <SwapObcToStablecoin/>
        <SwapStablecoinToObc />
        <CoinSwap />
    </div>)
}

export default Station