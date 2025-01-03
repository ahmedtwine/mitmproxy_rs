use anyhow::{Context, Result};
use rcgen::{
    Certificate, CertificateParams, DistinguishedName, DnType, KeyPair, PKCS_ECDSA_P256_SHA256,
};
use std::path::PathBuf;
use tracing::*;

pub const CA_NAME: &str = "Mitmproxy Desktop Root CA";

pub struct CertManager {
    ca_cert: Option<Certificate>,
    cert_path: PathBuf,
    key_path: PathBuf,
}

impl CertManager {
    pub fn new() -> Result<Self> {
        let config_dir = directories::ProjectDirs::from("org", "mitmproxy", "desktop")
            .context("Failed to get project directories")?
            .config_dir()
            .to_path_buf();

        std::fs::create_dir_all(&config_dir)?;

        Ok(Self {
            ca_cert: None,
            cert_path: config_dir.join("ca.crt"),
            key_path: config_dir.join("ca.key"),
        })
    }

    pub fn has_ca(&self) -> bool {
        if !self.cert_path.exists() || !self.key_path.exists() {
            return false;
        }

        #[cfg(target_os = "macos")]
        {
            // Also verify the certificate is properly installed in keychain
            match self.verify_cert_in_keychain() {
                Ok(is_valid) => is_valid,
                Err(e) => {
                    warn!("Failed to verify certificate in keychain: {}", e);
                    false
                }
            }
        }

        #[cfg(not(target_os = "macos"))]
        true
    }

    pub fn get_install_instructions(&self) -> String {
        #[cfg(target_os = "macos")]
        return "Additional steps to complete setup:\n\
                1. Restart your browsers for the certificate to take effect\n\
                2. For Firefox users:\n\
                   - Open Firefox Preferences\n\
                   - Search for 'Certificates'\n\
                   - Click 'View Certificates'\n\
                   - Go to 'Authorities' tab\n\
                   - Click 'Import' and select the certificate from:\n\
                   - '~/Library/Application Support/org.mitmproxy.desktop/ca.crt'\n\
                3. If you see any SSL warnings in apps:\n\
                   - Open Keychain Access\n\
                   - Find 'Mitmproxy Desktop Root CA'\n\
                   - Double click it\n\
                   - Expand 'Trust'\n\
                   - Set 'When using this certificate' to 'Always Trust'"
            .to_string();

        #[cfg(target_os = "linux")]
        return "To complete CA installation:\n\
                1. The certificate has been copied to /usr/local/share/ca-certificates/\n\
                2. Run 'sudo update-ca-certificates' if not already done\n\
                3. Restart your browser"
            .to_string();

        #[cfg(target_os = "windows")]
        return "To complete CA installation:\n\
                1. Open 'Manage Computer Certificates'\n\
                2. Find 'Mitmproxy Desktop Root CA' under 'Trusted Root Certification Authorities'\n\
                3. Verify it's properly installed\n\
                4. Restart your browser".to_string();
    }

