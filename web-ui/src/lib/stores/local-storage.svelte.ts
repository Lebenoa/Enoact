export type LocalStorageStoreConfig<T> = {
    serializer?: (value: T) => string;
    deserializer?: (value: string) => T;
}

export class LocalStorageStore<T> {
    #current: T | undefined = $state(undefined);
    private config = {
        serializer: (value: T) => JSON.stringify(value),
        deserializer: (value: string) => JSON.parse(value) as T,
    };

    constructor(private key: string, initialValue: T | undefined = undefined, overrides: LocalStorageStoreConfig<T> = {}) {
        if (!localStorage) {
            throw new Error("LocalStorage is not available");
        }

        if (overrides.serializer) this.config.serializer = overrides.serializer;
        if (overrides.deserializer) this.config.deserializer = overrides.deserializer;

        const existing = localStorage.getItem(this.key);
        if (existing) {
            this.#current = this.config.deserializer(existing);
        } else if (initialValue) {
            this.#current = initialValue;
        }
    }

    get(): T | undefined {
        return this.#current;
    }

    set(value: T): void {
        localStorage.setItem(this.key, this.config.serializer(value));
        this.#current = value;
    }
}
