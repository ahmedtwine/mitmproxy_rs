"""
mitmproxy_rs Traffic Interception Test
Run with: just demo
"""

import asyncio
import mitmproxy_rs

async def handle_tcp_stream(stream):
    """Handle intercepted TCP connections"""
    print(f"TCP INTERCEPTED: {stream.peername} -> {stream.sockname}")
    try:
        data = await stream.read(1024)
        if data and data.startswith(b'GET '):
            request = data.decode('utf-8', errors='ignore').split('\n')[0]
            print(f"HTTP: {request}")
    except Exception as e:
        print(f"TCP error: {e}")

async def handle_udp_stream(stream):
    """Handle intercepted UDP packets"""
    print(f"UDP INTERCEPTED: {stream}")

def check_vpn_tunnel():
    """Check if VPN tunnel was created"""
    import subprocess
    try:
        result = subprocess.run(["scutil", "--nc", "list"], capture_output=True, text=True)
        return "mitmproxy" in result.stdout.lower()
    except:
        return False

async def main():
    print("mitmproxy_rs Traffic Interception Test")
    print("=" * 40)

    # Check if local redirect is available
    reason = mitmproxy_rs.local.LocalRedirector.unavailable_reason()
    if reason:
        print(f"Local redirect unavailable: {reason}")
        return

    print("Local redirect mode available")

    try:
        # Start the redirector
        print("\nStarting redirector...")
        redirector = await mitmproxy_rs.local.start_local_redirector(
            handle_tcp_stream,
            handle_udp_stream
        )
        print("Redirector started")

        # Check if VPN tunnel was created
        print("\nChecking VPN tunnel creation...")
        if check_vpn_tunnel():
            print("VPN tunnel created successfully!")

            # Test traffic interception
            print("\nTesting traffic interception...")
            redirector.set_intercept("all")
            print("Monitoring all network traffic for 10 seconds...")
            print("Try running: curl http://httpbin.org/get")

            await asyncio.sleep(10)

            print("\nTest completed")
        else:
            print("VPN tunnel creation failed!")
            print("\nThis is the core issue preventing traffic interception.")
            print("The Network Extension needs a VPN tunnel to redirect traffic.")
            diagnose_issue()

        redirector.close()

    except Exception as e:
        print(f"Redirector failed: {e}")
        diagnose_issue()

def diagnose_issue():
    """Diagnose the VPN tunnel issue"""
    import subprocess
    print("\nDIAGNOSIS:")

    # Check system extension
    try:
        result = subprocess.run(["systemextensionsctl", "list"], capture_output=True, text=True)
        if "mitmproxy" in result.stdout and "[activated enabled]" in result.stdout:
            print("System Extension: Activated and enabled")
        else:
            print("System Extension: Not properly activated")
            print("Fix: System Settings > General > Login Items & Extensions")
            return
    except:
        print("System Extension: Check failed")
        return

    # Check redirector app
    import os
    if os.path.exists("/Applications/Mitmproxy Redirector.app"):
        print("Redirector App: Installed")
    else:
        print("Redirector App: Missing")
        return

    print("VPN Tunnel: Creation failed")
    print("\nROOT CAUSE:")
    print("The Swift redirector app is not successfully creating")
    print("the VPN tunnel configuration via NETransparentProxyManager.")
    print("\nPOSSIBLE SOLUTIONS:")
    print("1. Check Console.app for VPN/NetworkExtension errors")
    print("2. Disable conflicting VPN apps (Tailscale, etc.)")
    print("3. Reset Network Settings in System Settings")
    print("4. Check if VPN permissions are blocked in Security settings")

if __name__ == "__main__":
    asyncio.run(main())
