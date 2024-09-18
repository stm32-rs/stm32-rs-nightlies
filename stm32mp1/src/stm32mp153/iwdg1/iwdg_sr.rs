///Register `IWDG_SR` reader
pub type R = crate::R<IWDG_SRrs>;
///Field `PVU` reader - PVU
pub type PVU_R = crate::BitReader;
///Field `RVU` reader - RVU
pub type RVU_R = crate::BitReader;
///Field `WVU` reader - WVU
pub type WVU_R = crate::BitReader;
impl R {
    ///Bit 0 - PVU
    #[inline(always)]
    pub fn pvu(&self) -> PVU_R {
        PVU_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RVU
    #[inline(always)]
    pub fn rvu(&self) -> RVU_R {
        RVU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WVU
    #[inline(always)]
    pub fn wvu(&self) -> WVU_R {
        WVU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWDG_SR")
            .field("pvu", &self.pvu())
            .field("rvu", &self.rvu())
            .field("wvu", &self.wvu())
            .finish()
    }
}
/**Status register

You can [`read`](crate::Reg::read) this register and get [`iwdg_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#IWDG1:IWDG_SR)*/
pub struct IWDG_SRrs;
impl crate::RegisterSpec for IWDG_SRrs {
    type Ux = u32;
}
///`read()` method returns [`iwdg_sr::R`](R) reader structure
impl crate::Readable for IWDG_SRrs {}
///`reset()` method sets IWDG_SR to value 0
impl crate::Resettable for IWDG_SRrs {
    const RESET_VALUE: u32 = 0;
}
