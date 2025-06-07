import os
from asyncio import run, sleep

from httpx import AsyncClient

API_KEY = os.environ["API_KEY"]
DOMAIN = os.environ["DOMAIN"]
RECORD = os.environ["RECORD"]


async def get_current_ip() -> str:
    async with AsyncClient() as client:
        response = await client.get("https://api.ipify.org?format=json")
        response.raise_for_status()
        return response.json()["ip"]


async def update_records(
    current_ip: str,
):
    async with AsyncClient() as client:
        response = await client.put(
            f"https://developers.hostinger.com/api/dns/v1/zones/{DOMAIN}",
            headers={
                "Content-Type": "application/json",
                "Authorization": f"Bearer {API_KEY}",
            },
            json={
                "override": False,
                "zone": [
                    {
                        "name": RECORD,
                        "type": "A",
                        "ttl": 300,
                        "records": [{"content": current_ip}],
                    }
                ],
            },
        )

        response.raise_for_status()


async def sync():
    current_ip = await get_current_ip()
    await update_records(current_ip)


async def main():
    while True:
        try:
            await sync()
            print("DNS records updated successfully.")
        except Exception as e:
            print(f"Error during sync: {e}")

        await sleep(300)


if __name__ == "__main__":
    run(main())
