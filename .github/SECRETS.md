# GitHub Secrets Configuration

This file documents the secrets that need to be configured in the GitHub repository settings.

## Required Secrets

### `PYPI_API_TOKEN`
- **Purpose**: PyPI API token for publishing packages
- **How to get**: 
  1. Go to https://pypi.org/manage/account/token/
  2. Create a new API token
  3. Copy the token value
- **Where to set**: Repository Settings → Secrets and variables → Actions
- **Used by**: `.github/workflows/publish.yml`

### `CODECOV_TOKEN` (Optional)
- **Purpose**: Code coverage reporting
- **How to get**: 
  1. Sign up at https://codecov.io
  2. Connect your GitHub repository
  3. Get the token from the repository settings
- **Where to set**: Repository Settings → Secrets and variables → Actions
- **Used by**: `.github/workflows/ci.yml`

## Setting Up Secrets

1. Go to your GitHub repository
2. Click on "Settings" tab
3. In the left sidebar, click "Secrets and variables" → "Actions"
4. Click "New repository secret"
5. Add each secret with the appropriate name and value

## Security Notes

- Never commit secrets to the repository
- Use environment variables for local development
- Rotate tokens regularly
- Use least-privilege access for tokens

## Local Development

For local development, create a `.env` file (not committed to git):

```bash
# Copy from env.example
cp env.example .env

# Edit .env with your values
PYPI_USERNAME=your_username
PYPI_PASSWORD=your_password
```

## Testing Secrets

You can test if secrets are working by:

1. Creating a test workflow
2. Using the `secrets` context in GitHub Actions
3. Checking the Actions logs (secrets are masked in logs)
