#![allow(dead_code)]

use crate::components::ui::BadgeVariant;
use crate::i18n::{text, Language};
use crate::icons;
use gpui_kit::{rgb, Rgba, SharedString};
use std::collections::BTreeMap;
use umuryango_shared::{format_money, FamilyMemberDto, LoanDto, ProjectDto, TransactionItemDto};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntryModalKind {
    Income,
    Expense,
    Loan,
    Project,
    MemberInvite,
    Category,
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
            _ => (BadgeVariant::Neutral, "Recorded"),
        };
        let cat_lower = dto.category.to_lowercase();
        let is_loan = cat_lower.contains("loan");
        let category_variant = if is_loan {
            BadgeVariant::Warning
        } else if dto.is_income {
            BadgeVariant::Success
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
        let icon_name = category_icon(&dto.category, dto.is_income);

        Self {
            actor: dto.actor_name.clone().into(),
            title: dto.title.clone().into(),
            date: date_str.to_string().into(),
            amount: formatted_amount.into(),
            amount_rwf: dto.amount_minor_units,
            is_income: dto.is_income,
            is_loan,
            icon_name,
            category: dto.category.clone().into(),
            category_variant,
            status: status_label.into(),
            status_variant,
        }
    }
}

pub fn category_icon(category: &str, is_income: bool) -> &'static str {
    let lower = category.to_lowercase();
    if lower.contains("loan") {
        icons::ICON_HANDSHAKE
    } else if lower.contains("school") || lower.contains("educat") || lower.contains("tuition") {
        icons::ICON_GRADUATION_CAP
    } else if lower.contains("power") || lower.contains("water") || lower.contains("util") {
        icons::ICON_BOLT
    } else if lower.contains("farm") || lower.contains("garden") || lower.contains("ibimina") {
        icons::ICON_SEEDLING
    } else if lower.contains("project") || lower.contains("goal") {
        icons::ICON_BULLSEYE
    } else if lower.contains("house") || lower.contains("rent") {
        icons::ICON_HOUSE
    } else if is_income {
        icons::ICON_COINS
    } else {
        icons::ICON_CART_SHOPPING
    }
}

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
        let pct = dto.progress_percentage.clamp(0, 100);
        let is_completed = pct >= 100 || dto.status.eq_ignore_ascii_case("COMPLETED");
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
        let deadline_str = if is_completed {
            "Done".to_string()
        } else if dto.target_date.len() >= 10 {
            dto.target_date[..10].to_string()
        } else {
            "Active".into()
        };

        let (icon_name, fill_color) = match dto.category.to_uppercase().as_str() {
            "HOUSE" => (icons::ICON_HOUSE, rgb(0x8CE0B8)),
            "EDUCATION" => (icons::ICON_GRADUATION_CAP, rgb(0xF9D67A)),
            "BUSINESS" => (icons::ICON_SEEDLING, rgb(0xFFA69E)),
            "VEHICLE" => (icons::ICON_BOLT, rgb(0xA9C9FF)),
            _ => {
                if is_completed {
                    (icons::ICON_CIRCLE_CHECK, rgb(0xA9C9FF))
                } else {
                    (icons::ICON_BULLSEYE, rgb(0x8CE0B8))
                }
            }
        };

        let lead = dto
            .description
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or(&dto.category);

        Self {
            id: dto.id.clone().into(),
            name: dto.name.clone().into(),
            lead: lead.to_string().into(),
            target: target_str.into(),
            progress: progress_str.into(),
            contributors: contrib_str.into(),
            deadline: deadline_str.into(),
            saved_rwf: dto.raised_amount_minor_units,
            target_rwf: dto.target_amount_minor_units,
            pct,
            is_completed,
            icon_name,
            fill_color,
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
            self.target = format!(
                "Target: {} • Raised: {}",
                format_rwf(self.target_rwf),
                format_rwf(self.saved_rwf)
            )
            .into();
        }
    }
}

#[derive(Clone, Debug)]
pub struct MonthlyCashflowPoint {
    pub month: SharedString,
    pub income_pct: u8,
    pub expense_pct: u8,
    pub income_label: SharedString,
    pub expense_label: SharedString,
}

#[derive(Clone, Debug)]
pub struct ExpenseCategoryRow {
    pub name: SharedString,
    pub amount_rwf: i64,
    pub budget_rwf: i64,
    pub pct: u8,
    pub icon_name: &'static str,
    pub color: Rgba,
    pub emoji: &'static str,
}

