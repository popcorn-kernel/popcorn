# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

To report a vulnerability, please open a new issue and tell us as much info as you can.

1. Explain the vulnerability.
2. Tell us how we can recreate the vulnerability.
3. Tell us the machine you tested/found the vulnerability on.
4. If you have ideas of how to fix this issue, please share it with us.

For example:
Title: able to run administrator commands from a non-root shell

Body:
If you were to use a non-root shell and try to run root only commands, you will succeed.
I have found the issue while trying to change the password for one of my users on the system and forgot to change to a root shell.

To fix the issue we need to limit what commands the normal user can use.


## Accepting And Declining Vulnerabilities
Please act as professional as possible for us to accept your vulnerability report. Troll reports will be ignored.
How urgent is the vulnerability is also a major factor. If we see that the vulnerability is not major as other things, we will put it on hold.
