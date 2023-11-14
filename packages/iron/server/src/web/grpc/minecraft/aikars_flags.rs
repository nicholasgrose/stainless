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
            program: "java".into(),
            args: vec![
                format!("-Xms{}", value.memory).into(),
                format!("-Xmx{}", value.memory).into(),
                "-XX:+UseG1GC".into(),
                "-XX:+ParallelRefProcEnabled".into(),
                "-XX:MaxGCPauseMillis=200".into(),
                "-XX:+UnlockExperimentalVMOptions".into(),
                "-XX:+DisableExplicitGC".into(),
                "-XX:+AlwaysPreTouch".into(),
                "-XX:G1NewSizePercent=30".into(),
                "-XX:G1MaxNewSizePercent=40".into(),
                "-XX:G1HeapRegionSize=8M".into(),
                "-XX:G1ReservePercent=20".into(),
                "-XX:G1HeapWastePercent=5".into(),
                "-XX:G1MixedGCCountTarget=4".into(),
                "-XX:InitiatingHeapOccupancyPercent=15".into(),
                "-XX:G1MixedGCLiveThresholdPercent=90".into(),
                "-XX:G1RSetUpdatingPauseTimePercent=5".into(),
                "-XX:SurvivorRatio=32".into(),
                "-XX:+PerfDisableSharedMem".into(),
                "-XX:MaxTenuringThreshold=1".into(),
                "-Dusing.aikars.flags=https://mcflags.emc.gs".into(),
                "-Daikars.new.flags=true".into(),
                "-jar".into(),
                "paper.jar".into(),
                "--nogui".into(),
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