pub fn category_emoji(category: &str) -> &'static str {
    let lower = category.to_lowercase();
    if lower.contains("grocer") || lower.contains("food") || lower.contains("market") {
        "🥕"
    } else if lower.contains("school") || lower.contains("educat") || lower.contains("kid") {
        "🎒"
    } else if lower.contains("save") || lower.contains("ibimina") || lower.contains("invest") {
        "🐷"
    } else if lower.contains("power") || lower.contains("water") || lower.contains("util") {
        "💡"
    } else if lower.contains("farm") || lower.contains("garden") {
        "🌱"
    } else if lower.contains("health") || lower.contains("med") {
        "🩺"
    } else {
        "📦"
    }
}
#[derive(Clone, Debug)]
pub struct IncomeStreamRow {
    pub name: SharedString,
    pub cadence: SharedString,
    pub amount_rwf: i64,
    pub icon_name: &'static str,
}

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
    pub fn from_dto(dto: &LoanDto) -> Self {
        let due_str = if dto.due_date.len() >= 10 {
            dto.due_date[..10].to_string()
        } else {
            dto.due_date.clone()
        };
        Self {
            id: dto.id.clone().into(),
            is_lent: dto.is_lent,
            person: dto.counterparty_name.clone().into(),
            reason: dto.title.clone().into(),
            total_rwf: dto.total_amount_minor_units,
            paid_rwf: dto.paid_amount_minor_units,
            due: due_str.into(),
        }
    }

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

#[derive(Clone, Debug)]
pub struct ApprovalItem {
    pub id: SharedString,
    pub requester: SharedString,
    pub title: SharedString,
    pub category: SharedString,
    pub amount_rwf: i64,
    pub votes: SharedString,
}

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
    pub fn from_dto(
        dto: &FamilyMemberDto,
        index: usize,
        transactions: &[TransactionItemDto],
    ) -> Self {
        let contributed_rwf: i64 = transactions
            .iter()
            .filter(|t| {
                t.is_income
                    && t.status == "APPROVED"
                    && t.actor_name.eq_ignore_ascii_case(&dto.full_name)
            })
            .map(|t| t.amount_minor_units)
            .sum();

        let (role_label, default_goal, status_label) = match dto.role.as_str() {
            "HEAD" => ("Family Head · Co-Admin", 250_000, "Admin"),
            "ADMIN" | "TREASURER" => ("Co-Admin · Treasury Keeper", 200_000, "Admin"),
            "ELDER" => ("Elder Advisor", 100_000, "Active"),
            _ => ("Family Member", 100_000, "Active"),
        };
        let monthly_goal_rwf = default_goal.max(contributed_rwf);
        let swatches = [rgb(0x8CE0B8), rgb(0xFFA69E), rgb(0xF9D67A), rgb(0xA9C9FF)];
        let accent_color = swatches[index % swatches.len()];

        Self {
            id: dto.id.clone().into(),
            name: dto.full_name.clone().into(),
            role: role_label.into(),
            monthly_goal_rwf,
            contributed_rwf,
            status: status_label.into(),
            accent_color,
        }
    }

    pub fn progress_pct(&self) -> u8 {
        if self.monthly_goal_rwf <= 0 {
            100
        } else {
            ((self.contributed_rwf * 100) / self.monthly_goal_rwf).clamp(0, 100) as u8
        }
    }
}

/// Derives active income streams dynamically from backend transactions.
pub fn derive_income_streams(transactions: &[TransactionItemDto]) -> Vec<IncomeStreamRow> {
    let mut grouped: BTreeMap<String, (String, i64)> = BTreeMap::new();
    for tx in transactions
        .iter()
        .filter(|t| t.is_income && t.status == "APPROVED")
    {
        let key = format!("{} · {}", tx.actor_name, tx.category);
        let entry = grouped
            .entry(key)
            .or_insert_with(|| (tx.category.clone(), 0));
        entry.1 += tx.amount_minor_units;
    }

    grouped
        .into_iter()
        .map(|(name, (category, amount_rwf))| IncomeStreamRow {
            name: name.into(),
            cadence: format!("{category} · Recorded").into(),
            amount_rwf,
            icon_name: category_icon(&category, true),
        })
        .collect()
}

/// Derives expense category breakdown dynamically from backend transactions.
pub fn derive_expense_categories(transactions: &[TransactionItemDto]) -> Vec<ExpenseCategoryRow> {
    let mut grouped: BTreeMap<String, i64> = BTreeMap::new();
    let mut total_expenses: i64 = 0;

    for tx in transactions
        .iter()
        .filter(|t| !t.is_income && t.status == "APPROVED")
    {
        *grouped.entry(tx.category.clone()).or_insert(0) += tx.amount_minor_units;
        total_expenses += tx.amount_minor_units;
    }

    let swatches = [rgb(0xFFA69E), rgb(0xF9D67A), rgb(0x8CE0B8), rgb(0xA9C9FF)];

    grouped
        .into_iter()
        .enumerate()
        .map(|(idx, (name, amount_rwf))| {
            let pct = if total_expenses > 0 {
                ((amount_rwf * 100) / total_expenses).clamp(1, 100) as u8
            } else {
                0
            };
            let budget_rwf = ((amount_rwf * 120) / 100).max(50_000);
            let icon_name = category_icon(&name, false);
            ExpenseCategoryRow {
                name: name.clone().into(),
                amount_rwf,
                budget_rwf,
                pct,
                icon_name,
                color: swatches[idx % swatches.len()],
                emoji: category_emoji(&name),
            }
        })
        .collect()
}

