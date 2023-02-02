import UserData from '@/data-objects/UserData';
import Resource from '@/lib/Resource';

export class UserResource extends Resource<UserData> {
    base(): Record<string, any> {
        return {
            id: this.data.id,
            admin: this.data.admin,
            email: this.data.email,
            first_name: this.data.firstName,
            last_name: this.data.lastName,
            name: this.data.name,
            uses_imperial_units: this.data.usesImperialUnits,
        };
    }
}
