const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

function runCmd(cmd, cwd) {
  try {
    return execSync(cmd, { cwd, encoding: 'utf8', stdio: 'pipe' }).trim();
  } catch (err) {
    return null;
  }
}

async function main() {
  console.log('=== Waras (WhatsApp Bot) Git Commit & Push ===\n');
  const dir = __dirname;
  
  console.log('Checking Waras...');
  const status = runCmd('git status --porcelain', dir);
  
  if (status) {
    console.log('Changes detected in Waras!');
    const cargoTomlPath = path.join(dir, 'Cargo.toml');
    const cargoToml = fs.readFileSync(cargoTomlPath, 'utf8');
    
    // Parse version from Cargo.toml
    const versionMatch = cargoToml.match(/version\s*=\s*"([^"]+)"/);
    let currentVer = versionMatch ? versionMatch[1] : '0.1.0';
    console.log(`Current version in Cargo.toml: ${currentVer}`);
    
    // Increment version
    const parts = currentVer.split('.');
    if (parts.length === 3) {
      parts[2] = parseInt(parts[2], 10) + 1;
    }
    const nextVer = parts.join('.');
    console.log(`Next version will be: ${nextVer}`);
    
    // Update Cargo.toml
    const newCargoToml = cargoToml.replace(/(version\s*=\s*")([^"]+)(")/, `$1${nextVer}$3`);
    fs.writeFileSync(cargoTomlPath, newCargoToml, 'utf8');
    
    let msg = process.argv[2] || 'fix: update webhook payload handling to use raw body';
    
    console.log('Staging and committing changes...');
    execSync('git add .', { cwd: dir });
    execSync(`git commit -m "${msg}"`, { cwd: dir });
    
    const nextTag = `v${nextVer}`;
    console.log(`Creating tag: ${nextTag}`);
    execSync(`git tag ${nextTag}`, { cwd: dir });
    
    console.log('Pushing to origin main...');
    try {
      execSync('git push origin main', { cwd: dir, stdio: 'inherit' });
      execSync(`git push origin ${nextTag}`, { cwd: dir, stdio: 'inherit' });
      console.log('Push completed successfully!\n');
    } catch (e) {
      console.log('Push failed. Is origin configured?');
    }
  } else {
    console.log('No changes detected.\n');
  }
  
  console.log('=== Done ===');
}

main().catch(err => {
  console.error(err);
});
