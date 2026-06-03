## ADDED Requirements

### Requirement: User registration via email and password
The system SHALL allow new users to register with an email address and password. Email addresses SHALL be unique. Passwords SHALL be hashed before storage (handled by BetterAuth). A verification email SHALL be sent upon registration.

#### Scenario: Successful registration
- **WHEN** a user submits a valid email and password not already in use
- **THEN** an account is created, a verification email is sent, and the user is redirected to a post-signup screen

#### Scenario: Duplicate email
- **WHEN** a user attempts to register with an email already associated with an account
- **THEN** the system returns a validation error and does not create a duplicate account

#### Scenario: Weak password
- **WHEN** a user submits a password shorter than 8 characters
- **THEN** the system returns a validation error and does not create the account

---

### Requirement: Email verification
The system SHALL require email verification before a user can access authenticated features. An unverified user MAY log in but SHALL be prompted to verify their email.

#### Scenario: Verification link clicked
- **WHEN** a user clicks the verification link in their email
- **THEN** their email is marked verified and they are redirected to the dashboard

#### Scenario: Unverified user accesses dashboard
- **WHEN** an unverified user navigates to the dashboard
- **THEN** they are shown a prompt to verify their email with an option to resend the verification email

---

### Requirement: User login via email and password
The system SHALL authenticate users with their registered email and password and establish a server-side session.

#### Scenario: Successful login
- **WHEN** a user submits correct credentials
- **THEN** a session is created and the user is redirected to the dashboard

#### Scenario: Invalid credentials
- **WHEN** a user submits an incorrect email or password
- **THEN** the system returns a generic error ("Invalid email or password") without indicating which field is wrong

---

### Requirement: Session management
Sessions SHALL be managed server-side via BetterAuth. Sessions SHALL expire after a configurable idle period (default 30 days). The session cookie SHALL be HttpOnly and Secure.

#### Scenario: Active session on navigation
- **WHEN** a user with a valid session navigates to the dashboard
- **THEN** they are granted access without re-authentication

#### Scenario: Expired session
- **WHEN** a user's session has expired
- **THEN** they are redirected to the login page

---

### Requirement: Password reset via email
The system SHALL allow users to reset their password via a time-limited link sent to their registered email address.

#### Scenario: Password reset requested
- **WHEN** a user submits their email on the forgot-password page
- **THEN** a reset link is emailed to them (the UI shows a confirmation regardless of whether the email exists, to prevent user enumeration)

#### Scenario: Reset link used
- **WHEN** a user clicks a valid, unexpired reset link and submits a new password
- **THEN** their password is updated, existing sessions are invalidated, and they are redirected to login

#### Scenario: Expired reset link
- **WHEN** a user clicks a reset link older than 1 hour
- **THEN** they are shown an error and prompted to request a new link

---

### Requirement: Logout
The system SHALL allow authenticated users to terminate their session.

#### Scenario: User logs out
- **WHEN** a user clicks "Log out"
- **THEN** the server-side session is invalidated and the user is redirected to the login page
