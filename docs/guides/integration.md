# Integration Guide

Complete guide to integrating MUCM with CI/CD pipelines, static site generators, and development workflows.

## Overview

MUCM is designed to integrate seamlessly with modern development workflows. This guide covers integration patterns for continuous integration, documentation sites, testing frameworks, and development tools.

## CI/CD Integration

### GitHub Actions

#### Basic Validation Workflow

```yaml
# .github/workflows/use-case-validation.yml
name: Use Case Validation

on:
  push:
    branches: [ main, develop ]
    paths: [ 'docs/use-cases/**', '.config/.mucm/**' ]
  pull_request:
    branches: [ main ]
    paths: [ 'docs/use-cases/**', '.config/.mucm/**' ]

jobs:
  validate:
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout repository
      uses: actions/checkout@v4
      
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Install MUCM
      run: cargo install --git https://github.com/GuillaumeCoi/markdown-use-case-manager
      
    - name: Validate use cases
      run: |
        mucm status
        mucm validate --all
        
    - name: Check for broken references
      run: mucm validate --check-references
      
    - name: Generate status report
      run: |
        mucm status --format json > use-case-status.json
        mucm list --format json > use-case-list.json
        
    - name: Upload reports
      uses: actions/upload-artifact@v3
      with:
        name: use-case-reports
        path: |
          use-case-status.json
          use-case-list.json
```

#### Documentation Generation and Deployment

```yaml
# .github/workflows/docs-deploy.yml
name: Deploy Documentation

on:
  push:
    branches: [ main ]
  workflow_run:
    workflows: ["Use Case Validation"]
    types: [completed]

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    if: ${{ github.event.workflow_run.conclusion == 'success' }}
    
    steps:
    - name: Checkout repository
      uses: actions/checkout@v4
      
    - name: Install MUCM
      run: cargo install --git https://github.com/GuillaumeCoi/markdown-use-case-manager
      
    - name: Generate documentation
      run: |
        # Generate overview documentation
        mucm list --format markdown > docs/use-cases/index.md
        mucm status --detailed --format markdown > docs/project-status.md
        
        # Generate methodology documentation
        echo "# Use Case Methodologies" > docs/methodologies.md
        echo "This project uses: $(mucm config get templates.methodology)" >> docs/methodologies.md
        
    - name: Setup Node.js for static site
      uses: actions/setup-node@v3
      with:
        node-version: '18'
        
    - name: Install dependencies
      run: npm install
      
    - name: Build static site
      run: npm run build
      
    - name: Deploy to GitHub Pages
      uses: peaceiris/actions-gh-pages@v3
      with:
        github_token: ${{ secrets.GITHUB_TOKEN }}
        publish_dir: ./dist
```

### GitLab CI/CD

```yaml
# .gitlab-ci.yml
stages:
  - validate
  - test
  - build
  - deploy

variables:
  CARGO_HOME: $CI_PROJECT_DIR/cargo

cache:
  paths:
    - cargo/
    - target/

validate_use_cases:
  stage: validate
  image: rust:1.70
  before_script:
    - cargo install --git https://github.com/GuillaumeCoi/markdown-use-case-manager
  script:
    - mucm status
    - mucm validate --all
    - mucm status --format json > use-case-status.json
  artifacts:
    reports:
      junit: use-case-status.json
    paths:
      - use-case-status.json
  only:
    changes:
      - docs/use-cases/**/*
      - .config/.mucm/**/*

generate_docs:
  stage: build
  image: rust:1.70
  dependencies:
    - validate_use_cases
  before_script:
    - cargo install --git https://github.com/GuillaumeCoi/markdown-use-case-manager
  script:
    - mucm list --format markdown > docs/generated/use-case-index.md
    - mucm status --detailed > docs/generated/project-status.md
  artifacts:
    paths:
      - docs/generated/
  only:
    - main

deploy_docs:
  stage: deploy
  image: node:18
  dependencies:
    - generate_docs
  script:
    - npm install
    - npm run build
    - cp -r dist public
  artifacts:
    paths:
      - public
  only:
    - main
```

