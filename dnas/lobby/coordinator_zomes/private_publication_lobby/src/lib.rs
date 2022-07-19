use hdk::prelude::{holo_hash::AgentPubKeyB64, *};

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
struct Properties {
    progenitor: AgentPubKeyB64,
}

#[hdk_extern]
fn progenitor(_: ()) -> ExternResult<AgentPubKey> {
    let properties = dna_info()?.properties;
    
    let progenitor_properties = Properties::try_from(properties)
        .map_err(|err| {
            wasm_error!(err.into())
        })?;

    Ok(progenitor_properties.progenitor.into())
}

#[hdk_extern]
fn request_read_all_posts(_: ()) -> ExternResult<()> {
    Ok(())
}

/** Don't change */
#[cfg(feature = "exercise")]
extern crate private_publication_lobby;
