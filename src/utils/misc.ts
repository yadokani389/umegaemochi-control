import { ToastServiceMethods } from "primevue";

export function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export function addToast(
  toast: ToastServiceMethods,
  severity: "success" | "info" | "warn" | "error" | "secondary" | "contrast",
  summary: string,
  err: Error,
) {
  toast.add({
    severity,
    summary,
    detail: err + "\nMake sure both apps are the latest.",
    life: 3000,
  });
}