### Azure DevOps

```yaml
# azure-pipelines.yml
trigger:
  branches:
    include:
      - main
      - develop
  paths:
    include:
      - docs/use-cases/*
      - .config/.mucm/*

pool:
  vmImage: 'ubuntu-latest'

stages:
- stage: Validate
  displayName: 'Validate Use Cases'
  jobs:
  - job: ValidateUseCases
    displayName: 'Validate Use Cases'
    steps:
    - task: UseDotNet@2
      displayName: 'Install .NET SDK'
      inputs:
        packageType: 'sdk'
        version: '6.x'
        
    - script: |
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source $HOME/.cargo/env
        cargo install --git https://github.com/GuillaumeCoi/markdown-use-case-manager
      displayName: 'Install MUCM'
      
    - script: |
        export PATH=$HOME/.cargo/bin:$PATH
        mucm status
        mucm validate --all
      displayName: 'Validate Use Cases'
      
    - script: |
        export PATH=$HOME/.cargo/bin:$PATH
        mucm status --format json > $(Agent.TempDirectory)/use-case-status.json
        mucm list --format json > $(Agent.TempDirectory)/use-case-list.json
      displayName: 'Generate Reports'
      
    - task: PublishTestResults@2
      displayName: 'Publish Use Case Reports'
      inputs:
        testResultsFormat: 'JUnit'
        testResultsFiles: '$(Agent.TempDirectory)/use-case-*.json'
        mergeTestResults: true

- stage: Deploy
  displayName: 'Deploy Documentation'
  dependsOn: Validate
  condition: and(succeeded(), eq(variables['Build.SourceBranch'], 'refs/heads/main'))
  jobs:
  - job: DeployDocs
    displayName: 'Deploy Documentation'
    steps:
    - script: |
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source $HOME/.cargo/env
        cargo install --git https://github.com/GuillaumeCoi/markdown-use-case-manager
      displayName: 'Install MUCM'
      
    - script: |
        export PATH=$HOME/.cargo/bin:$PATH
        mucm list --format markdown > docs/use-cases/index.md
        mucm status --detailed > docs/project-status.md
      displayName: 'Generate Documentation'
      
    - task: AzureStaticWebApp@0
      inputs:
        app_location: 'docs'
        api_location: ''
        output_location: 'dist'
        azure_static_web_apps_api_token: $(deployment_token)
```

## Static Site Generators

### MkDocs

#### Configuration

```yaml
# mkdocs.yml
site_name: Project Documentation
site_description: Use case documentation and project status

theme:
  name: material
  features:
    - navigation.tabs
    - navigation.sections
    - navigation.expand
    - search.highlight

plugins:
  - search
  - git-revision-date-localized:
      type: date

nav:
  - Home: index.md
  - Use Cases:
    - Overview: use-cases/index.md
    - Authentication: 
      - use-cases/authentication/UC-AUTH-001.md
      - use-cases/authentication/UC-AUTH-002.md
    - Payment:
      - use-cases/payment/UC-PAY-001.md
      - use-cases/payment/UC-PAY-002.md
  - Project Status: project-status.md
  - Methodologies: methodologies.md

markdown_extensions:
  - admonition
  - codehilite
  - toc:
      permalink: true
  - pymdownx.superfences
  - pymdownx.tabbed
```

#### Build Script

```bash
#!/bin/bash
# scripts/build-docs.sh

set -e

echo "Generating use case documentation..."
mucm list --format markdown > docs/use-cases/index.md
mucm status --detailed --format markdown > docs/project-status.md

echo "Building MkDocs site..."
mkdocs build

echo "Documentation built successfully!"
```

#### Custom MkDocs Plugin

