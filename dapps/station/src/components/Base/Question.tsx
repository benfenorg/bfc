import { ReactComponent as QuestionIcon } from '~/assets/question_icon.svg';
export function Question({id}:{id:string}){
    return (
        <>
            <div id={id}><QuestionIcon /></div>
        </>
    )
}