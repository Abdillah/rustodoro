type Dispatch = (action: any) => any;

interface Model {
    props?: any,
    children?: any[],
    path?: string,
    dispatch?: Dispatch,
    context?: any,
    [props: string]: any,
}