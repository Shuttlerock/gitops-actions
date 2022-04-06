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
    - filePattern: '**/*.tf'
      variable: semver
      pattern:
        block: module
        labels: [test_resource]
        attributes:
          source: app.terraform.io/shuttlerock/creative-revision/shuttlerock//modules/event-consumer
        targetAttribute: version
```