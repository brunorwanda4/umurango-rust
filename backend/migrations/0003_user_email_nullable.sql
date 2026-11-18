ALTER TABLE "User"
ALTER COLUMN "email" DROP NOT NULL;

ALTER TABLE "Family"
ADD COLUMN IF NOT EXISTS "joinCode" TEXT;

CREATE UNIQUE INDEX IF NOT EXISTS "Family_joinCode_key"
ON "Family"("joinCode");

ALTER TABLE "FamilyInvite"
ADD COLUMN IF NOT EXISTS "phone" TEXT;

ALTER TABLE "FamilyInvite"
ALTER COLUMN "email" DROP NOT NULL;

CREATE INDEX IF NOT EXISTS "FamilyInvite_phone_idx"
ON "FamilyInvite"("phone");
