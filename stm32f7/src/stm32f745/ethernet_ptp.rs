#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ptptscr: PTPTSCR,
    ptpssir: PTPSSIR,
    ptptshr: PTPTSHR,
    ptptslr: PTPTSLR,
    ptptshur: PTPTSHUR,
    ptptslur: PTPTSLUR,
    ptptsar: PTPTSAR,
    ptptthr: PTPTTHR,
    ptpttlr: PTPTTLR,
    _reserved9: [u8; 0x04],
    ptptssr: PTPTSSR,
    ptpppscr: PTPPPSCR,
}
impl RegisterBlock {
    ///0x00 - Ethernet PTP time stamp control register
    #[inline(always)]
    pub const fn ptptscr(&self) -> &PTPTSCR {
        &self.ptptscr
    }
    ///0x04 - Ethernet PTP subsecond increment register
    #[inline(always)]
    pub const fn ptpssir(&self) -> &PTPSSIR {
        &self.ptpssir
    }
    ///0x08 - Ethernet PTP time stamp high register
    #[inline(always)]
    pub const fn ptptshr(&self) -> &PTPTSHR {
        &self.ptptshr
    }
    ///0x0c - Ethernet PTP time stamp low register
    #[inline(always)]
    pub const fn ptptslr(&self) -> &PTPTSLR {
        &self.ptptslr
    }
    ///0x10 - Ethernet PTP time stamp high update register
    #[inline(always)]
    pub const fn ptptshur(&self) -> &PTPTSHUR {
        &self.ptptshur
    }
    ///0x14 - Ethernet PTP time stamp low update register
    #[inline(always)]
    pub const fn ptptslur(&self) -> &PTPTSLUR {
        &self.ptptslur
    }
    ///0x18 - Ethernet PTP time stamp addend register
    #[inline(always)]
    pub const fn ptptsar(&self) -> &PTPTSAR {
        &self.ptptsar
    }
    ///0x1c - Ethernet PTP target time high register
    #[inline(always)]
    pub const fn ptptthr(&self) -> &PTPTTHR {
        &self.ptptthr
    }
    ///0x20 - Ethernet PTP target time low register
    #[inline(always)]
    pub const fn ptpttlr(&self) -> &PTPTTLR {
        &self.ptpttlr
    }
    ///0x28 - Ethernet PTP time stamp status register
    #[inline(always)]
    pub const fn ptptssr(&self) -> &PTPTSSR {
        &self.ptptssr
    }
    ///0x2c - Ethernet PTP PPS control register
    #[inline(always)]
    pub const fn ptpppscr(&self) -> &PTPPPSCR {
        &self.ptpppscr
    }
}
/**PTPTSCR (rw) register accessor: Ethernet PTP time stamp control register

You can [`read`](crate::Reg::read) this register and get [`ptptscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#Ethernet_PTP:PTPTSCR)

For information about available fields see [`mod@ptptscr`] module*/
pub type PTPTSCR = crate::Reg<ptptscr::PTPTSCRrs>;
///Ethernet PTP time stamp control register
pub mod ptptscr;
/**PTPSSIR (rw) register accessor: Ethernet PTP subsecond increment register

You can [`read`](crate::Reg::read) this register and get [`ptpssir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptpssir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#Ethernet_PTP:PTPSSIR)

For information about available fields see [`mod@ptpssir`] module*/
pub type PTPSSIR = crate::Reg<ptpssir::PTPSSIRrs>;
///Ethernet PTP subsecond increment register
pub mod ptpssir;
/**PTPTSHR (r) register accessor: Ethernet PTP time stamp high register

You can [`read`](crate::Reg::read) this register and get [`ptptshr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#Ethernet_PTP:PTPTSHR)

For information about available fields see [`mod@ptptshr`] module*/
pub type PTPTSHR = crate::Reg<ptptshr::PTPTSHRrs>;
///Ethernet PTP time stamp high register
pub mod ptptshr;
/**PTPTSLR (r) register accessor: Ethernet PTP time stamp low register

You can [`read`](crate::Reg::read) this register and get [`ptptslr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#Ethernet_PTP:PTPTSLR)

For information about available fields see [`mod@ptptslr`] module*/
pub type PTPTSLR = crate::Reg<ptptslr::PTPTSLRrs>;
///Ethernet PTP time stamp low register
pub mod ptptslr;
/**PTPTSHUR (rw) register accessor: Ethernet PTP time stamp high update register

You can [`read`](crate::Reg::read) this register and get [`ptptshur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptshur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#Ethernet_PTP:PTPTSHUR)

For information about available fields see [`mod@ptptshur`] module*/
pub type PTPTSHUR = crate::Reg<ptptshur::PTPTSHURrs>;
///Ethernet PTP time stamp high update register
pub mod ptptshur;
/**PTPTSLUR (rw) register accessor: Ethernet PTP time stamp low update register

You can [`read`](crate::Reg::read) this register and get [`ptptslur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptslur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#Ethernet_PTP:PTPTSLUR)

For information about available fields see [`mod@ptptslur`] module*/
pub type PTPTSLUR = crate::Reg<ptptslur::PTPTSLURrs>;
///Ethernet PTP time stamp low update register
pub mod ptptslur;
/**PTPTSAR (rw) register accessor: Ethernet PTP time stamp addend register

You can [`read`](crate::Reg::read) this register and get [`ptptsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#Ethernet_PTP:PTPTSAR)

For information about available fields see [`mod@ptptsar`] module*/
pub type PTPTSAR = crate::Reg<ptptsar::PTPTSARrs>;
///Ethernet PTP time stamp addend register
pub mod ptptsar;
/**PTPTTHR (rw) register accessor: Ethernet PTP target time high register

You can [`read`](crate::Reg::read) this register and get [`ptptthr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptthr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#Ethernet_PTP:PTPTTHR)

For information about available fields see [`mod@ptptthr`] module*/
pub type PTPTTHR = crate::Reg<ptptthr::PTPTTHRrs>;
///Ethernet PTP target time high register
pub mod ptptthr;
/**PTPTTLR (rw) register accessor: Ethernet PTP target time low register

You can [`read`](crate::Reg::read) this register and get [`ptpttlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptpttlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#Ethernet_PTP:PTPTTLR)

For information about available fields see [`mod@ptpttlr`] module*/
pub type PTPTTLR = crate::Reg<ptpttlr::PTPTTLRrs>;
///Ethernet PTP target time low register
pub mod ptpttlr;
/**PTPTSSR (r) register accessor: Ethernet PTP time stamp status register

You can [`read`](crate::Reg::read) this register and get [`ptptssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#Ethernet_PTP:PTPTSSR)

For information about available fields see [`mod@ptptssr`] module*/
pub type PTPTSSR = crate::Reg<ptptssr::PTPTSSRrs>;
///Ethernet PTP time stamp status register
pub mod ptptssr;
/**PTPPPSCR (r) register accessor: Ethernet PTP PPS control register

You can [`read`](crate::Reg::read) this register and get [`ptpppscr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#Ethernet_PTP:PTPPPSCR)

For information about available fields see [`mod@ptpppscr`] module*/
pub type PTPPPSCR = crate::Reg<ptpppscr::PTPPPSCRrs>;
///Ethernet PTP PPS control register
pub mod ptpppscr;
