///Register `IWDG_EWCR` reader
pub type R = crate::R<IWDG_EWCRrs>;
///Register `IWDG_EWCR` writer
pub type W = crate::W<IWDG_EWCRrs>;
/**Field `EWIT` reader - Watchdog counter window value These bits are write access protected (see Section126.4.6). They are written by software to define at which position of the IWDCNT down-counter the early wake-up interrupt must be generated. The early interrupt is generated when the IWDCNT is lower or equal to EWIT\[11:0\]1-11. EWIT\[11:0\]
must be bigger than 1. An interrupt is generated only if EWIE = 1. The EWU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the reload value. Note: Reading this register returns the Early wake-up comparator value and the Interrupt enable bit from the V<sub>DD</sub> voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the EWU bit in the IWDG status register (IWDG_SR) is reset.*/
pub type EWIT_R = crate::FieldReader<u16>;
/**Field `EWIT` writer - Watchdog counter window value These bits are write access protected (see Section126.4.6). They are written by software to define at which position of the IWDCNT down-counter the early wake-up interrupt must be generated. The early interrupt is generated when the IWDCNT is lower or equal to EWIT\[11:0\]1-11. EWIT\[11:0\]
must be bigger than 1. An interrupt is generated only if EWIE = 1. The EWU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the reload value. Note: Reading this register returns the Early wake-up comparator value and the Interrupt enable bit from the V<sub>DD</sub> voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the EWU bit in the IWDG status register (IWDG_SR) is reset.*/
pub type EWIT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `EWIC` writer - Watchdog early interrupt acknowledge The software must write a 1 into this bit in order to acknowledge the early wake-up interrupt and to clear the EWIF flag. Writing 0 has not effect, reading this flag returns a 0.
pub type EWIC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWIE` reader - Watchdog early interrupt enable Set and reset by software. The EWU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the value of this bit.
pub type EWIE_R = crate::BitReader;
///Field `EWIE` writer - Watchdog early interrupt enable Set and reset by software. The EWU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the value of this bit.
pub type EWIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    /**Bits 0:11 - Watchdog counter window value These bits are write access protected (see Section126.4.6). They are written by software to define at which position of the IWDCNT down-counter the early wake-up interrupt must be generated. The early interrupt is generated when the IWDCNT is lower or equal to EWIT\[11:0\]1-11. EWIT\[11:0\]
    must be bigger than 1. An interrupt is generated only if EWIE = 1. The EWU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the reload value. Note: Reading this register returns the Early wake-up comparator value and the Interrupt enable bit from the V<sub>DD</sub> voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the EWU bit in the IWDG status register (IWDG_SR) is reset.*/
    #[inline(always)]
    pub fn ewit(&self) -> EWIT_R {
        EWIT_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 15 - Watchdog early interrupt enable Set and reset by software. The EWU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the value of this bit.
    #[inline(always)]
    pub fn ewie(&self) -> EWIE_R {
        EWIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWDG_EWCR")
            .field("ewit", &self.ewit())
            .field("ewie", &self.ewie())
            .finish()
    }
}
impl W {
    /**Bits 0:11 - Watchdog counter window value These bits are write access protected (see Section126.4.6). They are written by software to define at which position of the IWDCNT down-counter the early wake-up interrupt must be generated. The early interrupt is generated when the IWDCNT is lower or equal to EWIT\[11:0\]1-11. EWIT\[11:0\]
    must be bigger than 1. An interrupt is generated only if EWIE = 1. The EWU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the reload value. Note: Reading this register returns the Early wake-up comparator value and the Interrupt enable bit from the V<sub>DD</sub> voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the EWU bit in the IWDG status register (IWDG_SR) is reset.*/
    #[inline(always)]
    pub fn ewit(&mut self) -> EWIT_W<IWDG_EWCRrs> {
        EWIT_W::new(self, 0)
    }
    ///Bit 14 - Watchdog early interrupt acknowledge The software must write a 1 into this bit in order to acknowledge the early wake-up interrupt and to clear the EWIF flag. Writing 0 has not effect, reading this flag returns a 0.
    #[inline(always)]
    pub fn ewic(&mut self) -> EWIC_W<IWDG_EWCRrs> {
        EWIC_W::new(self, 14)
    }
    ///Bit 15 - Watchdog early interrupt enable Set and reset by software. The EWU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the value of this bit.
    #[inline(always)]
    pub fn ewie(&mut self) -> EWIE_W<IWDG_EWCRrs> {
        EWIE_W::new(self, 15)
    }
}
/**IWDG early wake-up interrupt register

You can [`read`](crate::Reg::read) this register and get [`iwdg_ewcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_ewcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#IWDG:IWDG_EWCR)*/
pub struct IWDG_EWCRrs;
impl crate::RegisterSpec for IWDG_EWCRrs {
    type Ux = u32;
}
///`read()` method returns [`iwdg_ewcr::R`](R) reader structure
impl crate::Readable for IWDG_EWCRrs {}
///`write(|w| ..)` method takes [`iwdg_ewcr::W`](W) writer structure
impl crate::Writable for IWDG_EWCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IWDG_EWCR to value 0
impl crate::Resettable for IWDG_EWCRrs {
    const RESET_VALUE: u32 = 0;
}