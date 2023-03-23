#[derive(Debug, seaography::macros::QueryRoot)]
// #[seaography(entity = "crate::entities::aggregate_daily_app_name_metrics")]
// #[seaography(entity = "crate::entities::aggregate_daily_total_users_metrics")]
// #[seaography(entity = "crate::entities::aggregate_daily_unique_users_metrics")]
// #[seaography(entity = "crate::entities::aggregate_monthly_app_name_metrics")]
// #[seaography(entity = "crate::entities::aggregate_monthly_plays")]
// #[seaography(entity = "crate::entities::aggregate_monthly_total_users_metrics")]
// #[seaography(entity = "crate::entities::aggregate_monthly_unique_users_metrics")]
// #[seaography(entity = "crate::entities::aggregate_playlist")]
// #[seaography(entity = "crate::entities::aggregate_plays")]
// #[seaography(entity = "crate::entities::aggregate_track")]
// #[seaography(entity = "crate::entities::aggregate_user")]
// #[seaography(entity = "crate::entities::aggregate_user_tips")]
// #[seaography(entity = "crate::entities::alembic_version")]
// #[seaography(entity = "crate::entities::app_name_metrics")]
// #[seaography(entity = "crate::entities::associated_wallets")]
// #[seaography(entity = "crate::entities::audio_transactions_history")]
// #[seaography(entity = "crate::entities::audius_data_txs")]
// #[seaography(entity = "crate::entities::blocks")]
// #[seaography(entity = "crate::entities::blocks_copy")]
// #[seaography(entity = "crate::entities::challenge_disbursements")]
// #[seaography(entity = "crate::entities::challenge_listen_streak")]
// #[seaography(entity = "crate::entities::challenge_profile_completion")]
// #[seaography(entity = "crate::entities::challenges")]
// #[seaography(entity = "crate::entities::chat")]
// #[seaography(entity = "crate::entities::chat_blocked_users")]
// #[seaography(entity = "crate::entities::chat_member")]
// #[seaography(entity = "crate::entities::chat_message")]
// #[seaography(entity = "crate::entities::chat_message_reactions")]
// #[seaography(entity = "crate::entities::chat_permissions")]
// #[seaography(entity = "crate::entities::cid_data")]
// #[seaography(entity = "crate::entities::eth_blocks")]
// #[seaography(entity = "crate::entities::follows")]
// #[seaography(entity = "crate::entities::hourly_play_counts")]
// #[seaography(entity = "crate::entities::indexing_checkpoints")]
// #[seaography(entity = "crate::entities::milestones")]
// #[seaography(entity = "crate::entities::notification")]
// #[seaography(entity = "crate::entities::notification_seen")]
// #[seaography(entity = "crate::entities::playlist_routes")]
// #[seaography(entity = "crate::entities::playlist_seen")]
// #[seaography(entity = "crate::entities::playlists")]
// #[seaography(entity = "crate::entities::plays")]
// #[seaography(entity = "crate::entities::plays_archive")]
// #[seaography(entity = "crate::entities::reactions")]
// #[seaography(entity = "crate::entities::related_artists")]
// #[seaography(entity = "crate::entities::remixes")]
// #[seaography(entity = "crate::entities::reposts")]
// #[seaography(entity = "crate::entities::reward_manager_txs")]
// #[seaography(entity = "crate::entities::rewards_manager_backfill_txs")]
// #[seaography(entity = "crate::entities::route_metrics")]
// #[seaography(entity = "crate::entities::rpc_log")]
// #[seaography(entity = "crate::entities::saves")]
// #[seaography(entity = "crate::entities::schema_migrations")]
// #[seaography(entity = "crate::entities::skipped_transactions")]
// #[seaography(entity = "crate::entities::spl_token_backfill_txs")]
// #[seaography(entity = "crate::entities::spl_token_tx")]
// #[seaography(entity = "crate::entities::stems")]
// #[seaography(entity = "crate::entities::subscriptions")]
// #[seaography(entity = "crate::entities::supporter_rank_ups")]
// #[seaography(entity = "crate::entities::track_routes")]
// #[seaography(entity = "crate::entities::track_trending_scores")]
// #[seaography(entity = "crate::entities::tracks")]
// #[seaography(entity = "crate::entities::trending_results")]
// #[seaography(entity = "crate::entities::ursm_content_nodes")]
// #[seaography(entity = "crate::entities::user_balance_changes")]
// #[seaography(entity = "crate::entities::user_balances")]
// #[seaography(entity = "crate::entities::user_bank_accounts")]
// #[seaography(entity = "crate::entities::user_bank_backfill_txs")]
// #[seaography(entity = "crate::entities::user_bank_txs")]
// #[seaography(entity = "crate::entities::user_challenges")]
// #[seaography(entity = "crate::entities::user_events")]
// #[seaography(entity = "crate::entities::user_listening_history")]
// #[seaography(entity = "crate::entities::user_tips")]
#[seaography(entity = "crate::entities::users")]
pub struct QueryRoot;
