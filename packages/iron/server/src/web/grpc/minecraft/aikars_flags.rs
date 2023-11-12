use std::fmt::{Display, Formatter};

use anyhow::Context;

use iron_api::minecraft_service::{MemoryUnit, MinecraftServerDefinition};

use crate::manager::app::AppCommand;

// See https://docs.papermc.io/paper/aikars-flags for for info about these flags.
pub struct AikarsFlags {
    pub memory: MemoryAmount,
}

impl From<AikarsFlags> for AppCommand {
    fn from(value: AikarsFlags) -> Self {
        AppCommand {
            program: "java".to_string(),
            args: vec![
                format!("-Xms{}", value.memory),
                format!("-Xmx{}", value.memory),
                "-XX:+UseG1GC".to_string(),
                "-XX:+ParallelRefProcEnabled".to_string(),
                "-XX:MaxGCPauseMillis=200".to_string(),
                "-XX:+UnlockExperimentalVMOptions".to_string(),
                "-XX:+DisableExplicitGC".to_string(),
                "-XX:+AlwaysPreTouch".to_string(),
                "-XX:G1NewSizePercent=30".to_string(),
                "-XX:G1MaxNewSizePercent=40".to_string(),
                "-XX:G1HeapRegionSize=8M".to_string(),
                "-XX:G1ReservePercent=20".to_string(),
                "-XX:G1HeapWastePercent=5".to_string(),
                "-XX:G1MixedGCCountTarget=4".to_string(),
                "-XX:InitiatingHeapOccupancyPercent=15".to_string(),
                "-XX:G1MixedGCLiveThresholdPercent=90".to_string(),
                "-XX:G1RSetUpdatingPauseTimePercent=5".to_string(),
                "-XX:SurvivorRatio=32".to_string(),
                "-XX:+PerfDisableSharedMem".to_string(),
                "-XX:MaxTenuringThreshold=1".to_string(),
                "-Dusing.aikars.flags=https://mcflags.emc.gs".to_string(),
                "-Daikars.new.flags=true".to_string(),
                "-jar".to_string(),
                "paper.jar".to_string(),
                "--nogui".to_string(),
            ],
        }
    }
}

impl TryFrom<&MinecraftServerDefinition> for AikarsFlags {
    type Error = anyhow::Error;

    fn try_from(value: &MinecraftServerDefinition) -> anyhow::Result<Self> {
        let memory = value
            .memory
            .as_ref()
            .map(|m| m.try_into())
            .unwrap_or(Ok(MemoryAmount::Gibibytes(4)))?;

        Ok(AikarsFlags { memory })
    }
}

pub enum MemoryAmount {
    Gibibytes(u64),
    Mebibytes(u64),
    Kibibytes(u64),
}

impl Display for MemoryAmount {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&match self {
            MemoryAmount::Gibibytes(amount) => format!("{}G", amount),
            MemoryAmount::Mebibytes(amount) => format!("{}m", amount),
            MemoryAmount::Kibibytes(amount) => format!("{}k", amount),
        })?;

        Ok(())
    }
}

impl TryFrom<&iron_api::minecraft_service::MemoryAmount> for MemoryAmount {
    type Error = anyhow::Error;

    fn try_from(value: &iron_api::minecraft_service::MemoryAmount) -> anyhow::Result<Self> {
        let unit = MemoryUnit::try_from(value.unit).context("invalid memory unit provided")?;

        Ok(match unit {
            MemoryUnit::Gibibytes => MemoryAmount::Gibibytes(value.amount),
            MemoryUnit::Mebibytes => MemoryAmount::Mebibytes(value.amount),
            MemoryUnit::Kibibytes => MemoryAmount::Kibibytes(value.amount),
        })
    }
}