```python
# plugins/mucm_integration.py
from mkdocs.plugins import BasePlugin
from mkdocs.config import config_options
import subprocess
import json

class MucmIntegrationPlugin(BasePlugin):
    config_scheme = (
        ('mucm_command', config_options.Type(str, default='mucm')),
        ('auto_generate', config_options.Type(bool, default=True)),
    )

    def on_pre_build(self, config):
        if self.config['auto_generate']:
            # Generate use case index
            subprocess.run([
                self.config['mucm_command'], 
                'list', 
                '--format', 'markdown'
            ], 
            stdout=open('docs/use-cases/index.md', 'w'),
            check=True)
            
            # Generate project status
            subprocess.run([
                self.config['mucm_command'],
                'status',
                '--detailed',
                '--format', 'markdown'
            ],
            stdout=open('docs/project-status.md', 'w'),
            check=True)

    def on_page_markdown(self, markdown, page, config, files):
        # Add MUCM-specific processing
        if 'use-cases/' in page.file.src_path:
            # Add automatic navigation elements
            markdown += "\n\n[← Back to Use Cases](../index.md)"
        return markdown
```

### Docusaurus

#### Configuration

```javascript
// docusaurus.config.js
const lightCodeTheme = require('prism-react-renderer/themes/github');
const darkCodeTheme = require('prism-react-renderer/themes/dracula');

module.exports = {
  title: 'Project Documentation',
  tagline: 'Use case documentation and project guides',
  url: 'https://your-org.github.io',
  baseUrl: '/your-project/',
  
  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          editUrl: 'https://github.com/your-org/your-project/tree/main/',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      },
    ],
  ],

  plugins: [
    [
      '@docusaurus/plugin-content-docs',
      {
        id: 'use-cases',
        path: 'docs/use-cases',
        routeBasePath: 'use-cases',
        sidebarPath: require.resolve('./sidebars-use-cases.js'),
      },
    ],
  ],

  themeConfig: {
    navbar: {
      title: 'Project Docs',
      items: [
        {
          type: 'doc',
          docId: 'intro',
          position: 'left',
          label: 'Documentation',
        },
        {
          to: '/use-cases/',
          label: 'Use Cases',
          position: 'left',
        },
        {
          href: 'https://github.com/your-org/your-project',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    
    prism: {
      theme: lightCodeTheme,
      darkTheme: darkCodeTheme,
    },
  },
};
```

#### Sidebar Generation

```javascript
// scripts/generate-sidebars.js
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

function generateUseCaseSidebar() {
  // Get use case data from MUCM
  const useCaseData = JSON.parse(
    execSync('mucm list --format json', { encoding: 'utf8' })
  );
  
  const sidebar = {
    useCases: [
      {
        type: 'doc',
        id: 'index',
        label: 'Overview',
      },
    ],
  };
  
  // Group by category
  const categories = {};
  useCaseData.forEach(useCase => {
    if (!categories[useCase.category]) {
      categories[useCase.category] = [];
    }
    categories[useCase.category].push({
      type: 'doc',
      id: `${useCase.category.toLowerCase()}/${useCase.id}`,
      label: useCase.title,
    });
  });
  
  // Add categories to sidebar
  Object.keys(categories).forEach(category => {
    sidebar.useCases.push({
      type: 'category',
      label: category,
      items: categories[category],
    });
  });
  
  fs.writeFileSync(
    'sidebars-use-cases.js',
    `module.exports = ${JSON.stringify(sidebar, null, 2)};`
  );
}

generateUseCaseSidebar();
```

### GitBook

#### Configuration

```json
{
  "title": "Project Documentation",
  "description": "Use case documentation and project guides",
  "author": "Development Team",
  "language": "en",
  "gitbook": "3.2.3",
  "structure": {
    "readme": "README.md",
    "summary": "SUMMARY.md"
  },
  "plugins": [
    "include-codeblock",
    "anchors",
    "search-pro",
    "mucm-integration"
  ],
  "pluginsConfig": {
    "mucm-integration": {
      "autoGenerate": true,
      "summaryFile": "SUMMARY.md"
    }
  }
}
```

#### Summary Generation

