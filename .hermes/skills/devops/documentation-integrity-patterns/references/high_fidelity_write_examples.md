# High-Fidelity Shell Injection Templates

Use these templates to ensure data integrity during manual documentation rewrites.

## Template 1: The "Nuclear Rewrite" (Multi-line Markdown)
Best for: `README.md`, `A2A_PROTOCOLS.md`, `INSTALLATION.md`.

```bash
cat << 'EOF' > /absolute/path/to/target.md
# [Title]

[Content]
EOF
```
*Note: The single quotes around `'EOF'` are CRITICAL. They prevent the shell from attempting to expand variables like `$HOME` or `$PATH` within your documentation.*

## Template 2: The "Precise Injection" (Single Line/Variable)
Best for: Configuration files or small metadata updates.

```bash
printf "key=\"%s\"\n" "value" > /absolute/path/to/config.env
```

## Template 3: The "Byte-Verification Routine"
Always run this immediately after a write.

```bash
ls -la /absolute/path/to/target.md && cat /absolute/path/to/target.md && wc -c /absolute/path/to/target.md
```
