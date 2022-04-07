# Auto Promote Action

This action can be used to automatically promote software versions in remote repositories based on rules defined in a config file.

## Inputs

### `user`
**Required** - The User to use for accessing target repositories.

### `email`
**Required** - Email to use in signature.

### `password`
Password or access token for authenticating with target repositories.

### `config`
**Required** - Path to config file.

### `variables`
**Required** - Space separate list of variable / value pairs in the format of `x=y`.

### Configuration

#### Example usage:

```
- uses: actions/checkout@v3
- uses: Shuttlerock/gitops-actions/auto-promote-action@v1.9
  with:
    user: spielbot
    email: spielbot@shuttlerock.com
    password: ${{ secrets.GITHUB_TOKEN }}
    config: auto-promote.yml
    variables: semver=7.6.7
```

#### Example auto-promote.yml file:
```
targets:
- repository: https://github.com/Shuttlerock/example.git
  branch: develop
  enabled: true
  rules:
    - filePattern: '**/*.tf'
      variable: semver
      pattern:
        block: module
        labels: [test_resource]
        attributes:
          source: app.terraform.io/shuttlerock/creative-revision/shuttlerock//modules/event-consumer
        targetAttribute: version
```