```markdown
<!-- SUMMARY.md -->
# Summary

* [Introduction](README.md)

## Use Cases
<!-- Generated by MUCM plugin -->
* [Overview](use-cases/README.md)
* [Authentication](use-cases/authentication/README.md)
  * [UC-AUTH-001: User Login](use-cases/authentication/UC-AUTH-001.md)
  * [UC-AUTH-002: Password Reset](use-cases/authentication/UC-AUTH-002.md)
* [Payment](use-cases/payment/README.md)
  * [UC-PAY-001: Process Payment](use-cases/payment/UC-PAY-001.md)

## Project Information
* [Project Status](project-status.md)
* [Methodology Guide](methodology.md)
```

## Test Framework Integration

### Rust Integration

#### Test Generation

```rust
// tests/use_cases/authentication/uc_auth_001.rs
// Generated from UC-AUTH-001: User Login

use super::*;

#[tokio::test]
async fn test_user_login_main_flow() {
    // Test the main success scenario
    let mut app = test_app().await;
    
    // Given: User has valid credentials
    let user = create_test_user(&mut app).await;
    
    // When: User attempts to login
    let response = app
        .post("/api/auth/login")
        .json(&json!({
            "email": user.email,
            "password": "valid_password"
        }))
        .send()
        .await;
    
    // Then: Login should succeed
    assert_eq!(response.status(), 200);
    let body: LoginResponse = response.json().await;
    assert!(!body.token.is_empty());
    assert_eq!(body.user.email, user.email);
}

#[tokio::test] 
async fn test_user_login_invalid_credentials() {
    // Test alternative flow: Invalid credentials
    let mut app = test_app().await;
    
    // When: User provides invalid credentials
    let response = app
        .post("/api/auth/login")
        .json(&json!({
            "email": "invalid@example.com",
            "password": "wrong_password"
        }))
        .send()
        .await;
    
    // Then: Login should fail
    assert_eq!(response.status(), 401);
    let body: ErrorResponse = response.json().await;
    assert_eq!(body.error, "Invalid credentials");
}

#[tokio::test]
async fn test_user_login_account_locked() {
    // Test alternative flow: Account locked
    let mut app = test_app().await;
    let user = create_locked_user(&mut app).await;
    
    // When: Locked user attempts login
    let response = app
        .post("/api/auth/login")
        .json(&json!({
            "email": user.email,
            "password": "valid_password"
        }))
        .send()
        .await;
    
    // Then: Login should be blocked
    assert_eq!(response.status(), 423);
    let body: ErrorResponse = response.json().await;
    assert_eq!(body.error, "Account temporarily locked");
}
```

#### Custom Test Macros

```rust
// tests/macros.rs
macro_rules! use_case_test {
    ($name:ident, $use_case_id:expr, $scenario:expr, $test_fn:expr) => {
        #[tokio::test]
        async fn $name() {
            println!("Testing use case: {} - {}", $use_case_id, $scenario);
            $test_fn().await;
        }
    };
}

// Usage
use_case_test!(
    test_uc_auth_001_main_flow,
    "UC-AUTH-001",
    "User logs in with valid credentials",
    test_user_login_main_flow
);
```

### Python Integration

#### BDD Test Generation

