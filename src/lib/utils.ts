import { toast, ToastType } from "$stores/ToastStore"

export function parseArrayField(value: any, fieldName: string): any[] | null {
  try {
    const parsedValue = JSON.parse(value);

    if (Array.isArray(parsedValue)) {
      return parsedValue;
    } else {
      toast.set({
        type: ToastType.Error,
        description: `Error with array format on ${fieldName}.`
      });
      return null;
    }
  } catch (error) {
    console.error("Error parsing array field", fieldName, error);
    toast.set({
      type: ToastType.Error,
      description: `Error parsing array on ${fieldName}. ${error}`
    });
    return null;
  }
}
