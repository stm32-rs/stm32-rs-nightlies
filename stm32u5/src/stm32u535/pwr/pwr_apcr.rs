///Register `PWR_APCR` reader
pub type R = crate::R<PWR_APCRrs>;
///Register `PWR_APCR` writer
pub type W = crate::W<PWR_APCRrs>;
///Field `APC` reader - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PWR_PUCRx and PWR_PDCRx are applied. When this bit is cleared, PWR_PUCRx and PWR_PDCRx are not applied to the I/Os.
pub type APC_R = crate::BitReader;
///Field `APC` writer - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PWR_PUCRx and PWR_PDCRx are applied. When this bit is cleared, PWR_PUCRx and PWR_PDCRx are not applied to the I/Os.
pub type APC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PWR_PUCRx and PWR_PDCRx are applied. When this bit is cleared, PWR_PUCRx and PWR_PDCRx are not applied to the I/Os.
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_APCR")
            .field("apc", &self.apc())
            .finish()
    }
}
impl W {
    ///Bit 0 - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PWR_PUCRx and PWR_PDCRx are applied. When this bit is cleared, PWR_PUCRx and PWR_PDCRx are not applied to the I/Os.
    #[inline(always)]
    #[must_use]
    pub fn apc(&mut self) -> APC_W<PWR_APCRrs> {
        APC_W::new(self, 0)
    }
}
/**PWR apply pull configuration register

You can [`read`](crate::Reg::read) this register and get [`pwr_apcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_apcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#PWR:PWR_APCR)*/
pub struct PWR_APCRrs;
impl crate::RegisterSpec for PWR_APCRrs {
    type Ux = u32;
}
///`read()` method returns [`pwr_apcr::R`](R) reader structure
impl crate::Readable for PWR_APCRrs {}
///`write(|w| ..)` method takes [`pwr_apcr::W`](W) writer structure
impl crate::Writable for PWR_APCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PWR_APCR to value 0
impl crate::Resettable for PWR_APCRrs {
    const RESET_VALUE: u32 = 0;
}