```python
# tests/use_cases/authentication/test_uc_auth_001.py
# Generated from UC-AUTH-001: User Login

import pytest
from behave import given, when, then
from app.test_utils import TestClient, create_test_user

@pytest.fixture
def client():
    return TestClient()

@pytest.fixture
def test_user(client):
    return create_test_user(client, {
        'email': 'test@example.com',
        'password': 'valid_password'
    })

class TestUserLogin:
    """Test cases for UC-AUTH-001: User Login"""
    
    def test_main_flow_valid_credentials(self, client, test_user):
        """Test main success scenario"""
        response = client.post('/api/auth/login', json={
            'email': test_user.email,
            'password': 'valid_password'
        })
        
        assert response.status_code == 200
        assert 'token' in response.json()
        assert response.json()['user']['email'] == test_user.email
    
    def test_alternative_flow_invalid_credentials(self, client):
        """Test alternative flow: Invalid credentials"""
        response = client.post('/api/auth/login', json={
            'email': 'invalid@example.com',
            'password': 'wrong_password'
        })
        
        assert response.status_code == 401
        assert response.json()['error'] == 'Invalid credentials'
    
    def test_alternative_flow_account_locked(self, client):
        """Test alternative flow: Account locked"""
        locked_user = create_test_user(client, {
            'email': 'locked@example.com',
            'password': 'valid_password',
            'status': 'locked'
        })
        
        response = client.post('/api/auth/login', json={
            'email': locked_user.email,
            'password': 'valid_password'
        })
        
        assert response.status_code == 423
        assert 'locked' in response.json()['error'].lower()

# BDD-style tests with pytest-bdd
@scenario('features/authentication.feature', 'User logs in with valid credentials')
def test_user_login_valid():
    pass

@given('I am a registered user')
def registered_user(client):
    return create_test_user(client)

@when('I log in with valid credentials')
def login_with_valid_credentials(client, registered_user):
    client.context['response'] = client.post('/api/auth/login', json={
        'email': registered_user.email,
        'password': 'valid_password'
    })

@then('I should be authenticated')
def should_be_authenticated(client):
    response = client.context['response']
    assert response.status_code == 200
    assert 'token' in response.json()
```

### JavaScript/TypeScript Integration

#### Jest Test Generation

```javascript
// tests/use-cases/authentication/ucAuth001.test.js
// Generated from UC-AUTH-001: User Login

const request = require('supertest');
const app = require('../../../src/app');
const { createTestUser, cleanupDatabase } = require('../../utils/testHelpers');

describe('UC-AUTH-001: User Login', () => {
  let testUser;

  beforeEach(async () => {
    await cleanupDatabase();
    testUser = await createTestUser({
      email: 'test@example.com',
      password: 'validPassword123'
    });
  });

  afterEach(async () => {
    await cleanupDatabase();
  });

  describe('Main Flow: Valid Credentials', () => {
    it('should authenticate user with valid credentials', async () => {
      const response = await request(app)
        .post('/api/auth/login')
        .send({
          email: testUser.email,
          password: 'validPassword123'
        })
        .expect(200);

      expect(response.body).toHaveProperty('token');
      expect(response.body.user.email).toBe(testUser.email);
      expect(response.body.user).not.toHaveProperty('password');
    });
  });

  describe('Alternative Flows', () => {
    it('should reject invalid credentials', async () => {
      const response = await request(app)
        .post('/api/auth/login')
        .send({
          email: 'invalid@example.com',
          password: 'wrongPassword'
        })
        .expect(401);

      expect(response.body.error).toBe('Invalid credentials');
      expect(response.body).not.toHaveProperty('token');
    });

    it('should block login for locked accounts', async () => {
      // Lock the user account
      await request(app)
        .patch(`/api/admin/users/${testUser.id}`)
        .send({ status: 'locked' })
        .expect(200);

      const response = await request(app)
        .post('/api/auth/login')
        .send({
          email: testUser.email,
          password: 'validPassword123'
        })
        .expect(423);

      expect(response.body.error).toContain('locked');
    });
  });

  describe('Error Scenarios', () => {
    it('should handle missing email', async () => {
      const response = await request(app)
        .post('/api/auth/login')
        .send({
          password: 'validPassword123'
        })
        .expect(400);

      expect(response.body.error).toContain('email');
    });

    it('should handle missing password', async () => {
      const response = await request(app)
        .post('/api/auth/login')
        .send({
          email: testUser.email
        })
        .expect(400);

      expect(response.body.error).toContain('password');
    });
  });
});
```

## IDE Integration

### VS Code Extension

#### Extension Configuration

