<script lang="ts">
  import * as SimpleIcons from "simple-icons";

  export let processName: string;
  export let size: number = 16;

  function handleImageError(event: Event) {
    const img = event.target as HTMLImageElement;
    img.src = getIconForProcess("default");
    img.onerror = null;
  }

  function getIconForProcess(name: string): string {
    // First try with com.company.something pattern
    if (name.startsWith("com.")) {
      const companyName = name.replace(/^com\.([^.]+)\..*$/, "$1");
      const formattedCompanyName =
        companyName.charAt(0).toUpperCase() + companyName.slice(1);
      const companyIconKey = `si${formattedCompanyName}`;
      const companyIcon =
        SimpleIcons[companyIconKey as keyof typeof SimpleIcons];

      if (companyIcon) {
        // Use theme color instead of brand color
        const color = getComputedStyle(document.documentElement)
          .getPropertyValue("--text")
          .trim();
        const svg =
          typeof companyIcon === "object" && "svg" in companyIcon
            ? companyIcon.svg
            : "";
        const svgWithColor = svg.replace("<svg", `<svg fill="${color}"`);
        return `data:image/svg+xml;base64,${btoa(svgWithColor)}`;
      }
    }

    // If no company icon found, fall back to original implementation
    const cleanName = name
      .replace(/\.(app|exe)$/i, "")
      .replace(/[-_./\\]/g, " ")
      .split(" ")[0]
      .trim()
      .toLowerCase();

    const formattedName =
      cleanName.charAt(0).toUpperCase() + cleanName.slice(1);
    const iconKey = `si${formattedName}`;
    let simpleIcon = SimpleIcons[iconKey as keyof typeof SimpleIcons];

    // Default icon if no match found
    if (!simpleIcon) {
      simpleIcon = SimpleIcons.siGhostery;
    }

    // Use theme color instead of brand color
    const color = getComputedStyle(document.documentElement)
      .getPropertyValue("--text")
      .trim();

    const svg =
      typeof simpleIcon === "object" && "svg" in simpleIcon
        ? simpleIcon.svg
        : "";
    const svgWithColor = svg.replace("<svg", `<svg fill="${color}"`);
    return `data:image/svg+xml;base64,${btoa(svgWithColor)}`;
  }
</script>

<img
  class="process-icon"
  src={getIconForProcess(processName)}
  alt=""
  height={size}
  width={size}
  on:error={handleImageError}
/>

<style>
  .process-icon {
    width: 16px;
    height: 16px;
    object-fit: contain;
  }
</style>
