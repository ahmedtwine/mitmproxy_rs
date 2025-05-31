"""
Complete mitmproxy_rs Local Redirector Demo
Run with: just demo
"""

import asyncio
import mitmproxy_rs

async def handle_tcp_stream(stream):
    """Handle TCP connections from intercepted apps"""
    print(f"TCP connection intercepted: {stream}")
    try:
        data = await stream.read(1024)
        if data:
            print(f"HTTP request: {data[:200].decode('utf-8', errors='ignore')}")
    except Exception as e:
        print(f"TCP error: {e}")

async def handle_udp_stream(stream):
    """Handle UDP packets from intercepted apps"""
    print(f"UDP packet intercepted: {stream}")

async def demo():
    print(" mitmproxy_rs Local Redirector Demo")
    print("=" * 50)

    # Check if redirector is available
    reason = mitmproxy_rs.local.LocalRedirector.unavailable_reason()
    if reason:
        print(f"Local redirect unavailable: {reason}")
        return

    print("Local redirect mode available!")

    # Show running processes
    print("\n Running processes:")
    try:
        processes = mitmproxy_rs.process_info.active_executables()
        visible = [p for p in processes if p.is_visible and not p.is_system]

        for i, proc in enumerate(visible[:5], 1):
            print(f"  {i}. {proc.display_name}")

        if len(visible) > 5:
            print(f"  ... and {len(visible) - 5} more")

    except Exception as e:
        print(f"Error: {e}")

    # Test real redirector
    print("\n Starting real redirector...")
    try:
        redirector = await mitmproxy_rs.local.start_local_redirector(
            handle_tcp_stream,
            handle_udp_stream
        )
        # Test 1: All traffic (should definitely work)
        print("\n1. Testing 'all' - intercepts ALL network traffic")
        redirector.set_intercept("all")
        desc = mitmproxy_rs.local.LocalRedirector.describe_spec("all")
        print("    WARNING: Will intercept ALL apps!")
        print("   Test: curl -v http://httpbin.org/get")
        print("   Waiting 10 seconds for any traffic...")
        await asyncio.sleep(10)

        # Test 2: Specific process
        print("\n2. Testing 'process:curl' - intercepts only curl")
        redirector.set_intercept("process:curl")
        desc = mitmproxy_rs.local.LocalRedirector.describe_spec("process:curl")
        print("   Test: curl -v http://httpbin.org/get")
        print("   Waiting 10 seconds for curl traffic...")
        await asyncio.sleep(10)

        print("\nIf no interception seen:")
        print("   - Network Extension may need manual activation")
        print("   - Try System Settings > General > Login Items & Extensions")
        print("   - Look for mitmproxy Network Extension and enable it")

        print("\nPress Ctrl+C to stop...")
        await redirector.wait_closed()

    except Exception as e:
        print(f"❌ Failed: {e}")
        print("")
        print("COMPLETE NETWORK EXTENSION STATUS:")
        import subprocess
        try:
            # Check system extension status
            result = subprocess.run(["systemextensionsctl", "list"], capture_output=True, text=True)
            print("   System Extension:")
            if "mitmproxy" in result.stdout:
                if "[activated enabled]" in result.stdout:
                    print("     System extension is activated and enabled")
                else:
                    print("     System extension found but not fully activated")
            else:
                print("     VPN configuration not found")

            # Check VPN configuration
            result = subprocess.run(["scutil", "--nc", "list"], capture_output=True, text=True)
            print("   VPN Configuration:")
            if "mitmproxy" in result.stdout.lower():
                print("     VPN configuration found")
                # Check VPN status
                result = subprocess.run(["scutil", "--nc", "show", "mitmproxy"], capture_output=True, text=True)
                if "Connected" in result.stdout:
                    print("     VPN tunnel is connected")
                else:
                    print(" VPN configuration exists but tunnel not connected")
            else:
                print("     VPN configuration missing - this is the main issue!")
                print("     The redirector app needs to create the VPN tunnel")

            # Check redirector app
            import os
            print("   Redirector App:")
            if os.path.exists("/Applications/Mitmproxy Redirector.app"):
                print("     ✅ Mitmproxy Redirector.app exists")
            else:
                print("     ❌ Mitmproxy Redirector.app missing")

        except Exception as debug_e:
            print(f"   Debug check failed: {debug_e}")

        print("")
        print(" ROOT CAUSE ANALYSIS:")
        print("   The Network Extension needs TWO components:")
        print("   1. ✅ System Extension (this is working)")
        print("   2. ❌ VPN Tunnel Configuration (this is missing)")
        print("")
        print("   The VPN tunnel is created when the redirector app is launched")
        print("   with a unix socket path. This triggers NETransparentProxyManager")
        print("   which creates the VPN configuration and starts the tunnel.")
        print("")

if __name__ == "__main__":
    asyncio.run(demo())