```json
{
  "name": "mucm-vscode",
  "displayName": "MUCM Integration",
  "description": "VS Code integration for Markdown Use Case Manager",
  "version": "0.1.0",
  "engines": {
    "vscode": "^1.74.0"
  },
  "categories": ["Other"],
  "activationEvents": [
    "onLanguage:markdown",
    "workspaceContains:.config/.mucm/mucm.toml"
  ],
  "main": "./out/extension.js",
  "contributes": {
    "commands": [
      {
        "command": "mucm.createUseCase",
        "title": "Create Use Case",
        "category": "MUCM"
      },
      {
        "command": "mucm.validateUseCases",
        "title": "Validate Use Cases",
        "category": "MUCM"
      },
      {
        "command": "mucm.showStatus",
        "title": "Show Project Status",
        "category": "MUCM"
      }
    ],
    "menus": {
      "explorer/context": [
        {
          "when": "resourcePath =~ /use-cases/",
          "command": "mucm.createUseCase",
          "group": "mucm"
        }
      ]
    },
    "configuration": {
      "title": "MUCM",
      "properties": {
        "mucm.executable": {
          "type": "string",
          "default": "mucm",
          "description": "Path to MUCM executable"
        },
        "mucm.autoValidate": {
          "type": "boolean",
          "default": true,
          "description": "Automatically validate use cases on save"
        }
      }
    }
  }
}
```

#### Extension Implementation

```typescript
// src/extension.ts
import * as vscode from 'vscode';
import { exec } from 'child_process';
import * as path from 'path';

export function activate(context: vscode.ExtensionContext) {
  // Register commands
  const createUseCaseCommand = vscode.commands.registerCommand(
    'mucm.createUseCase',
    async () => {
      const title = await vscode.window.showInputBox({
        prompt: 'Enter use case title',
        validateInput: (value) => {
          return value.length > 0 ? null : 'Title cannot be empty';
        }
      });

      if (title) {
        const category = await vscode.window.showInputBox({
          prompt: 'Enter category (optional)'
        });

        const command = category 
          ? `mucm create "${title}" --category "${category}"`
          : `mucm create "${title}"`;

        execMucmCommand(command);
      }
    }
  );

  const validateCommand = vscode.commands.registerCommand(
    'mucm.validateUseCases',
    () => {
      execMucmCommand('mucm validate --all');
    }
  );

  const statusCommand = vscode.commands.registerCommand(
    'mucm.showStatus',
    async () => {
      exec('mucm status --format json', (error, stdout, stderr) => {
        if (error) {
          vscode.window.showErrorMessage(`MUCM Error: ${error.message}`);
          return;
        }

        try {
          const status = JSON.parse(stdout);
          const panel = vscode.window.createWebviewPanel(
            'mucmStatus',
            'MUCM Project Status',
            vscode.ViewColumn.One,
            { enableScripts: true }
          );

          panel.webview.html = generateStatusHtml(status);
        } catch (parseError) {
          vscode.window.showErrorMessage('Failed to parse status data');
        }
      });
    }
  );

  context.subscriptions.push(
    createUseCaseCommand,
    validateCommand,
    statusCommand
  );

  // Auto-validation on save
  const config = vscode.workspace.getConfiguration('mucm');
  if (config.get('autoValidate')) {
    vscode.workspace.onDidSaveTextDocument((document) => {
      if (document.fileName.includes('use-cases') && 
          document.fileName.endsWith('.md')) {
        execMucmCommand('mucm validate --all');
      }
    });
  }
}

function execMucmCommand(command: string) {
  const terminal = vscode.window.createTerminal('MUCM');
  terminal.sendText(command);
  terminal.show();
}

function generateStatusHtml(status: any): string {
  return `
    <!DOCTYPE html>
    <html>
    <head>
      <title>MUCM Status</title>
      <style>
        body { font-family: var(--vscode-font-family); }
        .status-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 20px; }
        .status-card { border: 1px solid var(--vscode-panel-border); padding: 15px; }
        .metric { font-size: 24px; font-weight: bold; color: var(--vscode-charts-blue); }
      </style>
    </head>
    <body>
      <h1>Project Status</h1>
      <div class="status-grid">
        <div class="status-card">
          <h3>Total Use Cases</h3>
          <div class="metric">${status.totalUseCases}</div>
        </div>
        <div class="status-card">
          <h3>Completed</h3>
          <div class="metric">${status.completedUseCases}</div>
        </div>
      </div>
    </body>
    </html>
  `;
}
```

