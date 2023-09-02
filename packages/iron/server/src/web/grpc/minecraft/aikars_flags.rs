use std::fmt::{Display, Formatter};

use anyhow::Context;

use iron_api::minecraft_service::{MemoryUnit, MinecraftServerDefinition};

// See https://docs.papermc.io/paper/aikars-flags for for info about these flags.
pub struct AikarsFlags {
    pub memory: MemoryAmount,
}

impl Display for AikarsFlags {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&format!(
            "java \
            -Xms{} \
            -Xmx{} \
            -XX:+UseG1GC \
            -XX:+ParallelRefProcEnabled \
            -XX:MaxGCPauseMillis=200 \
            -XX:+UnlockExperimentalVMOptions \
            -XX:+DisableExplicitGC \
            -XX:+AlwaysPreTouch \
            -XX:G1NewSizePercent=30 \
            -XX:G1MaxNewSizePercent=40 \
            -XX:G1HeapRegionSize=8M \
            -XX:G1ReservePercent=20 \
            -XX:G1HeapWastePercent=5 \
            -XX:G1MixedGCCountTarget=4 \
            -XX:InitiatingHeapOccupancyPercent=15 \
            -XX:G1MixedGCLiveThresholdPercent=90 \
            -XX:G1RSetUpdatingPauseTimePercent=5 \
            -XX:SurvivorRatio=32 \
            -XX:+PerfDisableSharedMem \
            -XX:MaxTenuringThreshold=1 \
            -Dusing.aikars.flags=https://mcflags.emc.gs \
            -Daikars.new.flags=true \
            -jar paper.jar --nogui",
            self.memory, self.memory
        ))?;

        Ok(())
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
