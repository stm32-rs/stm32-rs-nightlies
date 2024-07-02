///Register `PWR_SR` reader
pub type R = crate::R<PWR_SRrs>;
///Register `PWR_SR` writer
pub type W = crate::W<PWR_SRrs>;
///Field `CSSF` writer - Clear Stop and Standby flags This bit is protected against non-secure access when LPMSEC = 1 in PWR_SECCFGR. This bit is protected against unprivileged access when LPMSEC = 1 and SPRIV = 1 in PWR_PRIVCFGR, or when LPMSEC = 0 and NSPRIV = 1. Writing 1 to this bit clears the STOPF and SBF flags.
pub type CSSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOPF` reader - Stop flag This bit is set by hardware when the device enters a Stop mode, and is cleared by software by writing 1 to the CSSF bit.
pub type STOPF_R = crate::BitReader;
///Field `SBF` reader - Standby flag This bit is set by hardware when the device enters the Standby mode, and is cleared by writing 1 to the CSSF bit, or by a power-on reset. It is not cleared by the system reset.
pub type SBF_R = crate::BitReader;
impl R {
    ///Bit 1 - Stop flag This bit is set by hardware when the device enters a Stop mode, and is cleared by software by writing 1 to the CSSF bit.
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Standby flag This bit is set by hardware when the device enters the Standby mode, and is cleared by writing 1 to the CSSF bit, or by a power-on reset. It is not cleared by the system reset.
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_SR")
            .field("stopf", &self.stopf())
            .field("sbf", &self.sbf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clear Stop and Standby flags This bit is protected against non-secure access when LPMSEC = 1 in PWR_SECCFGR. This bit is protected against unprivileged access when LPMSEC = 1 and SPRIV = 1 in PWR_PRIVCFGR, or when LPMSEC = 0 and NSPRIV = 1. Writing 1 to this bit clears the STOPF and SBF flags.
    #[inline(always)]
    #[must_use]
    pub fn cssf(&mut self) -> CSSF_W<PWR_SRrs> {
        CSSF_W::new(self, 0)
    }
}
/**PWR status register

You can [`read`](crate::Reg::read) this register and get [`pwr_sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#PWR:PWR_SR)*/
pub struct PWR_SRrs;
impl crate::RegisterSpec for PWR_SRrs {
    type Ux = u32;
}
///`read()` method returns [`pwr_sr::R`](R) reader structure
impl crate::Readable for PWR_SRrs {}
///`write(|w| ..)` method takes [`pwr_sr::W`](W) writer structure
impl crate::Writable for PWR_SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PWR_SR to value 0
impl crate::Resettable for PWR_SRrs {
    const RESET_VALUE: u32 = 0;
}
