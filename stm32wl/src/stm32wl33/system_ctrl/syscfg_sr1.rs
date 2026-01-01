///Register `SYSCFG_SR1` reader
pub type R = crate::R<SYSCFG_SR1rs>;
///Field `RFIP_BUSY_STATUS` reader - RFIP_BUSY_STATUS: MR_SUBG BUSY status: Software should check that MR_SUBG IP is not busy (or relay on the related interrupt) before to initiate any system clock frequency switch to operate the switching in a safe way. 0: MR_SUBG is not busy. 1: MR_SUBG is busy
pub type RFIP_BUSY_STATUS_R = crate::BitReader;
impl R {
    ///Bit 5 - RFIP_BUSY_STATUS: MR_SUBG BUSY status: Software should check that MR_SUBG IP is not busy (or relay on the related interrupt) before to initiate any system clock frequency switch to operate the switching in a safe way. 0: MR_SUBG is not busy. 1: MR_SUBG is busy
    #[inline(always)]
    pub fn rfip_busy_status(&self) -> RFIP_BUSY_STATUS_R {
        RFIP_BUSY_STATUS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_SR1")
            .field("rfip_busy_status", &self.rfip_busy_status())
            .finish()
    }
}
/**SYSCFG_SR1 register

You can [`read`](crate::Reg::read) this register and get [`syscfg_sr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:SYSCFG_SR1)*/
pub struct SYSCFG_SR1rs;
impl crate::RegisterSpec for SYSCFG_SR1rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_sr1::R`](R) reader structure
impl crate::Readable for SYSCFG_SR1rs {}
///`reset()` method sets SYSCFG_SR1 to value 0
impl crate::Resettable for SYSCFG_SR1rs {}
