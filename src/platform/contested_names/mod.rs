mod query_dpns_contested_resources;
mod query_dpns_vote_contenders;

use crate::context::AppContext;
use dash_sdk::dpp::data_contract::accessors::v0::DataContractV0Getters;
use dash_sdk::dpp::data_contract::document_type::accessors::DocumentTypeV0Getters;
use dash_sdk::{platform::FetchMany, Sdk};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ContestedResourceTask {
    QueryDPNSContestedResources,
    QueryDPNSVoteContenders(String),
    // VoteOnContestedResource(VotePoll, ResourceVoteChoice),
}

impl AppContext {
    pub async fn run_contested_resource_task(
        &self,
        task: ContestedResourceTask,
        sdk: &Sdk,
    ) -> Result<(), String> {
        let sdk = sdk.clone();
        match &task {
            ContestedResourceTask::QueryDPNSContestedResources => {
                self.query_dpns_contested_resources(sdk).await
            }
            ContestedResourceTask::QueryDPNSVoteContenders(name) => {
                self.query_dpns_vote_contenders(name, sdk).await
            } // ContestedResourceTask::VoteOnContestedResource(vote_poll, vote_choice) => {
              //     let mut vote = Vote::default();
              //     let identity_private_keys_lock = self.known_identities_private_keys.lock().await;
              //     let loaded_identity_lock = match self.loaded_identity.lock().await.clone() {
              //         Some(identity) => identity,
              //         None => {
              //             return BackendEvent::TaskCompleted {
              //                 task: Task::Document(task),
              //                 execution_result: Err(
              //                     "No loaded identity for signing vote transaction".to_string(),
              //                 ),
              //             };
              //         }
              //     };
              //
              //     let mut signer = SimpleSigner::default();
              //     let Identity::V0(identity_v0) = &loaded_identity_lock;
              //     for (key_id, public_key) in &identity_v0.public_keys {
              //         let identity_key_tuple = (identity_v0.id, *key_id);
              //         if let Some(private_key_bytes) =
              //             identity_private_keys_lock.get(&identity_key_tuple)
              //         {
              //             signer
              //                 .private_keys
              //                 .insert(public_key.clone(), private_key_bytes.clone());
              //         }
              //     }
              //
              //     let voting_public_key = match loaded_identity_lock.get_first_public_key_matching(
              //         Purpose::VOTING,
              //         HashSet::from(SecurityLevel::full_range()),
              //         HashSet::from(KeyType::all_key_types()),
              //         false,
              //     ) {
              //         Some(voting_key) => voting_key,
              //         None => {
              //             return BackendEvent::TaskCompleted {
              //                 task: Task::Document(task),
              //                 execution_result: Err(
              //                     "No voting key in the loaded identity. Are you sure it's a masternode identity?".to_string()
              //                 ),
              //             };
              //         }
              //     };
              //
              //     match vote {
              //         Vote::ResourceVote(ref mut resource_vote) => match resource_vote {
              //             ResourceVote::V0(ref mut resource_vote_v0) => {
              //                 resource_vote_v0.vote_poll = vote_poll.clone();
              //                 resource_vote_v0.resource_vote_choice = *vote_choice;
              //                 let pro_tx_hash = self
              //                     .loaded_identity_pro_tx_hash
              //                     .lock()
              //                     .await
              //                     .expect("Expected a proTxHash in AppState");
              //                 match vote
              //                     .put_to_platform_and_wait_for_response(
              //                         pro_tx_hash,
              //                         voting_public_key,
              //                         sdk,
              //                         &signer,
              //                         None,
              //                     )
              //                     .await
              //                 {
              //                     Ok(_) => {
              //                         // TODO: Insert vote result into the database
              //                         BackendEvent::TaskCompleted {
              //                             task: Task::Document(task),
              //                             execution_result: Ok(CompletedTaskPayload::String(
              //                                 "Vote cast successfully".to_string(),
              //                             )),
              //                         }
              //                     }
              //                     Err(e) => BackendEvent::TaskCompleted {
              //                         task: Task::Document(task),
              //                         execution_result: Err(e.to_string()),
              //                     },
              //                 }
              //             }
              //         },
              //     }
              // }
        }
    }
}
