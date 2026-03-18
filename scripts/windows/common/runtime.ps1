# Shared container runtime utilities dot-sourced by Windows command scripts.
# Callers must declare the $Runtime parameter and $DryRun switch before
# dot-sourcing this file.  Functions reference $containerRuntime and
# $repoRoot which are set by the caller after dot-sourcing.

function Write-Log([string]$Level, [string]$Message) {
    $colors = @{ INFO = "Cyan"; OK = "Green"; WARN = "Yellow"; ERR = "Red" }
    $icons  = @{ INFO = "ℹ️";   OK = "✅";    WARN = "⚠️";     ERR = "❌" }
    Write-Host "$($icons[$Level]) $Message" -ForegroundColor $colors[$Level]
}

# Invoke-Step COMMAND
# Prints (dry-run) or executes a shell command string, throwing on failure.
function Invoke-Step([string]$Command) {
    if ($DryRun) {
        Write-Log INFO "[dry-run] $Command"
    } else {
        Invoke-Expression $Command
        if ($LASTEXITCODE -ne 0) {
            throw "Command failed with exit code $LASTEXITCODE."
        }
    }
}

# Resolve-Runtime
# Returns "docker" or "podman" depending on what is installed.
# Handles the case where 'docker' is a shim/alias for podman.
function Resolve-Runtime {
    if ($Runtime) { return $Runtime }

    $dockerCmd = Get-Command docker -ErrorAction SilentlyContinue
    $podmanCmd = Get-Command podman -ErrorAction SilentlyContinue

    if ($dockerCmd -and $podmanCmd) {
        # If docker's resolved definition points at podman, prefer the real one.
        $definition = try { $dockerCmd.Definition } catch { '' }
        if ($definition -match 'podman') { return 'podman' }
        return 'docker'
    }
    if ($dockerCmd) { return 'docker' }
    if ($podmanCmd) { return 'podman' }
    throw "Neither docker nor podman is available."
}

function Test-ImageExists([string]$Tag) {
    if ($DryRun) { return $false }
    & $containerRuntime image inspect $Tag *> $null
    return $LASTEXITCODE -eq 0
}

# Ensure-Image TAG DOCKERFILE
# Builds the image only when it does not already exist locally.
function Ensure-Image([string]$Tag, [string]$Dockerfile) {
    if (Test-ImageExists $Tag) {
        Write-Log INFO "Reusing container image: $Tag"
        return
    }
    Write-Log INFO "Building container image: $Tag"
    Invoke-Step "$containerRuntime build -t $Tag -f `"$Dockerfile`" `"$repoRoot`""
}
