# Release & Deployment Guide

This file contains handy scripts you can run whenever you want to push code updates, create a version tag, and deploy them to GitHub automatically.

We provide two scripts (for Windows and Linux/Mac). You can execute these scripts directly from your terminal whenever there's an update.

---

## 💻 Windows Users (PowerShell)

Use the `deploy.ps1` script. 

**How to run:**
Open your PowerShell terminal, then run the following command by supplying your commit message and release version (e.g., releasing version `v1.0.1`):

```powershell
.\deploy.ps1 -CommitMessage "Added webhook feature" -VersionTag "v1.0.1"
```

*(If you encounter an "Execution of scripts is disabled" error, run `Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass` first).*

---

## 🐧 Linux / Mac Users (Bash)

Use the `deploy.sh` script. 

**How to run:**
Ensure the script is executable (only needs to be done once):
```bash
chmod +x deploy.sh
```

Then run the script (e.g., releasing version `v1.0.1`):
```bash
./deploy.sh "Added webhook feature" "v1.0.1"
```

---

## ⚙️ What do these scripts do?
When executed, the script will sequentially perform:
1. `git add .` (Stages all recent changes).
2. `git commit -m "Your Message"` (Creates a commit with your dynamic message).
3. `git tag vX.X.X` (Creates a local Release Tag).
4. `git push origin main` (Pushes the source code to the main GitHub branch).
5. `git push origin vX.X.X` (Pushes the version tag to GitHub, which can trigger GitHub Releases/Actions if configured).
