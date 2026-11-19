#![allow(dead_code)]

use crate::components::ui::BadgeVariant;
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::{rgb, Rgba, SharedString};
use std::sync::LazyLock;
use umuryango_shared::{format_money, ProjectDto, TransactionItemDto};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MoneySubTab {
    #[default]
    Overview,
    Income,
    Expenses,
    Loans,
}

impl MoneySubTab {
    pub const ALL: [Self; 4] = [Self::Overview, Self::Income, Self::Expenses, Self::Loans];

    pub fn label(self) -> &'static str {
        match self {
            Self::Overview => "Overview",
            Self::Income => "Income",
            Self::Expenses => "Expenses",
            Self::Loans => "Loan & Borrowed",
        }
    }

    pub fn label_for(self, lang: Language) -> String {
        match self {
            Self::Overview => text(lang, "cozy.tab_overview"),
            Self::Income => text(lang, "cozy.tab_income"),
            Self::Expenses => text(lang, "cozy.tab_expenses"),
            Self::Loans => text(lang, "cozy.tab_loans"),
        }
    }

    pub fn nav_title_for(self, lang: Language) -> String {
        match self {
            Self::Overview => text(lang, "cozy.nav_money_overview_title"),
            Self::Income => text(lang, "cozy.nav_money_income_title"),
            Self::Expenses => text(lang, "cozy.nav_money_expenses_title"),
            Self::Loans => text(lang, "cozy.nav_money_loans_title"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartMode {
    #[default]
    Cashflow,
    Spending,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LoanFilter {
    #[default]
    All,
    Lent,
    Borrowed,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ProjectFilter {
    #[default]
    Active,
    Completed,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum QuickEntryKind {
    #[default]
    Income,
    Expense,
    Loan,
    Project,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionRow {
    pub actor: SharedString,
    pub title: SharedString,
    pub date: SharedString,
    pub amount: SharedString,
    pub amount_rwf: i64,
    pub is_income: bool,
    pub is_loan: bool,
    pub icon_name: &'static str,
    pub category: SharedString,
    pub category_variant: BadgeVariant,
    pub status: SharedString,
    pub status_variant: BadgeVariant,
}

impl TransactionRow {
    pub fn from_dto(dto: &TransactionItemDto) -> Self {
        let (status_variant, status_label) = match dto.status.as_str() {
            "APPROVED" => (BadgeVariant::Success, "Approved"),
            "PENDING" => (BadgeVariant::Warning, "Pending Approval"),
            "REJECTED" => (BadgeVariant::Error, "Rejected"),
            _ => (BadgeVariant::Neutral, "Unknown"),
        };
        let category_variant = if dto.is_income {
            BadgeVariant::Primary
        } else {
            BadgeVariant::Neutral
        };
        let formatted_amount = if dto.is_income {
            format!("+{}", format_money(dto.amount_minor_units, &dto.currency))
        } else {
            format!("-{}", format_money(dto.amount_minor_units, &dto.currency))
        };
        let date_str = if dto.occurred_at.len() >= 10 {
            &dto.occurred_at[..10]
        } else {
            &dto.occurred_at
        };
        let icon_name = if dto.is_income {
            icons::ICON_ARROW_DOWN
        } else {
            icons::ICON_CART_SHOPPING
        };
        Self {
            actor: dto.actor_name.clone().into(),
            title: dto.title.clone().into(),
            date: date_str.to_string().into(),
            amount: formatted_amount.into(),
            amount_rwf: dto.amount_minor_units / 100,
            is_income: dto.is_income,
            is_loan: false,
            icon_name,
            category: dto.category.clone().into(),
            category_variant,
            status: status_label.into(),
            status_variant,
        }
    }
}

pub static TRANSACTIONS: LazyLock<[TransactionRow; 6]> = LazyLock::new(|| {
    [
        TransactionRow {
            actor: "Agnes · MTN MoMo".into(),
            title: "Agnes Monthly Salary Contribution".into(),
            date: "Today, 09:40".into(),
            amount: "+450,000 RWF".into(),
            amount_rwf: 450_000,
            is_income: true,
            is_loan: false,
            icon_name: icons::ICON_COINS,
            category: "Salary".into(),
            category_variant: BadgeVariant::Success,
            status: "Approved".into(),
            status_variant: BadgeVariant::Success,
        },
        TransactionRow {
            actor: "Bruno · MoMo Pay".into(),
            title: "Kimironko Market & Groceries".into(),
            date: "Yesterday".into(),
            amount: "-65,000 RWF".into(),
            amount_rwf: 65_000,
            is_income: false,
            is_loan: false,
            icon_name: icons::ICON_CART_SHOPPING,
            category: "Groceries".into(),
            category_variant: BadgeVariant::Neutral,
            status: "Approved".into(),
            status_variant: BadgeVariant::Success,
        },
        TransactionRow {
            actor: "Bruno · Bank Transfer".into(),
            title: "Bruno Consulting & Tech Retainer".into(),
            date: "Nov 12".into(),
            amount: "+320,000 RWF".into(),
            amount_rwf: 320_000,
            is_income: true,
            is_loan: false,
            icon_name: icons::ICON_WALLET,
            category: "Consulting".into(),
            category_variant: BadgeVariant::Primary,
            status: "Approved".into(),
            status_variant: BadgeVariant::Success,
        },
        TransactionRow {
            actor: "Cousin Jean-Paul · MoMo".into(),
            title: "Jean-Paul Loan Repayment #2".into(),
            date: "Nov 10".into(),
            amount: "+40,000 RWF".into(),
            amount_rwf: 40_000,
            is_income: true,
            is_loan: true,
            icon_name: icons::ICON_HANDSHAKE,
            category: "Loan Repayment".into(),
            category_variant: BadgeVariant::Warning,
            status: "Approved".into(),
            status_variant: BadgeVariant::Success,
        },
        TransactionRow {
            actor: "Agnes · I&M Bank".into(),
            title: "Keza School Uniform & Books".into(),
            date: "Nov 08".into(),
            amount: "-115,000 RWF".into(),
            amount_rwf: 115_000,
            is_income: false,
            is_loan: false,
            icon_name: icons::ICON_GRADUATION_CAP,
            category: "Education".into(),
            category_variant: BadgeVariant::Info,
            status: "Approved".into(),
            status_variant: BadgeVariant::Success,
        },
        TransactionRow {
            actor: "Papa · Airtel Money".into(),
            title: "REG CashPower & WASAC Water".into(),
            date: "Nov 04".into(),
            amount: "-48,000 RWF".into(),
            amount_rwf: 48_000,
            is_income: false,
            is_loan: false,
            icon_name: icons::ICON_BOLT,
            category: "Utilities".into(),
            category_variant: BadgeVariant::Neutral,
            status: "Approved".into(),
            status_variant: BadgeVariant::Success,
        },
    ]
});

#[derive(Clone, Debug, PartialEq)]
pub struct ProjectSummary {
    pub id: SharedString,
    pub name: SharedString,
    pub lead: SharedString,
    pub target: SharedString,
    pub progress: SharedString,
    pub contributors: SharedString,
    pub deadline: SharedString,
    pub saved_rwf: i64,
    pub target_rwf: i64,
    pub pct: u8,
    pub is_completed: bool,
    pub icon_name: &'static str,
    pub fill_color: Rgba,
    pub variant: BadgeVariant,
}

impl ProjectSummary {
    pub fn from_dto(dto: &ProjectDto) -> Self {
        let pct = dto.progress_percentage.clamp(0, 100) as u8;
        let is_completed = pct >= 100;
        let variant = if pct >= 70 {
            BadgeVariant::Success
        } else {
            BadgeVariant::Primary
        };
        let target_str = format!(
            "Target: {} • Raised: {}",
            format_money(dto.target_amount_minor_units, &dto.currency),
            format_money(dto.raised_amount_minor_units, &dto.currency)
        );
        let progress_str = format!("{pct}% funded");
        let contrib_str = format!("{} contributors", dto.contributors_count);
        let deadline_str = if dto.target_date.len() >= 10 {
            dto.target_date[..10].to_string()
        } else {
            "Active".into()
        };

        Self {
            id: dto.id.clone().into(),
            name: dto.name.clone().into(),
            lead: "Family Circle".into(),
            target: target_str.into(),
            progress: progress_str.into(),
            contributors: contrib_str.into(),
            deadline: deadline_str.into(),
            saved_rwf: dto.raised_amount_minor_units / 100,
            target_rwf: dto.target_amount_minor_units / 100,
            pct,
            is_completed,
            icon_name: icons::ICON_BULLSEYE,
            fill_color: rgb(0x8CE0B8),
            variant,
        }
    }

    pub fn boost(&mut self, amount_rwf: i64) {
        self.saved_rwf = (self.saved_rwf + amount_rwf).min(self.target_rwf.max(self.saved_rwf));
        if self.target_rwf > 0 {
            let computed = ((self.saved_rwf * 100) / self.target_rwf).clamp(0, 100) as u8;
            self.pct = computed;
            self.is_completed = computed >= 100;
            self.progress = format!("{computed}% funded").into();
        }
    }
}

pub static PROJECTS: LazyLock<[ProjectSummary; 4]> = LazyLock::new(|| {
    [
        ProjectSummary {
            id: "p1".into(),
            name: "Gisenyi Solar Roof & Battery".into(),
            lead: "Bruno & Papa".into(),
            target: "Target: 1,500,000 RWF • Raised: 1,150,000 RWF".into(),
            progress: "77% funded".into(),
            contributors: "4 contributors".into(),
            deadline: "Dec 2026".into(),
            saved_rwf: 1_150_000,
            target_rwf: 1_500_000,
            pct: 77,
            is_completed: false,
            icon_name: icons::ICON_SUN,
            fill_color: rgb(0x8CE0B8),
            variant: BadgeVariant::Success,
        },
        ProjectSummary {
            id: "p2".into(),
            name: "Keza Term 1 Tuition Reserve".into(),
            lead: "Agnes".into(),
            target: "Target: 600,000 RWF • Raised: 540,000 RWF".into(),
            progress: "90% funded".into(),
            contributors: "4 contributors".into(),
            deadline: "Jan 2027".into(),
            saved_rwf: 540_000,
            target_rwf: 600_000,
            pct: 90,
            is_completed: false,
            icon_name: icons::ICON_GRADUATION_CAP,
            fill_color: rgb(0xF9D67A),
            variant: BadgeVariant::Success,
        },
        ProjectSummary {
            id: "p3".into(),
            name: "Musanze Grandma Garden Fence".into(),
            lead: "Papa Jean".into(),
            target: "Target: 450,000 RWF • Raised: 290,000 RWF".into(),
            progress: "64% funded".into(),
            contributors: "3 contributors".into(),
            deadline: "Feb 2027".into(),
            saved_rwf: 290_000,
            target_rwf: 450_000,
            pct: 64,
            is_completed: false,
            icon_name: icons::ICON_SEEDLING,
            fill_color: rgb(0xFFA69E),
            variant: BadgeVariant::Primary,
        },
        ProjectSummary {
            id: "p4".into(),
            name: "Home Water Tank 5,000L".into(),
            lead: "Bruno".into(),
            target: "Target: 380,000 RWF • Raised: 380,000 RWF".into(),
            progress: "100% funded".into(),
            contributors: "4 contributors".into(),
            deadline: "Done".into(),
            saved_rwf: 380_000,
            target_rwf: 380_000,
            pct: 100,
            is_completed: true,
            icon_name: icons::ICON_CIRCLE_CHECK,
            fill_color: rgb(0xA9C9FF),
            variant: BadgeVariant::Success,
        },
    ]
});

#[derive(Clone, Copy, Debug)]
pub struct MonthlyCashflowPoint {
    pub month: &'static str,
    pub income_pct: u8,
    pub expense_pct: u8,
    pub income_label: &'static str,
    pub expense_label: &'static str,
}

pub const MONTHLY_CASHFLOW: [MonthlyCashflowPoint; 6] = [
    MonthlyCashflowPoint {
        month: "Jun",
        income_pct: 68,
        expense_pct: 42,
        income_label: "680k",
        expense_label: "420k",
    },
    MonthlyCashflowPoint {
        month: "Jul",
        income_pct: 74,
        expense_pct: 48,
        income_label: "740k",
        expense_label: "480k",
    },
    MonthlyCashflowPoint {
        month: "Aug",
        income_pct: 64,
        expense_pct: 38,
        income_label: "640k",
        expense_label: "380k",
    },
    MonthlyCashflowPoint {
        month: "Sep",
        income_pct: 82,
        expense_pct: 51,
        income_label: "820k",
        expense_label: "510k",
    },
    MonthlyCashflowPoint {
        month: "Oct",
        income_pct: 79,
        expense_pct: 44,
        income_label: "790k",
        expense_label: "440k",
    },
    MonthlyCashflowPoint {
        month: "Nov",
        income_pct: 88,
        expense_pct: 41,
        income_label: "850k",
        expense_label: "410k",
    },
];

#[derive(Clone, Debug)]
pub struct ExpenseCategoryRow {
    pub name: SharedString,
    pub amount_rwf: i64,
    pub budget_rwf: i64,
    pub pct: u8,
    pub icon_name: &'static str,
    pub color: Rgba,
}

pub static EXPENSE_CATEGORIES: LazyLock<[ExpenseCategoryRow; 4]> = LazyLock::new(|| {
    [
        ExpenseCategoryRow {
            name: "Home & Groceries".into(),
            amount_rwf: 164_000,
            budget_rwf: 200_000,
            pct: 40,
            icon_name: icons::ICON_CART_SHOPPING,
            color: rgb(0xFFA69E),
        },
        ExpenseCategoryRow {
            name: "School & Kids".into(),
            amount_rwf: 115_000,
            budget_rwf: 120_000,
            pct: 28,
            icon_name: icons::ICON_GRADUATION_CAP,
            color: rgb(0xF9D67A),
        },
        ExpenseCategoryRow {
            name: "Ibimina Savings".into(),
            amount_rwf: 83_000,
            budget_rwf: 100_000,
            pct: 20,
            icon_name: icons::ICON_SEEDLING,
            color: rgb(0x8CE0B8),
        },
        ExpenseCategoryRow {
            name: "Power & Water".into(),
            amount_rwf: 48_000,
            budget_rwf: 60_000,
            pct: 12,
            icon_name: icons::ICON_BOLT,
            color: rgb(0xA9C9FF),
        },
    ]
});

#[derive(Clone, Debug)]
pub struct IncomeStreamRow {
    pub name: SharedString,
    pub cadence: SharedString,
    pub amount_rwf: i64,
    pub icon_name: &'static str,
}

pub static INCOME_STREAMS: LazyLock<[IncomeStreamRow; 4]> = LazyLock::new(|| {
    [
        IncomeStreamRow {
            name: "Agnes Ingabire Salary".into(),
            cadence: "Monthly · 1st".into(),
            amount_rwf: 450_000,
            icon_name: icons::ICON_BUILDING_COLUMNS,
        },
        IncomeStreamRow {
            name: "Bruno Design & Tech".into(),
            cadence: "Monthly · 10th".into(),
            amount_rwf: 320_000,
            icon_name: icons::ICON_WALLET,
        },
        IncomeStreamRow {
            name: "Kicukiro Rental Plot".into(),
            cadence: "Monthly · 15th".into(),
            amount_rwf: 50_000,
            icon_name: icons::ICON_HOUSE,
        },
        IncomeStreamRow {
            name: "Umuryango Ibimina Yield".into(),
            cadence: "Monthly Dividend".into(),
            amount_rwf: 30_000,
            icon_name: icons::ICON_SEEDLING,
        },
    ]
});

#[derive(Clone, Debug)]
pub struct LoanItem {
    pub id: SharedString,
    pub is_lent: bool,
    pub person: SharedString,
    pub reason: SharedString,
    pub total_rwf: i64,
    pub paid_rwf: i64,
    pub due: SharedString,
}

impl LoanItem {
    pub fn remaining_rwf(&self) -> i64 {
        (self.total_rwf - self.paid_rwf).max(0)
    }

    pub fn progress_pct(&self) -> u8 {
        if self.total_rwf <= 0 {
            100
        } else {
            ((self.paid_rwf * 100) / self.total_rwf).clamp(0, 100) as u8
        }
    }
}

pub static LOANS: LazyLock<[LoanItem; 3]> = LazyLock::new(|| {
    [
        LoanItem {
            id: "l1".into(),
            is_lent: true,
            person: "Cousin Jean-Paul".into(),
            reason: "Motorcycle Repair Bridge".into(),
            total_rwf: 200_000,
            paid_rwf: 120_000,
            due: "Dec 15, 2026".into(),
        },
        LoanItem {
            id: "l2".into(),
            is_lent: false,
            person: "Kigali Family Ibimina".into(),
            reason: "Solar Inverter Co-Op Advance".into(),
            total_rwf: 300_000,
            paid_rwf: 210_000,
            due: "Jan 10, 2027".into(),
        },
        LoanItem {
            id: "l3".into(),
            is_lent: true,
            person: "Auntie Clarisse".into(),
            reason: "Boutique Fabric Stock".into(),
            total_rwf: 150_000,
            paid_rwf: 80_000,
            due: "Dec 28, 2026".into(),
        },
    ]
});

#[derive(Clone, Debug)]
pub struct ApprovalItem {
    pub id: SharedString,
    pub requester: SharedString,
    pub title: SharedString,
    pub category: SharedString,
    pub amount_rwf: i64,
    pub votes: SharedString,
}

pub static APPROVALS: LazyLock<[ApprovalItem; 2]> = LazyLock::new(|| {
    [
        ApprovalItem {
            id: "ap1".into(),
            requester: "Papa Jean".into(),
            title: "Musanze Farm Fertilizer & Seeds".into(),
            category: "Projects · Garden".into(),
            amount_rwf: 75_000,
            votes: "1 of 2 Admins Approved".into(),
        },
        ApprovalItem {
            id: "ap2".into(),
            requester: "Keza".into(),
            title: "Science Club Robotics Kit".into(),
            category: "School & Kids".into(),
            amount_rwf: 35_000,
            votes: "1 of 2 Admins Approved".into(),
        },
    ]
});

#[derive(Clone, Debug)]
pub struct MemberContributionRow {
    pub id: SharedString,
    pub name: SharedString,
    pub role: SharedString,
    pub monthly_goal_rwf: i64,
    pub contributed_rwf: i64,
    pub status: SharedString,
    pub accent_color: Rgba,
}

impl MemberContributionRow {
    pub fn progress_pct(&self) -> u8 {
        if self.monthly_goal_rwf <= 0 {
            100
        } else {
            ((self.contributed_rwf * 100) / self.monthly_goal_rwf).clamp(0, 100) as u8
        }
    }
}

pub static FAMILY_MEMBERS: LazyLock<[MemberContributionRow; 4]> = LazyLock::new(|| {
    [
        MemberContributionRow {
            id: "m1".into(),
            name: "Bruno N.".into(),
            role: "Co-Admin · Family Planner".into(),
            monthly_goal_rwf: 250_000,
            contributed_rwf: 250_000,
            status: "Online".into(),
            accent_color: rgb(0x8CE0B8),
        },
        MemberContributionRow {
            id: "m2".into(),
            name: "Agnes Ingabire".into(),
            role: "Co-Admin · Treasury Keeper".into(),
            monthly_goal_rwf: 250_000,
            contributed_rwf: 250_000,
            status: "Online".into(),
            accent_color: rgb(0xFFA69E),
        },
        MemberContributionRow {
            id: "m3".into(),
            name: "Papa Jean".into(),
            role: "Elder Advisor · Musanze".into(),
            monthly_goal_rwf: 100_000,
            contributed_rwf: 85_000,
            status: "Active".into(),
            accent_color: rgb(0xF9D67A),
        },
        MemberContributionRow {
            id: "m4".into(),
            name: "Grandma Rose".into(),
            role: "Family Blessing · Gisenyi".into(),
            monthly_goal_rwf: 50_000,
            contributed_rwf: 50_000,
            status: "Blessed ♥".into(),
            accent_color: rgb(0xA9C9FF),
        },
    ]
});

pub fn format_rwf(amount_rwf: i64) -> String {
    format_money(amount_rwf * 100, "RWF")
}

pub fn format_signed_rwf(amount_rwf: i64, positive: bool) -> String {
    let base = format_rwf(amount_rwf.abs());
    if positive {
        format!("+{base}")
    } else {
        format!("-{base}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_boost_updates_saved_and_percentage_with_integer_math() {
        let mut proj = PROJECTS[0].clone();
        assert_eq!(proj.saved_rwf, 1_150_000);
        assert_eq!(proj.target_rwf, 1_500_000);
        assert_eq!(proj.pct, 77);

        proj.boost(50_000);
        assert_eq!(proj.saved_rwf, 1_200_000);
        assert_eq!(proj.pct, 80);
        assert!(!proj.is_completed);

        proj.boost(500_000);
        assert_eq!(proj.saved_rwf, 1_500_000);
        assert_eq!(proj.pct, 100);
        assert!(proj.is_completed);
    }

    #[test]
    fn loan_and_member_progress_compute_integer_percentages() {
        let loan = &LOANS[0];
        assert_eq!(loan.remaining_rwf(), 80_000);
        assert_eq!(loan.progress_pct(), 60);

        let member = &FAMILY_MEMBERS[2];
        assert_eq!(member.progress_pct(), 85);
    }
}
