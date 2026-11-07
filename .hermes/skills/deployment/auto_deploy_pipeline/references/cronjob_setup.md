# Cronjob Setup

Create a daily scheduled job that runs the auto‑deploy pipeline at 03:00 UTC.

```yaml
cronjob:
  action: create
  job_id: auto_deploy_daily
  schedule: "0 3 * * *"
  script: auto_deploy.sh
  deliver: origin
  workdir: /home/casey/.hermes/scripts
```

- `action: create` registers a new job.  
- `schedule`: cron expression (UTC).  
- `script`: name of the script (`auto_deploy.sh`).  
- `deliver: origin` pushes the job definition back to the central GitHub repo.  
- `workdir`: directory containing the script.

After committing this YAML definition, the job will fire automatically each day at the specified time.