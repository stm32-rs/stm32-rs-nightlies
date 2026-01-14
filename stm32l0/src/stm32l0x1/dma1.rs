#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    isr: ISR,
    ifcr: IFCR,
    ch: [CH; 7],
    _reserved3: [u8; 0x14],
    cselr: CSELR,
}
impl RegisterBlock {
    ///0x00 - interrupt status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x04 - interrupt flag clear register
    #[inline(always)]
    pub const fn ifcr(&self) -> &IFCR {
        &self.ifcr
    }
    ///0x08..0x94 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `CH1` cluster.</div>
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    ///Iterator for array of:
    ///0x08..0x94 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    ///0x08..0x1c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    #[inline(always)]
    pub const fn ch1(&self) -> &CH {
        self.ch(0)
    }
    ///0x1c..0x30 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    #[inline(always)]
    pub const fn ch2(&self) -> &CH {
        self.ch(1)
    }
    ///0x30..0x44 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    #[inline(always)]
    pub const fn ch3(&self) -> &CH {
        self.ch(2)
    }
    ///0x44..0x58 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    #[inline(always)]
    pub const fn ch4(&self) -> &CH {
        self.ch(3)
    }
    ///0x58..0x6c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    #[inline(always)]
    pub const fn ch5(&self) -> &CH {
        self.ch(4)
    }
    ///0x6c..0x80 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    #[inline(always)]
    pub const fn ch6(&self) -> &CH {
        self.ch(5)
    }
    ///0x80..0x94 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    #[inline(always)]
    pub const fn ch7(&self) -> &CH {
        self.ch(6)
    }
    ///0xa8 - channel selection register
    #[inline(always)]
    pub const fn cselr(&self) -> &CSELR {
        &self.cselr
    }
}
/**ISR (r) register accessor: interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#DMA1:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///interrupt status register
pub mod isr;
/**IFCR (w) register accessor: interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#DMA1:IFCR)

For information about available fields see [`mod@ifcr`] module*/
pub type IFCR = crate::Reg<ifcr::IFCRrs>;
///interrupt flag clear register
pub mod ifcr;
///Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
pub use self::ch::CH;
///Cluster
///Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
pub mod ch;
/**CSELR (rw) register accessor: channel selection register

You can [`read`](crate::Reg::read) this register and get [`cselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#DMA1:CSELR)

For information about available fields see [`mod@cselr`] module*/
pub type CSELR = crate::Reg<cselr::CSELRrs>;
///channel selection register
pub mod cselr;
