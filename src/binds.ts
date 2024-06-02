// bindings to rust layer
import { invoke } from "@tauri-apps/api/tauri";

export interface Password {
    domain: string,
    password: string
}

export class Binding {
    static get_master_hash(): Promise<string> {
        return invoke("get_master_hash", {});
    }

    static create_password(domain: string, password: string): Promise<void> {
        return new Promise((resolve) => {
            invoke("create_password", {
                domain: domain,
                password: password
            })
            .then(() => {
                resolve();
            })
        })
    }

    static generate_password(): Promise<string> {
        return invoke("generate_password", {});
    }

    static get_passwords(filter: string): Promise<Password[]> {
        return invoke("get_passwords", {
            filter: filter
        })
    }

    static setup_master_password(password: string, confirmPassword: string): Promise<string> {
        return invoke("setup_master_password", {password: password, confirmPassword: confirmPassword});
    }

    static login(password: string): Promise<string> {
        return invoke("login", {password: password});
    }

    static delete(domain: string): Promise<void> {
        return invoke("delete_password", {
            domain: domain
        });
    }
};