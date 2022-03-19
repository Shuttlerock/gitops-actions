## Auto Promote Action

### TODO

### Configuration

Example config.yaml file:
```
targets:
- repository: https://github.com/Shuttlerock/example.git
  branch: develop
  enabled: true
  rules:
    - fileName: variables_versions.tf
      variable: semver
      pattern:
        block: module
        labels: [creative_revision_event_consumer]
        attribute: version
```