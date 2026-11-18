-- Umuryango PostgreSQL Database Schema Migration
-- Generated from schema.prisma

-- ENUM TYPES
DO $$ BEGIN
    CREATE TYPE "UserStatus" AS ENUM ('ACTIVE', 'DISABLED');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "AuthAuditAction" AS ENUM (
        'LOGIN', 'LOGOUT', 'REFRESH_TOKEN', 'REGISTER',
        'EMAIL_VERIFICATION', 'PASSWORD_RESET', 'PROFILE_UPDATE'
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "AuthAuditResult" AS ENUM ('SUCCESS', 'FAILURE');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "FamilyType" AS ENUM ('NUCLEAR', 'JOINT_EXTENDED', 'SINGLE_PARENT', 'BLENDED', 'ADOPTIVE');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "FamilyAuthority" AS ENUM ('HEAD', 'ADMIN', 'MEMBER');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "FamilyCapability" AS ENUM (
        'MANAGE_FAMILY_SETTINGS', 'MANAGE_MEMBERS', 'MANAGE_CONTACTS', 'MANAGE_SAVINGS',
        'MANAGE_CONTRIBUTIONS', 'RECORD_INCOME', 'RECORD_EXPENSE', 'APPROVE_EXPENSE',
        'MANAGE_DEBTS', 'MANAGE_PROJECTS', 'MANAGE_EVENTS', 'MANAGE_ANNOUNCEMENTS',
        'MANAGE_DOCUMENTS', 'VIEW_REPORTS', 'EXPORT_REPORTS'
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "FamilyKinship" AS ENUM (
        'SON', 'DAUGHTER', 'FATHER', 'MOTHER', 'BROTHER', 'SISTER', 'UNCLE', 'AUNT',
        'GRANDFATHER', 'GRANDMOTHER', 'COUSIN', 'IN_LAW', 'FAMILY_FRIEND', 'NEIGHBOR',
        'CLEANER', 'OTHER'
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "FamilyAuditAction" AS ENUM (
        'MEMBER_INVITED', 'MEMBER_REMOVED', 'AUTHORITY_CHANGED', 'CAPABILITIES_CHANGED',
        'HEAD_GRANTED', 'SETTINGS_CHANGED', 'CONTACT_CREATED', 'CONTACT_UPDATED',
        'CONTACT_DELETED', 'CONTACT_IMPORTED', 'CONTRIBUTION_PLAN_CREATED',
        'CONTRIBUTION_RECORDED', 'CONTRIBUTION_VERIFIED', 'CONTRIBUTION_REVERSED',
        'PROJECT_CREATED', 'PROJECT_UPDATED', 'PROJECT_CONTRIBUTION_RECORDED',
        'PROJECT_UPDATE_CREATED', 'TRANSACTION_CREATED', 'TRANSACTION_UPDATED',
        'TRANSACTION_APPROVED', 'TRANSACTION_REJECTED', 'TRANSACTION_REVERSED',
        'CATEGORY_CREATED', 'CATEGORY_UPDATED', 'CATEGORY_ARCHIVED',
        'INVITATION_ACCEPTED', 'INVITATION_DECLINED', 'RECEIPT_UPLOADED',
        'SYNC_MUTATION_APPLIED', 'SYNC_MUTATION_REJECTED'
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "ContributionCadence" AS ENUM ('WEEKLY', 'BIWEEKLY', 'MONTHLY', 'QUARTERLY', 'YEARLY', 'ONE_TIME');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "ContributionPlanStatus" AS ENUM ('ACTIVE', 'PAUSED', 'ARCHIVED');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "ContributionCycleStatus" AS ENUM ('OPEN', 'COMPLETED', 'OVERDUE');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "ContributionPaymentMethod" AS ENUM ('CASH', 'MOBILE_MONEY', 'BANK_TRANSFER', 'CARD', 'OTHER');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "ContributionPaymentStatus" AS ENUM ('PENDING', 'VERIFIED', 'REJECTED', 'REVERSED');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "TransactionType" AS ENUM ('INCOME', 'EXPENSE');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "TransactionScope" AS ENUM ('FAMILY', 'PERSONAL');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "CategoryStatus" AS ENUM ('ACTIVE', 'INACTIVE');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "TransactionStatus" AS ENUM ('PENDING', 'APPROVED', 'REJECTED', 'REVERSED');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "IncomeRecurrence" AS ENUM ('ONE_TIME', 'MONTHLY');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "IncomePaymentStatus" AS ENUM ('NOT_EXPECTED', 'UNPAID', 'PARTIAL', 'PAID', 'OVERPAID');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "LoanType" AS ENUM ('BORROWED', 'LENT');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "LoanInterestType" AS ENUM ('NONE', 'FIXED', 'PERCENTAGE');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "LoanRepaymentFrequency" AS ENUM ('ONCE', 'WEEKLY', 'BIWEEKLY', 'MONTHLY', 'QUARTERLY', 'YEARLY', 'CUSTOM');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "LoanStatus" AS ENUM ('DRAFT', 'ACTIVE', 'OVERDUE', 'COMPLETED', 'CANCELLED');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "LoanApprovalStatus" AS ENUM ('DRAFT', 'PENDING', 'APPROVED', 'REJECTED', 'CANCELLED');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "LoanRepaymentMethod" AS ENUM ('CASH', 'MOBILE_MONEY', 'BANK_TRANSFER', 'CARD', 'OTHER');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "LoanAuditAction" AS ENUM (
        'CREATED', 'UPDATED', 'SUBMITTED_FOR_APPROVAL', 'APPROVED', 'CANCELLED',
        'COMPLETED', 'DELETED', 'REPAYMENT_CREATED', 'REPAYMENT_UPDATED',
        'REPAYMENT_DELETED', 'DOCUMENT_UPLOADED', 'DOCUMENT_DELETED', 'STATUS_UPDATED'
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "FamilyContactType" AS ENUM (
        'RELATIVE', 'FAMILY_FRIEND', 'NEIGHBOR', 'EMERGENCY_CONTACT',
        'DOCTOR', 'LAWYER', 'SCHOOL', 'CHURCH', 'BANK', 'SACCO',
        'BUSINESS_PARTNER', 'LOAN_CONTACT', 'OTHER'
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "FamilyContactStatus" AS ENUM ('ACTIVE', 'INACTIVE');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "PersonPlanType" AS ENUM ('TASK', 'LONG_TERM_GOAL');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "PersonPlanStatus" AS ENUM ('PENDING', 'IN_PROGRESS', 'COMPLETED', 'CANCELLED');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "PersonPlanPriority" AS ENUM ('LOW', 'MEDIUM', 'HIGH');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "PersonGoalProgressMethod" AS ENUM ('MILESTONE', 'FINANCIAL');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "FamilyProjectCategory" AS ENUM ('HOUSE', 'VEHICLE', 'EDUCATION', 'BUSINESS', 'EVENT', 'OTHER');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "FamilyProjectStatus" AS ENUM ('PLANNED', 'ACTIVE', 'PAUSED', 'COMPLETED', 'CANCELLED');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "FamilyProjectContributionStrategy" AS ENUM (
        'EQUAL_SHARES', 'PLEDGES', 'MONTHLY', 'AS_AVAILABLE', 'CURRENT_BALANCE'
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "FamilyProjectContributionStatus" AS ENUM ('PENDING', 'VERIFIED', 'REJECTED', 'REVERSED');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "FamilyInviteStatus" AS ENUM ('PENDING', 'ACCEPTED', 'DECLINED', 'EXPIRED', 'REVOKED');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE "NotificationType" AS ENUM (
        'FAMILY_INVITE', 'TRANSACTION_APPROVED', 'TRANSACTION_REJECTED',
        'CONTRIBUTION_VERIFIED', 'CONTRIBUTION_REVERSED', 'SYSTEM'
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;


-- TABLES & CONSTRAINTS

-- 1. User
CREATE TABLE IF NOT EXISTS "User" (
    "id" TEXT PRIMARY KEY,
    "email" TEXT,
    "pendingEmail" TEXT,
    "phoneNumber" TEXT,
    "fullName" TEXT NOT NULL,
    "gender" TEXT,
    "passwordHash" TEXT,
    "googleId" TEXT,
    "avatarUrl" TEXT,
    "status" "UserStatus" NOT NULL DEFAULT 'ACTIVE',
    "emailVerifiedAt" TIMESTAMPTZ,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS "User_email_key" ON "User"("email");
CREATE UNIQUE INDEX IF NOT EXISTS "User_pendingEmail_key" ON "User"("pendingEmail");
CREATE UNIQUE INDEX IF NOT EXISTS "User_phoneNumber_key" ON "User"("phoneNumber");
CREATE UNIQUE INDEX IF NOT EXISTS "User_googleId_key" ON "User"("googleId");
CREATE INDEX IF NOT EXISTS "User_status_idx" ON "User"("status");
ALTER TABLE "User" ALTER COLUMN "email" DROP NOT NULL;


-- 2. PersonPlan
CREATE TABLE IF NOT EXISTS "PersonPlan" (
    "id" TEXT PRIMARY KEY,
    "personId" TEXT NOT NULL REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "type" "PersonPlanType" NOT NULL,
    "title" TEXT NOT NULL,
    "description" TEXT,
    "status" "PersonPlanStatus" NOT NULL DEFAULT 'PENDING',
    "priority" "PersonPlanPriority",
    "startDate" TIMESTAMPTZ,
    "dueDate" TIMESTAMPTZ,
    "dueTime" TEXT,
    "targetDate" TIMESTAMPTZ,
    "reminderAt" TIMESTAMPTZ,
    "reminderFrequency" TEXT,
    "note" TEXT,
    "goalCategory" TEXT,
    "goalIcon" TEXT,
    "needsMoney" BOOLEAN NOT NULL DEFAULT false,
    "targetAmountMinorUnits" BIGINT,
    "currentAmountMinorUnits" BIGINT,
    "currency" TEXT NOT NULL DEFAULT 'RWF',
    "progressMethod" "PersonGoalProgressMethod" NOT NULL DEFAULT 'MILESTONE',
    "completedAt" TIMESTAMPTZ,
    "deletedAt" TIMESTAMPTZ,
    "createdById" TEXT NOT NULL REFERENCES "User"("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS "PersonPlan_personId_status_idx" ON "PersonPlan"("personId", "status");
CREATE INDEX IF NOT EXISTS "PersonPlan_personId_dueDate_idx" ON "PersonPlan"("personId", "dueDate");
CREATE INDEX IF NOT EXISTS "PersonPlan_personId_targetDate_idx" ON "PersonPlan"("personId", "targetDate");
CREATE INDEX IF NOT EXISTS "PersonPlan_personId_deletedAt_idx" ON "PersonPlan"("personId", "deletedAt");


-- 3. PersonPlanMilestone
CREATE TABLE IF NOT EXISTS "PersonPlanMilestone" (
    "id" TEXT PRIMARY KEY,
    "planId" TEXT NOT NULL REFERENCES "PersonPlan"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "title" TEXT NOT NULL,
    "position" INTEGER NOT NULL,
    "completedAt" TIMESTAMPTZ,
    "deletedAt" TIMESTAMPTZ,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS "PersonPlanMilestone_planId_position_idx" ON "PersonPlanMilestone"("planId", "position");
CREATE INDEX IF NOT EXISTS "PersonPlanMilestone_planId_deletedAt_idx" ON "PersonPlanMilestone"("planId", "deletedAt");


-- 4. EmailVerification
CREATE TABLE IF NOT EXISTS "EmailVerification" (
    "id" TEXT PRIMARY KEY,
    "userId" TEXT NOT NULL REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "codeHash" TEXT NOT NULL,
    "expiresAt" TIMESTAMPTZ NOT NULL,
    "consumedAt" TIMESTAMPTZ,
    "attempts" INTEGER NOT NULL DEFAULT 0,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS "EmailVerification_userId_idx" ON "EmailVerification"("userId");
CREATE INDEX IF NOT EXISTS "EmailVerification_expiresAt_idx" ON "EmailVerification"("expiresAt");


-- 5. PasswordReset
CREATE TABLE IF NOT EXISTS "PasswordReset" (
    "id" TEXT PRIMARY KEY,
    "userId" TEXT NOT NULL REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "codeHash" TEXT NOT NULL,
    "expiresAt" TIMESTAMPTZ NOT NULL,
    "consumedAt" TIMESTAMPTZ,
    "attempts" INTEGER NOT NULL DEFAULT 0,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS "PasswordReset_userId_idx" ON "PasswordReset"("userId");
CREATE INDEX IF NOT EXISTS "PasswordReset_expiresAt_idx" ON "PasswordReset"("expiresAt");


-- 6. AuthSession
CREATE TABLE IF NOT EXISTS "AuthSession" (
    "id" TEXT PRIMARY KEY,
    "userId" TEXT NOT NULL REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "refreshTokenHash" TEXT NOT NULL,
    "userAgent" TEXT,
    "ipAddress" TEXT,
    "expiresAt" TIMESTAMPTZ NOT NULL,
    "revokedAt" TIMESTAMPTZ,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS "AuthSession_userId_idx" ON "AuthSession"("userId");
CREATE INDEX IF NOT EXISTS "AuthSession_expiresAt_idx" ON "AuthSession"("expiresAt");


-- 7. Family
CREATE TABLE IF NOT EXISTS "Family" (
    "id" TEXT PRIMARY KEY,
    "name" TEXT NOT NULL,
    "slug" TEXT NOT NULL,
    "joinCode" TEXT,
    "country" TEXT NOT NULL,
    "defaultCurrency" TEXT NOT NULL,
    "contributionCadence" TEXT NOT NULL,
    "type" "FamilyType" NOT NULL DEFAULT 'NUCLEAR',
    "description" TEXT,
    "createdById" TEXT NOT NULL REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "personalOwnerId" TEXT REFERENCES "User"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS "Family_slug_key" ON "Family"("slug");
CREATE UNIQUE INDEX IF NOT EXISTS "Family_joinCode_key" ON "Family"("joinCode");
CREATE UNIQUE INDEX IF NOT EXISTS "Family_personalOwnerId_key" ON "Family"("personalOwnerId");
CREATE INDEX IF NOT EXISTS "Family_createdById_idx" ON "Family"("createdById");
CREATE INDEX IF NOT EXISTS "Family_updatedAt_id_idx" ON "Family"("updatedAt", "id");


-- 8. FamilyMember
CREATE TABLE IF NOT EXISTS "FamilyMember" (
    "id" TEXT PRIMARY KEY,
    "familyId" TEXT NOT NULL REFERENCES "Family"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "userId" TEXT NOT NULL REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "authority" "FamilyAuthority" NOT NULL DEFAULT 'MEMBER',
    "kinshipLabel" "FamilyKinship",
    "kinshipCustom" TEXT,
    "capabilities" "FamilyCapability"[] NOT NULL DEFAULT '{}',
    "removedAt" TIMESTAMPTZ,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "FamilyMember_familyId_userId_key" UNIQUE ("familyId", "userId")
);

CREATE INDEX IF NOT EXISTS "FamilyMember_userId_idx" ON "FamilyMember"("userId");
CREATE INDEX IF NOT EXISTS "FamilyMember_familyId_updatedAt_id_idx" ON "FamilyMember"("familyId", "updatedAt", "id");


-- 9. FamilyInvite
CREATE TABLE IF NOT EXISTS "FamilyInvite" (
    "id" TEXT PRIMARY KEY,
    "familyId" TEXT NOT NULL REFERENCES "Family"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "invitedById" TEXT REFERENCES "User"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    "email" TEXT,
    "phone" TEXT,
    "authority" "FamilyAuthority" NOT NULL DEFAULT 'MEMBER',
    "kinshipLabel" "FamilyKinship",
    "kinshipCustom" TEXT,
    "capabilities" "FamilyCapability"[] NOT NULL DEFAULT '{}',
    "status" "FamilyInviteStatus" NOT NULL DEFAULT 'PENDING',
    "tokenHash" TEXT NOT NULL,
    "expiresAt" TIMESTAMPTZ NOT NULL,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS "FamilyInvite_email_idx" ON "FamilyInvite"("email");
CREATE INDEX IF NOT EXISTS "FamilyInvite_phone_idx" ON "FamilyInvite"("phone");
CREATE INDEX IF NOT EXISTS "FamilyInvite_invitedById_idx" ON "FamilyInvite"("invitedById");


-- 10. FamilyAuditEvent
CREATE TABLE IF NOT EXISTS "FamilyAuditEvent" (
    "id" TEXT PRIMARY KEY,
    "familyId" TEXT NOT NULL REFERENCES "Family"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "actorId" TEXT NOT NULL,
    "action" "FamilyAuditAction" NOT NULL,
    "targetMemberId" TEXT,
    "before" JSONB,
    "after" JSONB,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS "FamilyAuditEvent_familyId_createdAt_idx" ON "FamilyAuditEvent"("familyId", "createdAt");


-- 11. Notification
CREATE TABLE IF NOT EXISTS "Notification" (
    "id" TEXT PRIMARY KEY,
    "userId" TEXT NOT NULL REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "familyId" TEXT REFERENCES "Family"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "type" "NotificationType" NOT NULL,
    "title" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "href" TEXT,
    "readAt" TIMESTAMPTZ,
    "dedupeKey" TEXT,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS "Notification_dedupeKey_key" ON "Notification"("dedupeKey");
CREATE INDEX IF NOT EXISTS "Notification_userId_createdAt_idx" ON "Notification"("userId", "createdAt");
CREATE INDEX IF NOT EXISTS "Notification_userId_readAt_idx" ON "Notification"("userId", "readAt");
CREATE INDEX IF NOT EXISTS "Notification_familyId_idx" ON "Notification"("familyId");
CREATE INDEX IF NOT EXISTS "Notification_familyId_updatedAt_id_idx" ON "Notification"("familyId", "updatedAt", "id");


-- 12. AuthAuditEvent
CREATE TABLE IF NOT EXISTS "AuthAuditEvent" (
    "id" TEXT PRIMARY KEY,
    "userId" TEXT REFERENCES "User"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    "emailOrMobile" TEXT NOT NULL,
    "action" "AuthAuditAction" NOT NULL,
    "result" "AuthAuditResult" NOT NULL,
    "ipAddress" TEXT,
    "userAgent" TEXT,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS "AuthAuditEvent_userId_createdAt_idx" ON "AuthAuditEvent"("userId", "createdAt");
CREATE INDEX IF NOT EXISTS "AuthAuditEvent_action_result_createdAt_idx" ON "AuthAuditEvent"("action", "result", "createdAt");


-- 13. FamilyContributionPlan
CREATE TABLE IF NOT EXISTS "FamilyContributionPlan" (
    "id" TEXT PRIMARY KEY,
    "familyId" TEXT NOT NULL REFERENCES "Family"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "name" TEXT NOT NULL,
    "description" TEXT,
    "cadence" "ContributionCadence" NOT NULL,
    "amountMinorUnits" BIGINT NOT NULL,
    "currency" TEXT NOT NULL,
    "status" "ContributionPlanStatus" NOT NULL DEFAULT 'ACTIVE',
    "createdById" TEXT NOT NULL,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "archivedAt" TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS "FamilyContributionPlan_familyId_status_idx" ON "FamilyContributionPlan"("familyId", "status");
CREATE INDEX IF NOT EXISTS "FamilyContributionPlan_familyId_updatedAt_id_idx" ON "FamilyContributionPlan"("familyId", "updatedAt", "id");


-- 14. FamilyContributionAssignment
CREATE TABLE IF NOT EXISTS "FamilyContributionAssignment" (
    "id" TEXT PRIMARY KEY,
    "planId" TEXT NOT NULL REFERENCES "FamilyContributionPlan"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "memberId" TEXT NOT NULL REFERENCES "FamilyMember"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "amountMinorUnits" BIGINT NOT NULL,
    "removedAt" TIMESTAMPTZ,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "FamilyContributionAssignment_planId_memberId_key" UNIQUE ("planId", "memberId")
);

CREATE INDEX IF NOT EXISTS "FamilyContributionAssignment_memberId_idx" ON "FamilyContributionAssignment"("memberId");


-- 15. FamilyContributionCycle
CREATE TABLE IF NOT EXISTS "FamilyContributionCycle" (
    "id" TEXT PRIMARY KEY,
    "familyId" TEXT NOT NULL REFERENCES "Family"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "planId" TEXT NOT NULL REFERENCES "FamilyContributionPlan"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "label" TEXT NOT NULL,
    "periodStart" TIMESTAMPTZ NOT NULL,
    "periodEnd" TIMESTAMPTZ NOT NULL,
    "dueDate" TIMESTAMPTZ NOT NULL,
    "expectedMinorUnits" BIGINT NOT NULL,
    "status" "ContributionCycleStatus" NOT NULL DEFAULT 'OPEN',
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS "FamilyContributionCycle_familyId_dueDate_idx" ON "FamilyContributionCycle"("familyId", "dueDate");
CREATE INDEX IF NOT EXISTS "FamilyContributionCycle_planId_periodStart_idx" ON "FamilyContributionCycle"("planId", "periodStart");
CREATE INDEX IF NOT EXISTS "FamilyContributionCycle_familyId_updatedAt_id_idx" ON "FamilyContributionCycle"("familyId", "updatedAt", "id");


-- 16. FamilyContributionPayment
CREATE TABLE IF NOT EXISTS "FamilyContributionPayment" (
    "id" TEXT PRIMARY KEY,
    "familyId" TEXT NOT NULL REFERENCES "Family"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "planId" TEXT NOT NULL REFERENCES "FamilyContributionPlan"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "cycleId" TEXT REFERENCES "FamilyContributionCycle"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    "memberId" TEXT NOT NULL REFERENCES "FamilyMember"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "amountMinorUnits" BIGINT NOT NULL,
    "method" "ContributionPaymentMethod" NOT NULL DEFAULT 'CASH',
    "status" "ContributionPaymentStatus" NOT NULL DEFAULT 'PENDING',
    "reference" TEXT,
    "note" TEXT,
    "idempotencyKey" TEXT,
    "paidAt" TIMESTAMPTZ NOT NULL,
    "recordedById" TEXT NOT NULL,
    "verifiedById" TEXT,
    "verifiedAt" TIMESTAMPTZ,
    "reversedAt" TIMESTAMPTZ,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "FamilyContributionPayment_familyId_idempotencyKey_key" UNIQUE ("familyId", "idempotencyKey")
);

CREATE INDEX IF NOT EXISTS "FamilyContributionPayment_familyId_status_paidAt_idx" ON "FamilyContributionPayment"("familyId", "status", "paidAt");
CREATE INDEX IF NOT EXISTS "FamilyContributionPayment_planId_cycleId_idx" ON "FamilyContributionPayment"("planId", "cycleId");
CREATE INDEX IF NOT EXISTS "FamilyContributionPayment_memberId_idx" ON "FamilyContributionPayment"("memberId");
CREATE INDEX IF NOT EXISTS "FamilyContributionPayment_familyId_updatedAt_id_idx" ON "FamilyContributionPayment"("familyId", "updatedAt", "id");


-- 17. FamilyProject
CREATE TABLE IF NOT EXISTS "FamilyProject" (
    "id" TEXT PRIMARY KEY,
    "familyId" TEXT NOT NULL REFERENCES "Family"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "name" TEXT NOT NULL,
    "category" "FamilyProjectCategory" NOT NULL,
    "description" TEXT,
    "targetAmountMinorUnits" BIGINT NOT NULL,
    "currency" TEXT NOT NULL,
    "startDate" TIMESTAMPTZ,
    "targetDate" TIMESTAMPTZ NOT NULL,
    "status" "FamilyProjectStatus" NOT NULL DEFAULT 'ACTIVE',
    "ownerMemberId" TEXT REFERENCES "FamilyMember"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    "contributionStrategy" "FamilyProjectContributionStrategy" NOT NULL,
    "expectedContributionMinorUnits" BIGINT,
    "participantNotes" TEXT,
    "idempotencyKey" TEXT,
    "createdById" TEXT NOT NULL REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "completedAt" TIMESTAMPTZ,
    "archivedAt" TIMESTAMPTZ,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "FamilyProject_familyId_idempotencyKey_key" UNIQUE ("familyId", "idempotencyKey")
);

CREATE INDEX IF NOT EXISTS "FamilyProject_familyId_status_idx" ON "FamilyProject"("familyId", "status");
CREATE INDEX IF NOT EXISTS "FamilyProject_familyId_targetDate_idx" ON "FamilyProject"("familyId", "targetDate");
CREATE INDEX IF NOT EXISTS "FamilyProject_ownerMemberId_idx" ON "FamilyProject"("ownerMemberId");
CREATE INDEX IF NOT EXISTS "FamilyProject_createdById_idx" ON "FamilyProject"("createdById");
CREATE INDEX IF NOT EXISTS "FamilyProject_familyId_updatedAt_id_idx" ON "FamilyProject"("familyId", "updatedAt", "id");


-- 18. FamilyProjectParticipant
CREATE TABLE IF NOT EXISTS "FamilyProjectParticipant" (
    "id" TEXT PRIMARY KEY,
    "projectId" TEXT NOT NULL REFERENCES "FamilyProject"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "memberId" TEXT NOT NULL REFERENCES "FamilyMember"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "roleNotes" TEXT,
    "removedAt" TIMESTAMPTZ,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "FamilyProjectParticipant_projectId_memberId_key" UNIQUE ("projectId", "memberId")
);

CREATE INDEX IF NOT EXISTS "FamilyProjectParticipant_memberId_idx" ON "FamilyProjectParticipant"("memberId");


-- 19. FamilyProjectContribution
CREATE TABLE IF NOT EXISTS "FamilyProjectContribution" (
    "id" TEXT PRIMARY KEY,
    "familyId" TEXT NOT NULL REFERENCES "Family"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "projectId" TEXT NOT NULL REFERENCES "FamilyProject"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "memberId" TEXT NOT NULL REFERENCES "FamilyMember"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "amountMinorUnits" BIGINT NOT NULL,
    "currency" TEXT NOT NULL,
    "status" "FamilyProjectContributionStatus" NOT NULL DEFAULT 'PENDING',
    "reference" TEXT,
    "note" TEXT,
    "idempotencyKey" TEXT,
    "paidAt" TIMESTAMPTZ NOT NULL,
    "recordedById" TEXT NOT NULL REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "verifiedById" TEXT REFERENCES "User"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    "verifiedAt" TIMESTAMPTZ,
    "reversedAt" TIMESTAMPTZ,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "FamilyProjectContribution_familyId_idempotencyKey_key" UNIQUE ("familyId", "idempotencyKey")
);

CREATE INDEX IF NOT EXISTS "FamilyProjectContribution_familyId_status_paidAt_idx" ON "FamilyProjectContribution"("familyId", "status", "paidAt");
CREATE INDEX IF NOT EXISTS "FamilyProjectContribution_projectId_paidAt_idx" ON "FamilyProjectContribution"("projectId", "paidAt");
CREATE INDEX IF NOT EXISTS "FamilyProjectContribution_memberId_idx" ON "FamilyProjectContribution"("memberId");


-- 20. FamilyProjectUpdate
CREATE TABLE IF NOT EXISTS "FamilyProjectUpdate" (
    "id" TEXT PRIMARY KEY,
    "familyId" TEXT NOT NULL REFERENCES "Family"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "projectId" TEXT NOT NULL REFERENCES "FamilyProject"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "title" TEXT NOT NULL,
    "body" TEXT,
    "createdById" TEXT NOT NULL REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS "FamilyProjectUpdate_familyId_createdAt_idx" ON "FamilyProjectUpdate"("familyId", "createdAt");
CREATE INDEX IF NOT EXISTS "FamilyProjectUpdate_projectId_createdAt_idx" ON "FamilyProjectUpdate"("projectId", "createdAt");


-- 21. FamilyContact
CREATE TABLE IF NOT EXISTS "FamilyContact" (
    "id" TEXT PRIMARY KEY,
    "familyId" TEXT NOT NULL REFERENCES "Family"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "fullName" TEXT NOT NULL,
    "contactType" "FamilyContactType" NOT NULL,
    "phoneNumber" TEXT,
    "email" TEXT,
    "address" TEXT,
    "relationshipToFamily" TEXT,
    "organization" TEXT,
    "notes" TEXT,
    "isEmergencyContact" BOOLEAN NOT NULL DEFAULT false,
    "isLoanContact" BOOLEAN NOT NULL DEFAULT false,
    "status" "FamilyContactStatus" NOT NULL DEFAULT 'ACTIVE',
    "createdById" TEXT NOT NULL REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "deletedAt" TIMESTAMPTZ,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS "FamilyContact_familyId_status_idx" ON "FamilyContact"("familyId", "status");
CREATE INDEX IF NOT EXISTS "FamilyContact_familyId_contactType_idx" ON "FamilyContact"("familyId", "contactType");
CREATE INDEX IF NOT EXISTS "FamilyContact_familyId_isEmergencyContact_idx" ON "FamilyContact"("familyId", "isEmergencyContact");
CREATE INDEX IF NOT EXISTS "FamilyContact_familyId_isLoanContact_idx" ON "FamilyContact"("familyId", "isLoanContact");
CREATE INDEX IF NOT EXISTS "FamilyContact_createdById_idx" ON "FamilyContact"("createdById");
CREATE INDEX IF NOT EXISTS "FamilyContact_familyId_updatedAt_id_idx" ON "FamilyContact"("familyId", "updatedAt", "id");


-- 22. FamilyContactDisplayName
CREATE TABLE IF NOT EXISTS "FamilyContactDisplayName" (
    "id" TEXT PRIMARY KEY,
    "contactId" TEXT NOT NULL REFERENCES "FamilyContact"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "userId" TEXT NOT NULL REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "fullName" TEXT NOT NULL,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "FamilyContactDisplayName_contactId_userId_key" UNIQUE ("contactId", "userId")
);

CREATE INDEX IF NOT EXISTS "FamilyContactDisplayName_userId_idx" ON "FamilyContactDisplayName"("userId");


-- 23. FamilyTransactionCategory
CREATE TABLE IF NOT EXISTS "FamilyTransactionCategory" (
    "id" TEXT PRIMARY KEY,
    "familyId" TEXT NOT NULL REFERENCES "Family"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "name" TEXT NOT NULL,
    "type" "TransactionType" NOT NULL,
    "description" TEXT,
    "icon" TEXT,
    "color" TEXT,
    "status" "CategoryStatus" NOT NULL DEFAULT 'ACTIVE',
    "isDefault" BOOLEAN NOT NULL DEFAULT false,
    "scope" "TransactionScope" NOT NULL DEFAULT 'FAMILY',
    "ownerId" TEXT REFERENCES "User"("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    "createdById" TEXT REFERENCES "User"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "FamilyTransactionCategory_familyId_scope_ownerId_type_name_key"
        UNIQUE ("familyId", "scope", "ownerId", "type", "name")
);

CREATE INDEX IF NOT EXISTS "FamilyTransactionCategory_familyId_type_idx" ON "FamilyTransactionCategory"("familyId", "type");
CREATE INDEX IF NOT EXISTS "FamilyTransactionCategory_familyId_status_idx" ON "FamilyTransactionCategory"("familyId", "status");
CREATE INDEX IF NOT EXISTS "FamilyTransactionCat_famId_scope_ownerId_type_status_idx"
    ON "FamilyTransactionCategory"("familyId", "scope", "ownerId", "type", "status");
CREATE INDEX IF NOT EXISTS "FamilyTransactionCategory_familyId_updatedAt_id_idx"
    ON "FamilyTransactionCategory"("familyId", "updatedAt", "id");


-- 24. FamilyTransaction
CREATE TABLE IF NOT EXISTS "FamilyTransaction" (
    "id" TEXT PRIMARY KEY,
    "familyId" TEXT NOT NULL REFERENCES "Family"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "categoryId" TEXT NOT NULL REFERENCES "FamilyTransactionCategory"("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    "amountMinorUnits" BIGINT NOT NULL,
    "currency" TEXT NOT NULL,
    "type" "TransactionType" NOT NULL,
    "scope" "TransactionScope" NOT NULL DEFAULT 'FAMILY',
    "status" "TransactionStatus" NOT NULL DEFAULT 'PENDING',
    "incomeRecurrence" "IncomeRecurrence" NOT NULL DEFAULT 'ONE_TIME',
    "paymentStatus" "IncomePaymentStatus" NOT NULL DEFAULT 'NOT_EXPECTED',
    "date" TIMESTAMPTZ NOT NULL,
    "description" TEXT,
    "paymentMethod" TEXT,
    "idempotencyKey" TEXT,
    "createdById" TEXT NOT NULL REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "approvedById" TEXT REFERENCES "User"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    "approvedAt" TIMESTAMPTZ,
    "reversedAt" TIMESTAMPTZ,
    "notes" TEXT,
    "receivedFrom" TEXT,
    "receivedFromContactId" TEXT REFERENCES "FamilyContact"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    "receivedBy" TEXT,
    "expectedAmountMinorUnits" BIGINT,
    "dueDate" TIMESTAMPTZ,
    "periodStart" TIMESTAMPTZ,
    "periodEnd" TIMESTAMPTZ,
    "propertyName" TEXT,
    "projectId" TEXT REFERENCES "FamilyProject"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "FamilyTransaction_familyId_idempotencyKey_key" UNIQUE ("familyId", "idempotencyKey")
);

CREATE INDEX IF NOT EXISTS "FamilyTransaction_familyId_scope_type_date_idx"
    ON "FamilyTransaction"("familyId", "scope", "type", "date");
CREATE INDEX IF NOT EXISTS "FamilyTransaction_familyId_type_date_idx"
    ON "FamilyTransaction"("familyId", "type", "date");
CREATE INDEX IF NOT EXISTS "FamilyTransaction_familyId_incomeRecurrence_dueDate_idx"
    ON "FamilyTransaction"("familyId", "incomeRecurrence", "dueDate");
CREATE INDEX IF NOT EXISTS "FamilyTransaction_familyId_paymentStatus_idx"
    ON "FamilyTransaction"("familyId", "paymentStatus");
CREATE INDEX IF NOT EXISTS "FamilyTransaction_familyId_status_idx"
    ON "FamilyTransaction"("familyId", "status");
CREATE INDEX IF NOT EXISTS "FamilyTransaction_familyId_receivedFromContactId_idx"
    ON "FamilyTransaction"("familyId", "receivedFromContactId");
CREATE INDEX IF NOT EXISTS "FamilyTransaction_familyId_projectId_status_idx"
    ON "FamilyTransaction"("familyId", "projectId", "status");
CREATE INDEX IF NOT EXISTS "FamilyTransaction_categoryId_idx"
    ON "FamilyTransaction"("categoryId");
CREATE INDEX IF NOT EXISTS "FamilyTransaction_familyId_updatedAt_id_idx"
    ON "FamilyTransaction"("familyId", "updatedAt", "id");


-- 25. FamilyReceipt
CREATE TABLE IF NOT EXISTS "FamilyReceipt" (
    "id" TEXT PRIMARY KEY,
    "familyId" TEXT NOT NULL REFERENCES "Family"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "transactionId" TEXT NOT NULL REFERENCES "FamilyTransaction"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "fileName" TEXT NOT NULL,
    "fileType" TEXT NOT NULL,
    "fileSizeBytes" INTEGER NOT NULL,
    "storageKey" TEXT NOT NULL,
    "checksumSha256" TEXT,
    "idempotencyKey" TEXT,
    "uploadedById" TEXT NOT NULL REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "FamilyReceipt_familyId_idempotencyKey_key" UNIQUE ("familyId", "idempotencyKey")
);

CREATE INDEX IF NOT EXISTS "FamilyReceipt_familyId_transactionId_idx"
    ON "FamilyReceipt"("familyId", "transactionId");
CREATE INDEX IF NOT EXISTS "FamilyReceipt_familyId_updatedAt_id_idx"
    ON "FamilyReceipt"("familyId", "updatedAt", "id");


-- 26. FamilyBudget
CREATE TABLE IF NOT EXISTS "FamilyBudget" (
    "id" TEXT PRIMARY KEY,
    "familyId" TEXT NOT NULL REFERENCES "Family"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "categoryId" TEXT REFERENCES "FamilyTransactionCategory"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "amountMinorUnits" BIGINT NOT NULL,
    "currency" TEXT NOT NULL,
    "startDate" TIMESTAMPTZ NOT NULL,
    "endDate" TIMESTAMPTZ NOT NULL,
    "createdById" TEXT NOT NULL REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS "FamilyBudget_familyId_startDate_endDate_idx"
    ON "FamilyBudget"("familyId", "startDate", "endDate");
CREATE INDEX IF NOT EXISTS "FamilyBudget_categoryId_idx"
    ON "FamilyBudget"("categoryId");


-- 27. FamilyLoan
CREATE TABLE IF NOT EXISTS "FamilyLoan" (
    "id" TEXT PRIMARY KEY,
    "familyId" TEXT NOT NULL REFERENCES "Family"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "title" TEXT NOT NULL,
    "description" TEXT,
    "loanType" "LoanType" NOT NULL,
    "principalAmountMinorUnits" BIGINT NOT NULL,
    "interestRate" DECIMAL(12, 4) NOT NULL DEFAULT 0,
    "interestType" "LoanInterestType" NOT NULL DEFAULT 'NONE',
    "totalExpectedAmountMinorUnits" BIGINT NOT NULL,
    "paidAmountMinorUnits" BIGINT NOT NULL DEFAULT 0,
    "remainingBalanceMinorUnits" BIGINT NOT NULL,
    "currency" TEXT NOT NULL,
    "borrowerName" TEXT,
    "borrowerMemberId" TEXT REFERENCES "FamilyMember"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    "borrowerContactId" TEXT REFERENCES "FamilyContact"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    "lenderName" TEXT,
    "lenderMemberId" TEXT REFERENCES "FamilyMember"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    "lenderContactId" TEXT REFERENCES "FamilyContact"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    "categoryId" TEXT REFERENCES "FamilyTransactionCategory"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    "startDate" TIMESTAMPTZ NOT NULL,
    "dueDate" TIMESTAMPTZ NOT NULL,
    "repaymentFrequency" "LoanRepaymentFrequency" NOT NULL DEFAULT 'ONCE',
    "status" "LoanStatus" NOT NULL DEFAULT 'DRAFT',
    "approvalStatus" "LoanApprovalStatus" NOT NULL DEFAULT 'DRAFT',
    "approvedById" TEXT REFERENCES "User"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    "approvedAt" TIMESTAMPTZ,
    "createdById" TEXT NOT NULL REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "updatedById" TEXT REFERENCES "User"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    "idempotencyKey" TEXT,
    "deletedAt" TIMESTAMPTZ,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "FamilyLoan_familyId_idempotencyKey_key" UNIQUE ("familyId", "idempotencyKey")
);

CREATE INDEX IF NOT EXISTS "FamilyLoan_familyId_status_idx" ON "FamilyLoan"("familyId", "status");
CREATE INDEX IF NOT EXISTS "FamilyLoan_familyId_approvalStatus_idx" ON "FamilyLoan"("familyId", "approvalStatus");
CREATE INDEX IF NOT EXISTS "FamilyLoan_familyId_dueDate_idx" ON "FamilyLoan"("familyId", "dueDate");
CREATE INDEX IF NOT EXISTS "FamilyLoan_familyId_loanType_idx" ON "FamilyLoan"("familyId", "loanType");
CREATE INDEX IF NOT EXISTS "FamilyLoan_familyId_categoryId_idx" ON "FamilyLoan"("familyId", "categoryId");
CREATE INDEX IF NOT EXISTS "FamilyLoan_borrowerMemberId_idx" ON "FamilyLoan"("borrowerMemberId");
CREATE INDEX IF NOT EXISTS "FamilyLoan_borrowerContactId_idx" ON "FamilyLoan"("borrowerContactId");
CREATE INDEX IF NOT EXISTS "FamilyLoan_lenderMemberId_idx" ON "FamilyLoan"("lenderMemberId");
CREATE INDEX IF NOT EXISTS "FamilyLoan_lenderContactId_idx" ON "FamilyLoan"("lenderContactId");
CREATE INDEX IF NOT EXISTS "FamilyLoan_createdById_idx" ON "FamilyLoan"("createdById");


-- 28. FamilyLoanRepayment
CREATE TABLE IF NOT EXISTS "FamilyLoanRepayment" (
    "id" TEXT PRIMARY KEY,
    "loanId" TEXT NOT NULL REFERENCES "FamilyLoan"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "familyId" TEXT NOT NULL REFERENCES "Family"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "amountMinorUnits" BIGINT NOT NULL,
    "paymentDate" TIMESTAMPTZ NOT NULL,
    "paymentMethod" "LoanRepaymentMethod" NOT NULL DEFAULT 'CASH',
    "paidByMemberId" TEXT REFERENCES "FamilyMember"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    "receivedByMemberId" TEXT REFERENCES "FamilyMember"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    "receiptId" TEXT REFERENCES "FamilyReceipt"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    "notes" TEXT,
    "idempotencyKey" TEXT,
    "createdById" TEXT NOT NULL REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "deletedAt" TIMESTAMPTZ,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "FamilyLoanRepayment_familyId_idempotencyKey_key" UNIQUE ("familyId", "idempotencyKey")
);

CREATE INDEX IF NOT EXISTS "FamilyLoanRepayment_familyId_paymentDate_idx"
    ON "FamilyLoanRepayment"("familyId", "paymentDate");
CREATE INDEX IF NOT EXISTS "FamilyLoanRepayment_loanId_deletedAt_idx"
    ON "FamilyLoanRepayment"("loanId", "deletedAt");
CREATE INDEX IF NOT EXISTS "FamilyLoanRepayment_paidByMemberId_idx"
    ON "FamilyLoanRepayment"("paidByMemberId");
CREATE INDEX IF NOT EXISTS "FamilyLoanRepayment_receivedByMemberId_idx"
    ON "FamilyLoanRepayment"("receivedByMemberId");
CREATE INDEX IF NOT EXISTS "FamilyLoanRepayment_receiptId_idx"
    ON "FamilyLoanRepayment"("receiptId");


-- 29. FamilyLoanDocument
CREATE TABLE IF NOT EXISTS "FamilyLoanDocument" (
    "id" TEXT PRIMARY KEY,
    "loanId" TEXT NOT NULL REFERENCES "FamilyLoan"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "fileName" TEXT NOT NULL,
    "fileUrl" TEXT NOT NULL,
    "fileType" TEXT NOT NULL,
    "fileSizeBytes" INTEGER,
    "uploadedById" TEXT NOT NULL REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "deletedAt" TIMESTAMPTZ,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS "FamilyLoanDocument_loanId_deletedAt_idx"
    ON "FamilyLoanDocument"("loanId", "deletedAt");
CREATE INDEX IF NOT EXISTS "FamilyLoanDocument_uploadedById_idx"
    ON "FamilyLoanDocument"("uploadedById");


-- 30. FamilyLoanAuditLog
CREATE TABLE IF NOT EXISTS "FamilyLoanAuditLog" (
    "id" TEXT PRIMARY KEY,
    "loanId" TEXT NOT NULL REFERENCES "FamilyLoan"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "action" "LoanAuditAction" NOT NULL,
    "oldValue" JSONB,
    "newValue" JSONB,
    "description" TEXT,
    "performedById" TEXT NOT NULL REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS "FamilyLoanAuditLog_loanId_createdAt_idx"
    ON "FamilyLoanAuditLog"("loanId", "createdAt");
CREATE INDEX IF NOT EXISTS "FamilyLoanAuditLog_performedById_idx"
    ON "FamilyLoanAuditLog"("performedById");


-- 31. FamilySyncMutation
CREATE TABLE IF NOT EXISTS "FamilySyncMutation" (
    "id" TEXT PRIMARY KEY,
    "familyId" TEXT NOT NULL REFERENCES "Family"("id") ON DELETE CASCADE ON UPDATE CASCADE,
    "actorId" TEXT NOT NULL,
    "deviceId" TEXT NOT NULL,
    "idempotencyKey" TEXT NOT NULL,
    "localRecordId" TEXT NOT NULL,
    "commandType" TEXT NOT NULL,
    "payloadHash" TEXT NOT NULL,
    "outcome" TEXT NOT NULL,
    "conflictCode" TEXT,
    "serverRecordId" TEXT,
    "responseJson" JSONB NOT NULL,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "FamilySyncMutation_familyId_idempotencyKey_key" UNIQUE ("familyId", "idempotencyKey")
);

CREATE INDEX IF NOT EXISTS "FamilySyncMutation_familyId_actorId_createdAt_idx"
    ON "FamilySyncMutation"("familyId", "actorId", "createdAt");
CREATE INDEX IF NOT EXISTS "FamilySyncMutation_familyId_updatedAt_id_idx"
    ON "FamilySyncMutation"("familyId", "updatedAt", "id");
