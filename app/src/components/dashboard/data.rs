use crate::components::ui::BadgeVariant;

pub struct TransactionRow {
    pub actor: &'static str,
    pub title: &'static str,
    pub date: &'static str,
    pub amount: &'static str,
    pub is_income: bool,
    pub category: &'static str,
    pub category_variant: BadgeVariant,
    pub status: &'static str,
    pub status_variant: BadgeVariant,
}

pub const TRANSACTIONS: [TransactionRow; 5] = [
    TransactionRow {
        actor: "Keza Alice",
        title: "Monthly Contribution",
        date: "Today, 10:30 AM",
        amount: "+50,000 RWF",
        is_income: true,
        category: "Contribution",
        category_variant: BadgeVariant::Primary,
        status: "Approved",
        status_variant: BadgeVariant::Success,
    },
    TransactionRow {
        actor: "Bruno Mugabo",
        title: "School Fees Q1 - Junior",
        date: "Yesterday, 3:15 PM",
        amount: "-120,000 RWF",
        is_income: false,
        category: "Education",
        category_variant: BadgeVariant::Neutral,
        status: "Approved",
        status_variant: BadgeVariant::Success,
    },
    TransactionRow {
        actor: "Eric Mugabo",
        title: "Medical Emergency Assistance",
        date: "Nov 16, 2026",
        amount: "-85,000 RWF",
        is_income: false,
        category: "Health",
        category_variant: BadgeVariant::Neutral,
        status: "Pending Approval",
        status_variant: BadgeVariant::Warning,
    },
    TransactionRow {
        actor: "Claudine Mugabo",
        title: "Agricultural Inputs - Seeds",
        date: "Nov 14, 2026",
        amount: "-65,000 RWF",
        is_income: false,
        category: "Farming",
        category_variant: BadgeVariant::Neutral,
        status: "Approved",
        status_variant: BadgeVariant::Success,
    },
    TransactionRow {
        actor: "Bruno Mugabo",
        title: "Monthly Contribution",
        date: "Nov 12, 2026",
        amount: "+100,000 RWF",
        is_income: true,
        category: "Contribution",
        category_variant: BadgeVariant::Primary,
        status: "Approved",
        status_variant: BadgeVariant::Success,
    },
];

pub struct ProjectSummary {
    pub name: &'static str,
    pub target: &'static str,
    pub progress: &'static str,
    pub contributors: &'static str,
    pub deadline: &'static str,
    pub variant: BadgeVariant,
}

pub const PROJECTS: [ProjectSummary; 2] = [
    ProjectSummary {
        name: "Family Land Purchase (Bugesera)",
        target: "Target: 5,000,000 RWF • Raised: 3,750,000 RWF",
        progress: "75% funded",
        contributors: "4 contributors",
        deadline: "Closes in 45 days",
        variant: BadgeVariant::Primary,
    },
    ProjectSummary {
        name: "Holiday Reunion & Feast 2026",
        target: "Target: 800,000 RWF • Raised: 600,000 RWF",
        progress: "75% funded",
        contributors: "8 contributors",
        deadline: "Closes in 20 days",
        variant: BadgeVariant::Success,
    },
];
