# Security Policy

This document outlines the security policy for Ratatui, including supported versions, how to report vulnerabilities, and our commitment to security.

## Supported Versions

We release patches for security vulnerabilities for the current version(s).

**Note:** We generally support the current major/minor release and the previous minor release. Users are encouraged to update to the latest version to receive security updates.

## Reporting a Vulnerability

We take the security of Ratatui seriously. If you have discovered a security vulnerability in Ratatui, we appreciate your help in disclosing it to us in a responsible manner.

### How to Report

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report security vulnerabilities by emailing the maintainers at:

- Email: security@ratatui.rs (if available) or directly to the [maintainers](https://github.com/orgs/ratatui-org/people)
- Alternatively, you can use GitHub's private vulnerability reporting feature:
  1. Navigate to the [Security Advisories](https://github.com/ratatui-org/ratatui/security/advisories) page
  2. Click "Report a vulnerability"
  3. Fill in the details

### What to Include

Please include the following information in your report:

- Type of vulnerability (e.g., buffer overflow, injection, authentication bypass, etc.)
- Full paths of source file(s) related to the manifestation of the vulnerability
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the vulnerability, including how an attacker might exploit it

### Response Timeline

- **Initial Response:** We will acknowledge your report within 48 hours
- **Status Update:** We will send a more detailed response within 7 days, indicating the next steps in handling your report
- **Fix Timeline:** We aim to release a security patch within 30 days for critical vulnerabilities, and within 90 days for other vulnerabilities
- **Disclosure:** We will coordinate with you on the disclosure timeline

### Safe Harbor

We support safe harbor for security researchers who:

- Make a good faith effort to avoid privacy violations, destruction of data, and interruption or degradation of our services
- Only interact with accounts you own or with explicit permission of the account holder
- Do not exploit a security issue you discover for any reason (including to demonstrate additional risk)
- Report vulnerabilities as soon as you discover them

We will not take legal action against researchers who follow these guidelines.

## Security Update Process

When we receive a security vulnerability report:

1. **Confirmation:** We will confirm the problem and determine the affected versions
2. **Fix Development:** We will develop a fix and prepare a release
3. **Testing:** The fix will be tested to ensure it resolves the issue without introducing new problems
4. **Release:** We will release a new version with the security fix
5. **Disclosure:** We will publish a security advisory on GitHub and update the CHANGELOG

## Disclosure Policy

- We will coordinate with the reporter on the disclosure timeline
- We prefer to fully disclose vulnerabilities once a fix is available
- Security advisories will be published on our [GitHub Security Advisories](https://github.com/ratatui-org/ratatui/security/advisories) page
- Critical vulnerabilities will be announced through our communication channels (Discord, Matrix, GitHub)

## Security Best Practices for Users

When using Ratatui in your applications, consider the following security best practices:

### Input Validation

- **Validate all user input:** Even though Ratatui is a UI library, ensure that any user input processed through your application is properly validated and sanitized
- **Limit input lengths:** Be cautious with unbounded input that could lead to memory exhaustion
- **Handle escape sequences carefully:** Be aware of terminal escape sequences in user-provided text that could affect terminal behavior

### Dependencies

- **Keep dependencies updated:** Regularly update Ratatui and all dependencies to receive security patches
- **Audit dependencies:** Use `cargo audit` to check for known vulnerabilities in your dependency tree
- **Minimize dependencies:** Only include necessary features to reduce your attack surface

### Terminal Security

- **Sanitize terminal output:** Be cautious when rendering untrusted content to the terminal
- **Escape special characters:** Properly escape any special characters that could be interpreted as terminal control sequences
- **Consider terminal emulator security:** Be aware that different terminal emulators may have different security characteristics

### Error Handling

- **Don't expose sensitive information:** Ensure error messages don't leak sensitive information
- **Handle panics gracefully:** Use proper error handling to prevent panics that could crash your application

## Security-Related Development Practices

Our development process includes several security-focused practices:

### Code Review

- All code changes go through pull request review
- Security-sensitive changes receive additional scrutiny
- We use automated tools to check for common security issues

### Static Analysis

- We use Clippy with strict lints to catch potential issues
- We run `cargo deny` to check for security advisories in dependencies
- We maintain a policy of zero unsafe code (`unsafe_code = "forbid"`)

### Testing

- We maintain high test coverage to catch regressions
- Security-relevant functionality receives dedicated tests
- We test with multiple Rust versions including the MSRV

### Continuous Integration

- All changes are tested through GitHub Actions
- We use Dependabot to keep dependencies updated
- We run security scans on dependencies

## Known Security Considerations

### Terminal Emulator Vulnerabilities

Ratatui operates at the terminal level and inherits any security characteristics of the terminal emulator being used. Users should:

- Use up-to-date terminal emulators
- Be aware of terminal emulator-specific vulnerabilities
- Understand that terminal escape sequences can affect terminal behavior

### Resource Consumption

While Ratatui is designed to be efficient:

- Rendering very large amounts of text or complex layouts could consume significant memory
- Applications should implement appropriate limits on rendering complexity
- Consider implementing rate limiting for UI updates if processing untrusted input

## Acknowledgments

We appreciate the security researchers and community members who help keep Ratatui secure. Contributors who report valid security issues may be acknowledged in our security advisories (with permission).

## Contact

- **General inquiries:** Open an issue on [GitHub](https://github.com/ratatui-org/ratatui/issues)
- **Security issues:** Use the reporting methods described above
- **Community:** Join our [Discord](https://discord.gg/pMCEU9hNEj) or [Matrix](https://matrix.to/#/#ratatui:matrix.org)

## Changes to This Policy

This security policy may be updated from time to time. We will notify users of significant changes through our normal communication channels.

---

**Last Updated:** February 2026
