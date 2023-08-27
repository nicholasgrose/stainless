use std::fmt::{Display, Formatter};

pub enum MemoryAmount {
    Gibibyte(u32),
}

impl Display for MemoryAmount {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&match self {
            MemoryAmount::Gibibyte(amount) => format!("{}G", amount),
        })?;

        Ok(())
    }
}

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
