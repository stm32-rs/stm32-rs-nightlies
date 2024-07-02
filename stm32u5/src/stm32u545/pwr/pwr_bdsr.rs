///Register `PWR_BDSR` reader
pub type R = crate::R<PWR_BDSRrs>;
///Field `VBATH` reader - Backup domain voltage level monitoring versus high threshold
pub type VBATH_R = crate::BitReader;
///Field `TEMPL` reader - Temperature level monitoring versus low threshold
pub type TEMPL_R = crate::BitReader;
///Field `TEMPH` reader - Temperature level monitoring versus high threshold
pub type TEMPH_R = crate::BitReader;
impl R {
    ///Bit 1 - Backup domain voltage level monitoring versus high threshold
    #[inline(always)]
    pub fn vbath(&self) -> VBATH_R {
        VBATH_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Temperature level monitoring versus low threshold
    #[inline(always)]
    pub fn templ(&self) -> TEMPL_R {
        TEMPL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Temperature level monitoring versus high threshold
    #[inline(always)]
    pub fn temph(&self) -> TEMPH_R {
        TEMPH_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_BDSR")
            .field("vbath", &self.vbath())
            .field("templ", &self.templ())
            .field("temph", &self.temph())
            .finish()
    }
}
/**PWR Backup domain status register

You can [`read`](crate::Reg::read) this register and get [`pwr_bdsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#PWR:PWR_BDSR)*/
pub struct PWR_BDSRrs;
impl crate::RegisterSpec for PWR_BDSRrs {
    type Ux = u32;
}
///`read()` method returns [`pwr_bdsr::R`](R) reader structure
impl crate::Readable for PWR_BDSRrs {}
///`reset()` method sets PWR_BDSR to value 0
impl crate::Resettable for PWR_BDSRrs {
    const RESET_VALUE: u32 = 0;
}
