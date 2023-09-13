import { ReactComponent as DaoAgree } from '../../assets/DaoAgree.svg';
import { ReactComponent as DaoReject } from '../../assets/DaoReject.svg';

export const AgreeSpan = () => (
    <div className="h-5 flex items-center gap-0.5 border border-obc-green rounded bg-obc-green_10p px-1 text-body text-obc-green font-medium">
        <DaoAgree/>同意
    </div>
)

export const RejectSpan = () => (
    <div className="h-5 flex items-center gap-0.5 border border-obc-green rounded bg-obc-green_10p px-1 text-body text-obc-green font-medium">
        <DaoReject/>反对
    </div>
)

export const StatusSpan = () => (
    <div className="h-5 flex items-center border border-obc-border rounded  px-1 text-body text-obc-text1 font-medium">
        asddsdsd
    </div>
)