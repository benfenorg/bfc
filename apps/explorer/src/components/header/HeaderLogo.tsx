import { ReactComponent as DarkHeaderLogo } from '../../assets/dark_header_logo.svg';
import { ReactComponent as LigthHeaderLogo } from '../../assets/header_logo.svg';

export const HeaderLogo = ({isDarker}:any) => {
    return isDarker ? <DarkHeaderLogo /> : <LigthHeaderLogo />
}