### IntelliJ IDEA Plugin

#### Plugin Configuration

```xml
<!-- plugin.xml -->
<idea-plugin>
  <id>com.example.mucm</id>
  <name>MUCM Integration</name>
  <version>1.0</version>
  <vendor>Your Company</vendor>

  <description><![CDATA[
    Integration plugin for Markdown Use Case Manager (MUCM)
  ]]></description>

  <depends>com.intellij.modules.platform</depends>

  <extensions defaultExtensionNS="com.intellij">
    <toolWindow id="MUCM" secondary="true" anchor="right" 
                factoryClass="com.example.mucm.MucmToolWindowFactory"/>
    <projectService serviceImplementation="com.example.mucm.MucmService"/>
  </extensions>

  <actions>
    <group id="MucmActionGroup" text="MUCM" description="MUCM actions">
      <add-to-group group-id="ToolsMenu" anchor="last"/>
      <action id="CreateUseCase" class="com.example.mucm.CreateUseCaseAction" 
              text="Create Use Case" description="Create a new use case"/>
      <action id="ValidateUseCases" class="com.example.mucm.ValidateUseCasesAction"
              text="Validate Use Cases" description="Validate all use cases"/>
    </group>
  </actions>
</idea-plugin>
```

## Database Integration

### Metadata Storage

```sql
-- PostgreSQL schema for use case metadata
CREATE TABLE use_case_metadata (
    id VARCHAR(50) PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    category VARCHAR(100) NOT NULL,
    priority VARCHAR(20) DEFAULT 'medium',
    status VARCHAR(20) DEFAULT 'planned',
    methodology VARCHAR(50) NOT NULL,
    author VARCHAR(100),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    file_path VARCHAR(500) NOT NULL,
    file_hash VARCHAR(64) NOT NULL
);

CREATE TABLE use_case_scenarios (
    id VARCHAR(50) PRIMARY KEY,
    use_case_id VARCHAR(50) REFERENCES use_case_metadata(id),
    title VARCHAR(255) NOT NULL,
    priority VARCHAR(20) DEFAULT 'medium',
    status VARCHAR(20) DEFAULT 'planned',
    step_count INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE use_case_relationships (
    source_id VARCHAR(50) REFERENCES use_case_metadata(id),
    target_id VARCHAR(50) REFERENCES use_case_metadata(id),
    relationship_type VARCHAR(50) NOT NULL, -- 'includes', 'extends', 'depends_on'
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (source_id, target_id, relationship_type)
);

-- Indexes for performance
CREATE INDEX idx_use_case_category ON use_case_metadata(category);
CREATE INDEX idx_use_case_status ON use_case_metadata(status);
CREATE INDEX idx_use_case_methodology ON use_case_metadata(methodology);
CREATE INDEX idx_scenario_use_case ON use_case_scenarios(use_case_id);
```

### Sync Script

