use crate::components::ui::BadgeVariant;
use gpui_kit::SharedString;
use std::sync::LazyLock;
use umuryango_shared::{format_money, ProjectDto, TransactionItemDto};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionRow {
    pub actor: SharedString,
    pub title: SharedString,
    pub date: SharedString,
    pub amount: SharedString,
    pub is_income: bool,
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
        Self {
            actor: dto.actor_name.clone().into(),
            title: dto.title.clone().into(),
            date: date_str.to_string().into(),
            amount: formatted_amount.into(),
            is_income: dto.is_income,
            category: dto.category.clone().into(),
            category_variant,
            status: status_label.into(),
            status_variant,
        }
    }
}

pub static TRANSACTIONS: LazyLock<[TransactionRow; 5]> = LazyLock::new(|| {
    [
        TransactionRow {
            actor: "Keza Alice".into(),
            title: "Monthly Contribution".into(),
            date: "Today, 10:30 AM".into(),
            amount: "+50,000 RWF".into(),
            is_income: true,
            category: "Contribution".into(),
            category_variant: BadgeVariant::Primary,
            status: "Approved".into(),
            status_variant: BadgeVariant::Success,
        },
        TransactionRow {
            actor: "Bruno Mugabo".into(),
            title: "School Fees Q1 - Junior".into(),
            date: "Yesterday, 3:15 PM".into(),
            amount: "-120,000 RWF".into(),
            is_income: false,
            category: "Education".into(),
            category_variant: BadgeVariant::Neutral,
            status: "Approved".into(),
            status_variant: BadgeVariant::Success,
        },
        TransactionRow {
            actor: "Eric Mugabo".into(),
            title: "Medical Emergency Assistance".into(),
            date: "Nov 16, 2026".into(),
            amount: "-85,000 RWF".into(),
            is_income: false,
            category: "Health".into(),
            category_variant: BadgeVariant::Neutral,
            status: "Pending Approval".into(),
            status_variant: BadgeVariant::Warning,
        },
        TransactionRow {
            actor: "Claudine Mugabo".into(),
            title: "Agricultural Inputs - Seeds".into(),
            date: "Nov 14, 2026".into(),
            amount: "-65,000 RWF".into(),
            is_income: false,
            category: "Farming".into(),
            category_variant: BadgeVariant::Neutral,
            status: "Approved".into(),
            status_variant: BadgeVariant::Success,
        },
        TransactionRow {
            actor: "Bruno Mugabo".into(),
            title: "Monthly Contribution".into(),
            date: "Nov 12, 2026".into(),
            amount: "+100,000 RWF".into(),
            is_income: true,
            category: "Contribution".into(),
            category_variant: BadgeVariant::Primary,
            status: "Approved".into(),
            status_variant: BadgeVariant::Success,
        },
    ]
});

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectSummary {
    pub name: SharedString,
    pub target: SharedString,
    pub progress: SharedString,
    pub contributors: SharedString,
    pub deadline: SharedString,
    pub variant: BadgeVariant,
}

impl ProjectSummary {
    pub fn from_dto(dto: &ProjectDto) -> Self {
        let variant = if dto.progress_percentage >= 70 {
            BadgeVariant::Success
        } else {
            BadgeVariant::Primary
        };
        let target_str = format!(
            "Target: {} • Raised: {}",
            format_money(dto.target_amount_minor_units, &dto.currency),
            format_money(dto.raised_amount_minor_units, &dto.currency)
        );
        let progress_str = format!("{}% funded", dto.progress_percentage);
        let contrib_str = format!("{} contributors", dto.contributors_count);
        let deadline_str = if dto.target_date.len() >= 10 {
            format!("Due {}", &dto.target_date[..10])
        } else {
            "Active".into()
        };

        Self {
            name: dto.name.clone().into(),
            target: target_str.into(),
            progress: progress_str.into(),
            contributors: contrib_str.into(),
            deadline: deadline_str.into(),
            variant,
        }
    }
}

pub static PROJECTS: LazyLock<[ProjectSummary; 2]> = LazyLock::new(|| {
    [
        ProjectSummary {
            name: "Family Land Purchase (Bugesera)".into(),
            target: "Target: 5,000,000 RWF • Raised: 3,750,000 RWF".into(),
            progress: "75% funded".into(),
            contributors: "4 contributors".into(),
            deadline: "Closes in 45 days".into(),
            variant: BadgeVariant::Primary,
        },
        ProjectSummary {
            name: "Holiday Reunion & Feast 2026".into(),
            target: "Target: 800,000 RWF • Raised: 600,000 RWF".into(),
            progress: "75% funded".into(),
            contributors: "8 contributors".into(),
            deadline: "Closes in 20 days".into(),
            variant: BadgeVariant::Success,
        },
    ]
});
