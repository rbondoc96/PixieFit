type RouteAttributes<TPath extends string> = {
    name: string | symbol;
    pathName: TPath;
    parents?: string[];
};

export class Route<TPath extends string = string> {
    private constructor(
        public readonly name: string | symbol,
        public readonly pathName: TPath,
        public readonly parents: string[] = [],
    ) {}

    public static create<TPath extends string = string>(
        attributes: RouteAttributes<TPath>,
    ): Route<TPath> {
        return new Route(attributes.name, attributes.pathName, attributes.parents ?? []);
    }

    public get path(): `/${TPath}` {
        return `/${this.pathName}`;
    }

    public get href(): `/${TPath}` | `/${string}/${TPath}` {
        if (this.parents.length > 0) {
            return `/${this.parents.join('/')}/${this.pathName}`;
        }

        return `/${this.pathName}`;
    }
}

export default Route.create;
