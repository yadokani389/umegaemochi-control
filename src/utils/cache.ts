import * as fs from "@tauri-apps/plugin-fs";

export async function getAddress(): Promise<string> {
  if (await fs.exists("address.txt", { baseDir: fs.BaseDirectory.AppCache })) {
    const address = await fs.readTextFile("address.txt", {
      baseDir: fs.BaseDirectory.AppCache,
    });
    return address;
  } else {
    const file = await fs.create("address.txt", {
      baseDir: fs.BaseDirectory.AppCache,
    });
    await file.close();
    return "";
  }
}

export async function saveAddress(address: string): Promise<void> {
  await fs.writeTextFile("address.txt", address, {
    baseDir: fs.BaseDirectory.AppCache,
  });
}
