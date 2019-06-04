import { element } from 'deku';

export default function (props) {
    return (
        <div class="">
            {(new Date).toTimeString()}
        </div>
    );
}