param (
    [Parameter(Mandatory=$true)]
    [string]$CommitMessage,
    
    [Parameter(Mandatory=$true)]
    [string]$VersionTag
)

Write-Host "🚀 Memulai proses rilis untuk versi $VersionTag..." -ForegroundColor Cyan

# 1. Add all changes
Write-Host "📦 Menambahkan perubahan (git add .)..."
git add .

# 2. Commit changes
Write-Host "📝 Membuat commit..."
git commit -m $CommitMessage

# 3. Create Tag
Write-Host "🏷️ Membuat tag versi $VersionTag..."
git tag $VersionTag

# 4. Push to main branch
Write-Host "☁️ Mendorong kode ke branch main (git push origin main)..."
git push origin main

# 5. Push tags
Write-Host "☁️ Mendorong tag ke GitHub..."
git push origin $VersionTag

Write-Host "✅ Selesai! Versi $VersionTag berhasil di-push ke GitHub." -ForegroundColor Green