    pub fn generate_ca(&mut self) -> Result<()> {
        info!("Generating new CA certificate...");

        // Clean up old certificates first
        #[cfg(target_os = "macos")]
        self.cleanup_old_certificates()?;

        let mut params = CertificateParams::default();
        params.alg = &PKCS_ECDSA_P256_SHA256;

        let mut distinguished_name = DistinguishedName::new();
        distinguished_name.push(DnType::OrganizationName, "Mitmproxy Desktop Local CA");
        distinguished_name.push(DnType::CommonName, CA_NAME);
        params.distinguished_name = distinguished_name;

        params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
        params.key_pair = Some(KeyPair::generate(&PKCS_ECDSA_P256_SHA256)?);

        let cert = Certificate::from_params(params)?;

        // Save the certificate and private key
        std::fs::write(&self.cert_path, cert.serialize_pem()?)?;
        std::fs::write(&self.key_path, cert.serialize_private_key_pem())?;

        self.ca_cert = Some(cert);
        self.install_ca()?;

        info!("CA certificate generated and saved to {:?}", self.cert_path);
        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn verify_cert_in_keychain(&self) -> Result<bool> {
        use std::process::Command;

        // Check all keychains for our certificates
        let find_output = Command::new("security")
            .args(&[
                "find-identity",
                "-p",
                "ssl",
                "-a", // All matches
            ])
            .output()?;

        let output_str = String::from_utf8_lossy(&find_output.stdout);

        // If we find any of our certificates, they need cleanup
        Ok(output_str.contains("Mitmproxy Desktop Root CA"))
    }

    #[cfg(target_os = "macos")]
    fn cleanup_old_certificates(&self) -> Result<()> {
        use std::process::Command;
        info!("Cleaning up old certificates...");

        // First verify if we need to clean up
        if !self.verify_cert_in_keychain()? {
            info!("No existing certificates found to clean up");
            return Ok(());
        }

        // Get list of all keychains
        let keychain_list = Command::new("security")
            .args(&["list-keychains"])
            .output()?;

        let keychain_output = String::from_utf8_lossy(&keychain_list.stdout);

        // Parse keychain paths
        let keychains: Vec<_> = keychain_output
            .lines()
            .filter_map(|line| {
                let cleaned = line.trim().trim_matches('"');
                if cleaned.contains("keychain") {
                    Some(cleaned.to_string())
                } else {
                    None
                }
            })
            .collect();

        info!("Found {} keychains to clean", keychains.len());

        // Clean from each keychain
        for keychain in keychains {
            info!("Cleaning from keychain: {}", keychain);

            // First try to find all instances of our certificate
            let find_output = Command::new("security")
                .args(&[
                    "find-certificate",
                    "-c",
                    CA_NAME,
                    "-a", // All matches
                    "-Z", // Show SHA-1 hash
                    "-k",
                    &keychain,
                ])
                .output()?;

            let output_str = String::from_utf8_lossy(&find_output.stdout);

            // Extract SHA-1 hashes
            for line in output_str.lines() {
                if line.starts_with("SHA-1 hash:") {
                    let hash = line.split_whitespace().last().unwrap_or("");
                    if !hash.is_empty() {
                        // Delete by hash which is more reliable
                        let _ = Command::new("sudo")
                            .args(&["security", "delete-certificate", "-Z", hash, &keychain])
                            .output()?;
                    }
                }
            }
        }

        // Verify cleanup was successful
        if self.verify_cert_in_keychain()? {
            warn!("Some certificates may remain in the keychain. You may need to manually remove them from Keychain Access.");
        } else {
            info!("All certificates successfully cleaned up");
        }

        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn install_ca(&self) -> Result<()> {
        use std::env;
        use std::process::Command;

        info!("Installing CA certificate in macOS Keychain...");

        // Get the absolute path to user's login keychain
        let home = env::var("HOME").context("Failed to get HOME directory")?;
        let login_keychain = format!("{}/Library/Keychains/login.keychain-db", home);

        // First, import the certificate into the login keychain
        let status = Command::new("security")
            .args(&[
                "import",
                self.cert_path.to_str().unwrap(),
                "-k",
                &login_keychain,
                "-t",
                "cert", // Type is certificate
                "-f",
                "pemseq", // Format is PEM sequence
            ])
            .status()?;

        if !status.success() {
            // If login keychain fails, try system keychain with admin privileges
            info!("Requesting admin access to install certificate...");
            let status = Command::new("osascript")
                .args(&[
                    "-e",
                    &format!(
                        "do shell script \"security import '{}' -k /Library/Keychains/System.keychain -t cert -f pemseq\" with administrator privileges",
                        self.cert_path.display()
                    ),
                ])
                .status()?;

            if !status.success() {
                anyhow::bail!("Failed to import certificate to keychain");
            }
        }

        info!("Certificate imported, setting trust settings...");

        // Mark the certificate as trusted for SSL using add-trusted-cert
        let status = Command::new("security")
            .args(&[
                "add-trusted-cert",
                "-d", // Add to admin cert store
                "-r",
                "trustRoot",
                "-p",
                "ssl",
                "-k",
                &login_keychain,
                self.cert_path.to_str().unwrap(),
            ])
            .status()?;

        if !status.success() {
            // If that fails, try with admin privileges
            let status = Command::new("osascript")
                .args(&[
                    "-e",
                    &format!(
                        "do shell script \"security add-trusted-cert -d -r trustRoot -p ssl -k /Library/Keychains/System.keychain '{}'\" with administrator privileges",
                        self.cert_path.display()
                    ),
                ])
                .status()?;

            if !status.success() {
                anyhow::bail!("Failed to trust certificate. Please open Keychain Access and trust the certificate manually.");
            }
        }

        // Verify the certificate is properly installed and trusted
        let verify = Command::new("security")
            .args(&[
                "verify-cert",
                "-c",
                self.cert_path.to_str().unwrap(),
                "-p",
                "ssl",
            ])
            .output()?;

        if !verify.status.success() {
            info!("Certificate may need manual trust settings. Please check Keychain Access.");
            info!("Certificate is installed but may require manual trust configuration in Keychain Access.");
        } else {
            info!("CA certificate successfully installed and trusted in macOS Keychain");
        }

        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn install_ca(&self) -> Result<()> {
        use std::process::Command;

        info!("Installing CA certificate in Linux trust store...");

        // Copy to system trust store
        std::fs::copy(
            &self.cert_path,
            "/usr/local/share/ca-certificates/mitmproxy-desktop.crt",
        )?;

        // Update CA certificates
        let status = Command::new("update-ca-certificates").status()?;

        if !status.success() {
            anyhow::bail!("Failed to update CA certificates");
        }

        info!("CA certificate installed in Linux trust store");
        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn install_ca(&self) -> Result<()> {
        use std::process::Command;

        info!("Installing CA certificate in Windows trust store...");

        let status = Command::new("certutil")
            .args(&["-addstore", "ROOT"])
            .arg(&self.cert_path)
            .status()?;

        if !status.success() {
            anyhow::bail!("Failed to install CA certificate");
        }

        info!("CA certificate installed in Windows trust store");
        Ok(())
    }

    pub fn get_cert_path(&self) -> &PathBuf {
        &self.cert_path
    }

    pub fn get_key_path(&self) -> &PathBuf {
        &self.key_path
    }
}
