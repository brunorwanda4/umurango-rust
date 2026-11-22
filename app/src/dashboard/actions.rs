use super::*;

impl DashboardView {
    pub fn boost_project(&mut self, index: usize, amount_rwf: i64, cx: &mut Context<Self>) {
        let Some((project_id, proj_name)) = self
            .projects
            .get(index)
            .map(|p| (p.id.to_string(), p.name.to_string()))
        else {
            return;
        };

        if let Some(fid) = crate::auth::client::get_current_family_id() {
            let req = umuryango_shared::ProjectContributionRequest {
                amount_minor_units: amount_rwf,
                note: Some(format!("Boosted {proj_name}")),
            };
            match crate::auth::client::api_contribute_project(&fid, &project_id, &req) {
                Ok(_) => {
                    let tx_req = umuryango_shared::CreateTransactionRequest {
                        amount_minor_units: amount_rwf,
                        is_income: true,
                        category: "Project Boost".to_string(),
                        description: format!("Boosted {proj_name}"),
                        notes: None,
                        idempotency_key: Some(uuid::Uuid::new_v4().to_string()),
                    };
                    let _ = crate::auth::client::api_create_transaction(&fid, &tx_req);
                    self.toast_success(
                        format!("Boosted {proj_name}!"),
                        Some(format!("Contributed {}", format_rwf(amount_rwf))),
                        cx,
                    );
                    self.toast_notice = Some(format!(
                        "{} {proj_name}!",
                        text(self.language, "cozy.toast_boosted")
                    ));
                    self.refresh_from_backend(cx);
                }
                Err(err) => {
                    self.toast_error("Project Boost Failed", Some(err.clone()), cx);
                    self.toast_notice = Some(err);
                    cx.notify();
                }
            }
        }
    }

    pub fn record_loan_repayment(&mut self, index: usize, amount_rwf: i64, cx: &mut Context<Self>) {
        let Some((loan_id, person, remaining)) = self
            .loans
            .get(index)
            .map(|l| (l.id.to_string(), l.person.to_string(), l.remaining_rwf()))
        else {
            return;
        };

        let step = amount_rwf.min(remaining);
        if step <= 0 {
            self.toast_info(
                "Loan Already Settled",
                Some(format!("{person} has no remaining balance")),
                cx,
            );
            self.toast_notice = Some(format!(
                "{} {person}",
                text(self.language, "cozy.toast_loan_settled")
            ));
            cx.notify();
            return;
        }

        if let Some(fid) = crate::auth::client::get_current_family_id() {
            let req = umuryango_shared::LoanRepaymentRequest {
                amount_minor_units: step,
                note: Some(format!("Loan Repayment · {person}")),
            };
            match crate::auth::client::api_repay_loan(&fid, &loan_id, &req) {
                Ok(_) => {
                    self.toast_success(
                        format!("Loan Repaid · {person}"),
                        Some(format!("Repaid {}", format_rwf(step))),
                        cx,
                    );
                    self.toast_notice = Some(format!(
                        "{} {person} ({})!",
                        text(self.language, "cozy.toast_loan_repaid"),
                        format_rwf(step)
                    ));
                    self.refresh_from_backend(cx);
                }
                Err(err) => {
                    self.toast_error("Loan Repayment Failed", Some(err.clone()), cx);
                    self.toast_notice = Some(err);
                    cx.notify();
                }
            }
        }
    }

