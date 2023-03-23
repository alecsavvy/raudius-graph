
    use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
    use sea_orm::{DatabaseConnection, DbErr, EntityTrait};
    use crate::entities::{prelude::*, *};

    pub type AudiusSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

    pub struct QueryRoot;

    #[Object]
    impl QueryRoot {
        async fn howdy(&self) -> &'static str {
            "partner 🤠"
        }
    
        async fn user_listening_history(&self, ctx: &Context<'_>) -> Result<Vec<user_listening_history::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            UserListeningHistory::find().all(db).await
        }

    
        async fn follows(&self, ctx: &Context<'_>) -> Result<Vec<follows::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            Follows::find().all(db).await
        }

    
        async fn blocks_copy(&self, ctx: &Context<'_>) -> Result<Vec<blocks_copy::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            BlocksCopy::find().all(db).await
        }

    
        async fn chat_message(&self, ctx: &Context<'_>) -> Result<Vec<chat_message::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            ChatMessage::find().all(db).await
        }

    
        async fn challenge_profile_completion(&self, ctx: &Context<'_>) -> Result<Vec<challenge_profile_completion::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            ChallengeProfileCompletion::find().all(db).await
        }

    
        async fn chat_blocked_users(&self, ctx: &Context<'_>) -> Result<Vec<chat_blocked_users::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            ChatBlockedUsers::find().all(db).await
        }

    
        async fn user_balances(&self, ctx: &Context<'_>) -> Result<Vec<user_balances::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            UserBalances::find().all(db).await
        }

    
        async fn ursm_content_nodes(&self, ctx: &Context<'_>) -> Result<Vec<ursm_content_nodes::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            UrsmContentNodes::find().all(db).await
        }

    
        async fn chat_member(&self, ctx: &Context<'_>) -> Result<Vec<chat_member::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            ChatMember::find().all(db).await
        }

    
        async fn milestones(&self, ctx: &Context<'_>) -> Result<Vec<milestones::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            Milestones::find().all(db).await
        }

    
        async fn notification(&self, ctx: &Context<'_>) -> Result<Vec<notification::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            Notification::find().all(db).await
        }

    
        async fn related_artists(&self, ctx: &Context<'_>) -> Result<Vec<related_artists::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            RelatedArtists::find().all(db).await
        }

    
        async fn reactions(&self, ctx: &Context<'_>) -> Result<Vec<reactions::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            Reactions::find().all(db).await
        }

    
        async fn chat_message_reactions(&self, ctx: &Context<'_>) -> Result<Vec<chat_message_reactions::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            ChatMessageReactions::find().all(db).await
        }

    
        async fn reward_manager_txs(&self, ctx: &Context<'_>) -> Result<Vec<reward_manager_txs::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            RewardManagerTxs::find().all(db).await
        }

    
        async fn alembic_version(&self, ctx: &Context<'_>) -> Result<Vec<alembic_version::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            AlembicVersion::find().all(db).await
        }

    
        async fn playlists(&self, ctx: &Context<'_>) -> Result<Vec<playlists::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            Playlists::find().all(db).await
        }

    
        async fn aggregate_playlist(&self, ctx: &Context<'_>) -> Result<Vec<aggregate_playlist::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            AggregatePlaylist::find().all(db).await
        }

    
        async fn subscriptions(&self, ctx: &Context<'_>) -> Result<Vec<subscriptions::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            Subscriptions::find().all(db).await
        }

    
        async fn cid_data(&self, ctx: &Context<'_>) -> Result<Vec<cid_data::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            CidData::find().all(db).await
        }

    
        async fn aggregate_daily_total_users_metrics(&self, ctx: &Context<'_>) -> Result<Vec<aggregate_daily_total_users_metrics::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            AggregateDailyTotalUsersMetrics::find().all(db).await
        }

    
        async fn notification_seen(&self, ctx: &Context<'_>) -> Result<Vec<notification_seen::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            NotificationSeen::find().all(db).await
        }

    
        async fn aggregate_monthly_app_name_metrics(&self, ctx: &Context<'_>) -> Result<Vec<aggregate_monthly_app_name_metrics::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            AggregateMonthlyAppNameMetrics::find().all(db).await
        }

    
        async fn track_routes(&self, ctx: &Context<'_>) -> Result<Vec<track_routes::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            TrackRoutes::find().all(db).await
        }

    
        async fn users(&self, ctx: &Context<'_>) -> Result<Vec<users::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            Users::find().all(db).await
        }

    
        async fn aggregate_plays(&self, ctx: &Context<'_>) -> Result<Vec<aggregate_plays::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            AggregatePlays::find().all(db).await
        }

    
        async fn rewards_manager_backfill_txs(&self, ctx: &Context<'_>) -> Result<Vec<rewards_manager_backfill_txs::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            RewardsManagerBackfillTxs::find().all(db).await
        }

    
        async fn tracks(&self, ctx: &Context<'_>) -> Result<Vec<tracks::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            Tracks::find().all(db).await
        }

    
        async fn remixes(&self, ctx: &Context<'_>) -> Result<Vec<remixes::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            Remixes::find().all(db).await
        }

    
        async fn eth_blocks(&self, ctx: &Context<'_>) -> Result<Vec<eth_blocks::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            EthBlocks::find().all(db).await
        }

    
        async fn indexing_checkpoints(&self, ctx: &Context<'_>) -> Result<Vec<indexing_checkpoints::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            IndexingCheckpoints::find().all(db).await
        }

    
        async fn user_bank_backfill_txs(&self, ctx: &Context<'_>) -> Result<Vec<user_bank_backfill_txs::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            UserBankBackfillTxs::find().all(db).await
        }

    
        async fn skipped_transactions(&self, ctx: &Context<'_>) -> Result<Vec<skipped_transactions::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            SkippedTransactions::find().all(db).await
        }

    
        async fn aggregate_daily_app_name_metrics(&self, ctx: &Context<'_>) -> Result<Vec<aggregate_daily_app_name_metrics::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            AggregateDailyAppNameMetrics::find().all(db).await
        }

    
        async fn playlist_routes(&self, ctx: &Context<'_>) -> Result<Vec<playlist_routes::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            PlaylistRoutes::find().all(db).await
        }

    
        async fn aggregate_monthly_total_users_metrics(&self, ctx: &Context<'_>) -> Result<Vec<aggregate_monthly_total_users_metrics::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            AggregateMonthlyTotalUsersMetrics::find().all(db).await
        }

    
        async fn plays_archive(&self, ctx: &Context<'_>) -> Result<Vec<plays_archive::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            PlaysArchive::find().all(db).await
        }

    
        async fn rpc_log(&self, ctx: &Context<'_>) -> Result<Vec<rpc_log::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            RpcLog::find().all(db).await
        }

    
        async fn challenges(&self, ctx: &Context<'_>) -> Result<Vec<challenges::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            Challenges::find().all(db).await
        }

    
        async fn user_tips(&self, ctx: &Context<'_>) -> Result<Vec<user_tips::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            UserTips::find().all(db).await
        }

    
        async fn user_balance_changes(&self, ctx: &Context<'_>) -> Result<Vec<user_balance_changes::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            UserBalanceChanges::find().all(db).await
        }

    
        async fn user_events(&self, ctx: &Context<'_>) -> Result<Vec<user_events::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            UserEvents::find().all(db).await
        }

    
        async fn playlist_seen(&self, ctx: &Context<'_>) -> Result<Vec<playlist_seen::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            PlaylistSeen::find().all(db).await
        }

    
        async fn route_metrics(&self, ctx: &Context<'_>) -> Result<Vec<route_metrics::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            RouteMetrics::find().all(db).await
        }

    
        async fn aggregate_daily_unique_users_metrics(&self, ctx: &Context<'_>) -> Result<Vec<aggregate_daily_unique_users_metrics::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            AggregateDailyUniqueUsersMetrics::find().all(db).await
        }

    
        async fn spl_token_tx(&self, ctx: &Context<'_>) -> Result<Vec<spl_token_tx::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            SplTokenTx::find().all(db).await
        }

    
        async fn aggregate_track(&self, ctx: &Context<'_>) -> Result<Vec<aggregate_track::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            AggregateTrack::find().all(db).await
        }

    
        async fn hourly_play_counts(&self, ctx: &Context<'_>) -> Result<Vec<hourly_play_counts::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            HourlyPlayCounts::find().all(db).await
        }

    
        async fn aggregate_monthly_unique_users_metrics(&self, ctx: &Context<'_>) -> Result<Vec<aggregate_monthly_unique_users_metrics::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            AggregateMonthlyUniqueUsersMetrics::find().all(db).await
        }

    
        async fn stems(&self, ctx: &Context<'_>) -> Result<Vec<stems::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            Stems::find().all(db).await
        }

    
        async fn aggregate_monthly_plays(&self, ctx: &Context<'_>) -> Result<Vec<aggregate_monthly_plays::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            AggregateMonthlyPlays::find().all(db).await
        }

    
        async fn blocks(&self, ctx: &Context<'_>) -> Result<Vec<blocks::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            Blocks::find().all(db).await
        }

    
        async fn supporter_rank_ups(&self, ctx: &Context<'_>) -> Result<Vec<supporter_rank_ups::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            SupporterRankUps::find().all(db).await
        }

    
        async fn user_bank_accounts(&self, ctx: &Context<'_>) -> Result<Vec<user_bank_accounts::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            UserBankAccounts::find().all(db).await
        }

    
        async fn audio_transactions_history(&self, ctx: &Context<'_>) -> Result<Vec<audio_transactions_history::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            AudioTransactionsHistory::find().all(db).await
        }

    
        async fn saves(&self, ctx: &Context<'_>) -> Result<Vec<saves::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            Saves::find().all(db).await
        }

    
        async fn aggregate_user(&self, ctx: &Context<'_>) -> Result<Vec<aggregate_user::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            AggregateUser::find().all(db).await
        }

    
        async fn audius_data_txs(&self, ctx: &Context<'_>) -> Result<Vec<audius_data_txs::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            AudiusDataTxs::find().all(db).await
        }

    
        async fn challenge_listen_streak(&self, ctx: &Context<'_>) -> Result<Vec<challenge_listen_streak::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            ChallengeListenStreak::find().all(db).await
        }

    
        async fn plays(&self, ctx: &Context<'_>) -> Result<Vec<plays::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            Plays::find().all(db).await
        }

    
        async fn track_trending_scores(&self, ctx: &Context<'_>) -> Result<Vec<track_trending_scores::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            TrackTrendingScores::find().all(db).await
        }

    
        async fn schema_migrations(&self, ctx: &Context<'_>) -> Result<Vec<schema_migrations::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            SchemaMigrations::find().all(db).await
        }

    
        async fn reposts(&self, ctx: &Context<'_>) -> Result<Vec<reposts::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            Reposts::find().all(db).await
        }

    
        async fn app_name_metrics(&self, ctx: &Context<'_>) -> Result<Vec<app_name_metrics::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            AppNameMetrics::find().all(db).await
        }

    
        async fn chat_permissions(&self, ctx: &Context<'_>) -> Result<Vec<chat_permissions::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            ChatPermissions::find().all(db).await
        }

    
        async fn challenge_disbursements(&self, ctx: &Context<'_>) -> Result<Vec<challenge_disbursements::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            ChallengeDisbursements::find().all(db).await
        }

    
        async fn associated_wallets(&self, ctx: &Context<'_>) -> Result<Vec<associated_wallets::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            AssociatedWallets::find().all(db).await
        }

    
        async fn user_challenges(&self, ctx: &Context<'_>) -> Result<Vec<user_challenges::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            UserChallenges::find().all(db).await
        }

    
        async fn user_bank_txs(&self, ctx: &Context<'_>) -> Result<Vec<user_bank_txs::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            UserBankTxs::find().all(db).await
        }

    
        async fn aggregate_user_tips(&self, ctx: &Context<'_>) -> Result<Vec<aggregate_user_tips::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            AggregateUserTips::find().all(db).await
        }

    
        async fn chat(&self, ctx: &Context<'_>) -> Result<Vec<chat::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            Chat::find().all(db).await
        }

    
        async fn spl_token_backfill_txs(&self, ctx: &Context<'_>) -> Result<Vec<spl_token_backfill_txs::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            SplTokenBackfillTxs::find().all(db).await
        }

    
        async fn trending_results(&self, ctx: &Context<'_>) -> Result<Vec<trending_results::Model>, DbErr>
        {    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            TrendingResults::find().all(db).await
        }

    
        }
    