```python
#!/usr/bin/env python3
# scripts/sync_metadata.py

import json
import subprocess
import psycopg2
import hashlib
from pathlib import Path

def get_mucm_data():
    """Get use case data from MUCM"""
    result = subprocess.run(
        ['mucm', 'list', '--format', 'json'],
        capture_output=True,
        text=True,
        check=True
    )
    return json.loads(result.stdout)

def calculate_file_hash(file_path):
    """Calculate MD5 hash of file content"""
    with open(file_path, 'rb') as f:
        return hashlib.md5(f.read()).hexdigest()

def sync_to_database(use_cases, conn):
    """Sync use case data to database"""
    with conn.cursor() as cur:
        for uc in use_cases:
            file_hash = calculate_file_hash(uc['file_path'])
            
            # Insert or update use case metadata
            cur.execute("""
                INSERT INTO use_case_metadata 
                (id, title, category, priority, status, methodology, author, file_path, file_hash)
                VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s)
                ON CONFLICT (id) DO UPDATE SET
                    title = EXCLUDED.title,
                    category = EXCLUDED.category,
                    priority = EXCLUDED.priority,
                    status = EXCLUDED.status,
                    file_hash = EXCLUDED.file_hash,
                    updated_at = CURRENT_TIMESTAMP
            """, (
                uc['id'], uc['title'], uc['category'], 
                uc['priority'], uc['status'], uc['methodology'],
                uc.get('author'), uc['file_path'], file_hash
            ))
            
            # Sync scenarios
            for scenario in uc.get('scenarios', []):
                cur.execute("""
                    INSERT INTO use_case_scenarios
                    (id, use_case_id, title, priority, status, step_count)
                    VALUES (%s, %s, %s, %s, %s, %s)
                    ON CONFLICT (id) DO UPDATE SET
                        title = EXCLUDED.title,
                        priority = EXCLUDED.priority,
                        status = EXCLUDED.status,
                        step_count = EXCLUDED.step_count,
                        updated_at = CURRENT_TIMESTAMP
                """, (
                    scenario['id'], uc['id'], scenario['title'],
                    scenario['priority'], scenario['status'],
                    len(scenario.get('steps', []))
                ))
        
        conn.commit()

if __name__ == '__main__':
    # Database connection
    conn = psycopg2.connect(
        host='localhost',
        database='project_db',
        user='db_user',
        password='db_password'
    )
    
    try:
        use_cases = get_mucm_data()
        sync_to_database(use_cases, conn)
        print(f"Synced {len(use_cases)} use cases to database")
    finally:
        conn.close()
```

## Monitoring and Analytics

### Prometheus Metrics

```python
# scripts/export_metrics.py
from prometheus_client import CollectorRegistry, Gauge, push_to_gateway
import subprocess
import json

def collect_mucm_metrics():
    """Collect MUCM metrics for Prometheus"""
    registry = CollectorRegistry()
    
    # Define metrics
    total_use_cases = Gauge(
        'mucm_use_cases_total',
        'Total number of use cases',
        registry=registry
    )
    
    use_cases_by_status = Gauge(
        'mucm_use_cases_by_status',
        'Number of use cases by status',
        ['status'],
        registry=registry
    )
    
    use_cases_by_category = Gauge(
        'mucm_use_cases_by_category', 
        'Number of use cases by category',
        ['category'],
        registry=registry
    )
    
    # Get data from MUCM
    result = subprocess.run(
        ['mucm', 'status', '--format', 'json'],
        capture_output=True,
        text=True,
        check=True
    )
    
    data = json.loads(result.stdout)
    
    # Set metrics
    total_use_cases.set(data['total_use_cases'])
    
    for status, count in data['by_status'].items():
        use_cases_by_status.labels(status=status).set(count)
    
    for category, count in data['by_category'].items():
        use_cases_by_category.labels(category=category).set(count)
    
    # Push to Prometheus gateway
    push_to_gateway(
        'localhost:9091',
        job='mucm_metrics',
        registry=registry
    )

if __name__ == '__main__':
    collect_mucm_metrics()
```

### Grafana Dashboard

```json
{
  "dashboard": {
    "title": "MUCM Project Metrics",
    "panels": [
      {
        "title": "Total Use Cases",
        "type": "stat",
        "targets": [
          {
            "expr": "mucm_use_cases_total",
            "legendFormat": "Total"
          }
        ]
      },
      {
        "title": "Use Cases by Status",
        "type": "piechart",
        "targets": [
          {
            "expr": "mucm_use_cases_by_status",
            "legendFormat": "{{status}}"
          }
        ]
      },
      {
        "title": "Use Cases by Category",
        "type": "bargraph",
        "targets": [
          {
            "expr": "mucm_use_cases_by_category",
            "legendFormat": "{{category}}"
          }
        ]
      }
    ]
  }
}
```

This integration guide provides comprehensive coverage of how to integrate MUCM with various tools and workflows. Each integration pattern is designed to enhance your development process while maintaining the benefits of methodology-aware use case documentation.