    pub fn resolve_approval(&mut self, index: usize, approve: bool, cx: &mut Context<Self>) {
        let Some(item) = self.approvals.get(index).cloned() else {
            return;
        };
        let tx_id = item.id.to_string();

        if let Some(fid) = crate::auth::client::get_current_family_id() {
            let res = if approve {
                crate::auth::client::api_approve_transaction(&fid, &tx_id)
            } else {
                crate::auth::client::api_reject_transaction(&fid, &tx_id)
            };

            match res {
                Ok(_) => {
                    if approve {
                        self.toast_success(
                            "Transaction Approved",
                            Some(format!("{} · {}", item.title, format_rwf(item.amount_rwf))),
                            cx,
                        );
                    } else {
                        self.toast_warning(
                            "Transaction Declined",
                            Some(format!("Declined: {}", item.title)),
                            cx,
                        );
                    }
                    self.toast_notice = if approve {
                        Some(format!(
                            "{} {} ({})",
                            text(self.language, "cozy.toast_approved"),
                            item.title,
                            format_rwf(item.amount_rwf)
                        ))
                    } else {
                        Some(format!(
                            "{} {}",
                            text(self.language, "cozy.toast_declined"),
                            item.title
                        ))
                    };
                    self.refresh_from_backend(cx);
                }
                Err(err) => {
                    self.toast_error("Approval Action Failed", Some(err.clone()), cx);
                    self.toast_notice = Some(err);
                    cx.notify();
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn log_member_contribution(
        &mut self,
        index: usize,
        amount_rwf: i64,
        cx: &mut Context<Self>,
    ) {
        let Some(name) = self.family_members.get(index).map(|m| m.name.to_string()) else {
            return;
        };

        if let Some(fid) = crate::auth::client::get_current_family_id() {
            let req = umuryango_shared::CreateTransactionRequest {
                amount_minor_units: amount_rwf,
                is_income: true,
                category: "Contribution".to_string(),
                description: format!("{name} Family Circle Pledge"),
                notes: Some(format!("Recorded for {name}")),
                idempotency_key: Some(uuid::Uuid::new_v4().to_string()),
            };
            match crate::auth::client::api_create_transaction(&fid, &req) {
                Ok(_) => {
                    self.toast_success(
                        "Pledge Recorded",
                        Some(format!("Recorded {} for {name}", format_rwf(amount_rwf))),
                        cx,
                    );
                    self.toast_notice = Some(format!(
                        "{} {name}!",
                        text(self.language, "cozy.toast_member_contrib")
                    ));
                    self.refresh_from_backend(cx);
                }
                Err(err) => {
                    self.toast_error("Failed to Record Pledge", Some(err.clone()), cx);
                    self.toast_notice = Some(err);
                    cx.notify();
                }
            }
        }
    }

    pub fn refresh_from_backend(&mut self, cx: &mut Context<Self>) {
        // 1. Refresh authenticated user profile
        let current_user = crate::auth::client::get_current_user()
            .or_else(|| crate::auth::client::api_get_me().ok());
        if let Some(ref user) = current_user {
            if !user.full_name.trim().is_empty() {
                self.member_name = user.full_name.clone();
            }
            if let Some(ref email) = user.email {
                self.member_email = email.trim().to_string();
            }
            if let Some(ref phone) = user.phone_number {
                self.member_phone = phone.trim().to_string();
            }
            let mut contact_parts = Vec::new();
            if let Some(ref phone) = user.phone_number {
                if !phone.trim().is_empty() {
                    contact_parts.push(phone.trim().to_string());
                }
            }
            if let Some(ref email) = user.email {
                if !email.trim().is_empty() {
                    contact_parts.push(email.trim().to_string());
                }
            }
            self.member_contact = contact_parts.join(" · ");
        }

        if let Some(role) = crate::auth::client::get_current_role() {
            self.member_role = match role.as_str() {
                "HEAD" => "Family Head · Co-Admin".to_string(),
                "ADMIN" => "Co-Admin · Treasury Keeper".to_string(),
                _ => "Family Member".to_string(),
            };
        }

        // 2. Resolve active family workspace
        let family_id = crate::auth::client::get_current_family_id().or_else(|| {
            if let Ok(fams) = crate::auth::client::api_list_families() {
                if let Some(first) = fams.first() {
                    crate::auth::client::set_current_family(
                        Some(first.id.to_string()),
                        Some(first.name.clone()),
                    );
                    return Some(first.id.to_string());
                }
            }
            None
        });

        if let Some(ref fam_name) = crate::auth::client::get_current_family_name() {
            self.family_name = fam_name.clone();
        }

        if let Some(ref fid) = family_id {
            // 3. Authoritative treasury summary
            if let Ok(summary) = crate::auth::client::api_get_treasury_summary(fid) {
                self.treasury_balance_rwf = summary.total_treasury_minor_units;
                self.monthly_income_rwf = summary.monthly_contributions_minor_units;
                self.monthly_expenses_rwf = summary.monthly_expenses_minor_units;
                self.total_treasury = umuryango_shared::format_money(
                    summary.total_treasury_minor_units,
                    &summary.currency,
                );
                self.monthly_contributions = umuryango_shared::format_money(
                    summary.monthly_contributions_minor_units,
                    &summary.currency,
                );
                self.monthly_expenses = umuryango_shared::format_money(
                    summary.monthly_expenses_minor_units,
                    &summary.currency,
                );
                self.emergency_reserve = umuryango_shared::format_money(
                    summary.emergency_reserve_minor_units,
                    &summary.currency,
                );
                self.pending_approval_count = summary.pending_approval_count;
            }

            // 4. Authoritative transactions + dynamic income streams, expense categories, cashflow, and approvals
            let mut raw_txs = Vec::new();
            if let Ok(txs) = crate::auth::client::api_list_transactions(fid, None) {
                let pending_items: Vec<ApprovalItem> = txs
                    .iter()
                    .filter(|t| t.status == "PENDING")
                    .map(|p| ApprovalItem {
                        id: p.id.clone().into(),
                        requester: p.actor_name.clone().into(),
                        title: p.title.clone().into(),
                        category: p.category.clone().into(),
                        amount_rwf: p.amount_minor_units,
                        votes: "Pending Admin Approval".into(),
                    })
                    .collect();

                self.pending_approval_count = pending_items.len();
                if let Some(first) = pending_items.first() {
                    self.pending_approval_tx_id = Some(first.id.to_string());
                    self.pending_approval_tx_title = Some(first.title.to_string());
                    self.pending_approval_tx_amount = Some(format_rwf(first.amount_rwf));
                    self.show_approval_alert = true;
                } else {
                    self.pending_approval_tx_id = None;
                    self.pending_approval_tx_title = None;
                    self.pending_approval_tx_amount = None;
                    self.show_approval_alert = false;
                }
                self.approvals = pending_items;

                self.income_streams = derive_income_streams(&txs);
                self.expense_categories = derive_expense_categories(&txs);
                self.monthly_cashflow = derive_monthly_cashflow(&txs);
                self.transactions = txs.iter().map(TransactionRow::from_dto).collect();
                raw_txs = txs;
            }

            // 5. Authoritative communal projects
            if let Ok(projs) = crate::auth::client::api_list_projects(fid) {
                self.projects = projs.iter().map(ProjectSummary::from_dto).collect();
            }

            // 6. Authoritative family loans
            if let Ok(loans) = crate::auth::client::api_list_loans(fid) {
                self.loans = loans.iter().map(LoanItem::from_dto).collect();
            }

            // 7. Authoritative family members roster
            if let Ok(mems) = crate::auth::client::api_list_members(fid) {
                if let Some(ref user) = current_user {
                    if let Some(me_member) = mems.iter().find(|m| m.user_id == user.id) {
                        self.member_role = match me_member.role.as_str() {
                            "HEAD" => "Family Head · Co-Admin".to_string(),
                            "ADMIN" | "TREASURER" => "Co-Admin · Treasury Keeper".to_string(),
                            "ELDER" => "Elder Advisor".to_string(),
                            _ => "Family Member".to_string(),
                        };
                    }
                }

                self.family_members = mems
                    .iter()
                    .enumerate()
                    .map(|(idx, m)| MemberContributionRow::from_dto(m, idx, &raw_txs))
                    .collect();

                let base_fam_name = crate::auth::client::get_current_family_name()
                    .unwrap_or_else(|| self.family_name.clone());
                if !base_fam_name.is_empty() {
                    self.family_name = format!("{} · {} Members", base_fam_name, mems.len());
                }

                self.members = mems
                    .into_iter()
                    .map(|m| {
                        let role_desc = match m.role.as_str() {
                            "HEAD" => "Family Head / Admin",
                            "ADMIN" | "TREASURER" => "Treasurer / Admin",
                            "ELDER" => "Elder",
                            "YOUTH" => "Youth Member",
                            _ => "Member",
                        };
                        let variant = match m.role.as_str() {
                            "HEAD" => BadgeVariant::Primary,
                            "ADMIN" | "TREASURER" => BadgeVariant::Success,
                            "ELDER" => BadgeVariant::Warning,
                            "YOUTH" => BadgeVariant::Info,
                            _ => BadgeVariant::Neutral,
                        };
                        (m.full_name, role_desc.to_string(), variant)
                    })
                    .collect();
            }
        }
        cx.notify();
    }

    pub fn sync_preferences(
        &mut self,
        theme: ThemeMode,
        language: Language,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.theme != theme {
            self.set_theme(theme, cx);
        }
        if self.language != language {
            self.set_language(language, window, cx);
        }
    }
}
