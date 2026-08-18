#!/bin/bash

# Memeriksa apakah argumen yang diberikan cukup
if [ "$#" -ne 2 ]; then
    echo "Penggunaan: ./deploy.sh \"Pesan Commit\" \"vX.X.X\""
    exit 1
fi

COMMIT_MESSAGE=$1
VERSION_TAG=$2

echo "🚀 Memulai proses rilis untuk versi $VERSION_TAG..."

echo "📦 Menambahkan perubahan (git add .)..."
git add .

echo "📝 Membuat commit..."
git commit -m "$COMMIT_MESSAGE"

echo "🏷️ Membuat tag versi $VERSION_TAG..."
git tag "$VERSION_TAG"

echo "☁️ Mendorong kode ke branch main (git push origin main)..."
git push origin main

echo "☁️ Mendorong tag ke GitHub..."
git push origin "$VERSION_TAG"

echo "✅ Selesai! Versi $VERSION_TAG berhasil di-push ke GitHub."