/// Derives monthly cashflow chart points dynamically from backend transactions.
pub fn derive_monthly_cashflow(transactions: &[TransactionItemDto]) -> Vec<MonthlyCashflowPoint> {
    let mut monthly: BTreeMap<String, (i64, i64)> = BTreeMap::new();

    for tx in transactions.iter().filter(|t| t.status == "APPROVED") {
        let ym = if tx.occurred_at.len() >= 7 {
            tx.occurred_at[..7].to_string()
        } else {
            "Current".to_string()
        };
        let entry = monthly.entry(ym).or_insert((0, 0));
        if tx.is_income {
            entry.0 += tx.amount_minor_units;
        } else {
            entry.1 += tx.amount_minor_units;
        }
    }

    if monthly.is_empty() {
        return vec![MonthlyCashflowPoint {
            month: "Now".into(),
            income_pct: 15,
            expense_pct: 12,
            income_label: "0".into(),
            expense_label: "0".into(),
        }];
    }

    let max_val = monthly
        .values()
        .flat_map(|(inc, exp)| [*inc, *exp])
        .max()
        .unwrap_or(1)
        .max(1);

    monthly
        .into_iter()
        .rev()
        .take(6)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .map(|(ym, (inc, exp))| {
            let month_label = month_short_label(&ym);
            let income_pct = ((inc * 92) / max_val).clamp(15, 96) as u8;
            let expense_pct = ((exp * 92) / max_val).clamp(12, 96) as u8;
            MonthlyCashflowPoint {
                month: month_label.into(),
                income_pct,
                expense_pct,
                income_label: format_compact_rwf(inc).into(),
                expense_label: format_compact_rwf(exp).into(),
            }
        })
        .collect()
}

fn month_short_label(ym: &str) -> String {
    if ym.len() >= 7 {
        match &ym[5..7] {
            "01" => "Jan".into(),
            "02" => "Feb".into(),
            "03" => "Mar".into(),
            "04" => "Apr".into(),
            "05" => "May".into(),
            "06" => "Jun".into(),
            "07" => "Jul".into(),
            "08" => "Aug".into(),
            "09" => "Sep".into(),
            "10" => "Oct".into(),
            "11" => "Nov".into(),
            "12" => "Dec".into(),
            other => other.to_string(),
        }
    } else {
        ym.to_string()
    }
}

pub fn format_rwf(amount_rwf: i64) -> String {
    format_money(amount_rwf, "RWF")
}

pub fn format_signed_rwf(amount_rwf: i64, positive: bool) -> String {
    let base = format_rwf(amount_rwf.abs());
    if positive {
        format!("+{base}")
    } else {
        format!("-{base}")
    }
}

pub fn format_compact_rwf(amount_rwf: i64) -> String {
    let abs = amount_rwf.abs();
    if abs >= 1_000 {
        format!("{}k", abs / 1_000)
    } else {
        abs.to_string()
    }
}

pub fn format_compact_signed_rwf(amount_rwf: i64, positive: bool) -> String {
    let compact = format_compact_rwf(amount_rwf);
    if positive {
        format!("+{compact}")
    } else {
        format!("-{compact}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_boost_updates_saved_and_percentage_with_integer_math() {
        let dto = ProjectDto {
            id: "p1".into(),
            family_id: "f1".into(),
            name: "Gisenyi Solar Roof".into(),
            category: "HOUSE".into(),
            description: Some("Bruno & Papa".into()),
            target_amount_minor_units: 1_500_000,
            raised_amount_minor_units: 1_150_000,
            currency: "RWF".into(),
            progress_percentage: 77,
            contributors_count: 4,
            status: "ACTIVE".into(),
            target_date: "2026-12-31".into(),
        };
        let mut proj = ProjectSummary::from_dto(&dto);
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
        let loan = LoanItem::from_dto(&LoanDto {
            id: "l1".into(),
            family_id: "f1".into(),
            title: "Motorcycle Repair Bridge".into(),
            counterparty_name: "Cousin Jean-Paul".into(),
            is_lent: true,
            total_amount_minor_units: 200_000,
            paid_amount_minor_units: 120_000,
            remaining_amount_minor_units: 80_000,
            currency: "RWF".into(),
            status: "ACTIVE".into(),
            due_date: "2026-12-15".into(),
        });
        assert_eq!(loan.remaining_rwf(), 80_000);
        assert_eq!(loan.progress_pct(), 60);

        let txs = vec![TransactionItemDto {
            id: "t1".into(),
            family_id: "f1".into(),
            actor_name: "Papa Jean".into(),
            title: "Monthly Contribution".into(),
            amount_minor_units: 85_000,
            currency: "RWF".into(),
            is_income: true,
            category: "Contribution".into(),
            status: "APPROVED".into(),
            occurred_at: "2026-11-10".into(),
            notes: None,
        }];
        let member = MemberContributionRow::from_dto(
            &FamilyMemberDto {
                id: "m3".into(),
                user_id: "u3".into(),
                full_name: "Papa Jean".into(),
                phone_number: None,
                email: None,
                role: "ELDER".into(),
                avatar_url: None,
                joined_at: "2026-11-01".into(),
            },
            0,
            &txs,
        );
        assert_eq!(member.contributed_rwf, 85_000);
        assert_eq!(member.progress_pct(), 85);
    }
}
