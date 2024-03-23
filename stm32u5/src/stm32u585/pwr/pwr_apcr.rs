#[doc = "Register `PWR_APCR` reader"]
pub type R = crate::R<PWR_APCRrs>;
#[doc = "Register `PWR_APCR` writer"]
pub type W = crate::W<PWR_APCRrs>;
#[doc = "Field `APC` reader - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PWR_PUCRx and PWR_PDCRx are applied. When this bit is cleared, PWR_PUCRx and PWR_PDCRx are not applied to the I/Os."]
pub type APC_R = crate::BitReader;
#[doc = "Field `APC` writer - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PWR_PUCRx and PWR_PDCRx are applied. When this bit is cleared, PWR_PUCRx and PWR_PDCRx are not applied to the I/Os."]
pub type APC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PWR_PUCRx and PWR_PDCRx are applied. When this bit is cleared, PWR_PUCRx and PWR_PDCRx are not applied to the I/Os."]
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in PWR_PUCRx and PWR_PDCRx are applied. When this bit is cleared, PWR_PUCRx and PWR_PDCRx are not applied to the I/Os."]
    #[inline(always)]
    #[must_use]
    pub fn apc(&mut self) -> APC_W<PWR_APCRrs> {
        APC_W::new(self, 0)
    }
}
#[doc = "PWR apply pull configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_apcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_apcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_APCRrs;
impl crate::RegisterSpec for PWR_APCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_apcr::R`](R) reader structure"]
impl crate::Readable for PWR_APCRrs {}
#[doc = "`write(|w| ..)` method takes [`pwr_apcr::W`](W) writer structure"]
impl crate::Writable for PWR_APCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_APCR to value 0"]
impl crate::Resettable for PWR_APCRrs {
    const RESET_VALUE: u32 = 0;
}
