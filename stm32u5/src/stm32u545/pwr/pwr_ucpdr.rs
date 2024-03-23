#[doc = "Register `PWR_UCPDR` reader"]
pub type R = crate::R<PWR_UCPDRrs>;
#[doc = "Register `PWR_UCPDR` writer"]
pub type W = crate::W<PWR_UCPDRrs>;
#[doc = "Field `UCPD_DBDIS` reader - UCPD dead battery disable After exiting reset, the USB Type-C “dead battery” behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to handover control to the UCPD (the UCPD must be initialized before doing the disable)."]
pub type UCPD_DBDIS_R = crate::BitReader;
#[doc = "Field `UCPD_DBDIS` writer - UCPD dead battery disable After exiting reset, the USB Type-C “dead battery” behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to handover control to the UCPD (the UCPD must be initialized before doing the disable)."]
pub type UCPD_DBDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPD_STBY` reader - UCPD Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD. It must be written to 0 after exiting the Standby mode and before writing any UCPD registers."]
pub type UCPD_STBY_R = crate::BitReader;
#[doc = "Field `UCPD_STBY` writer - UCPD Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD. It must be written to 0 after exiting the Standby mode and before writing any UCPD registers."]
pub type UCPD_STBY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UCPD dead battery disable After exiting reset, the USB Type-C “dead battery” behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to handover control to the UCPD (the UCPD must be initialized before doing the disable)."]
    #[inline(always)]
    pub fn ucpd_dbdis(&self) -> UCPD_DBDIS_R {
        UCPD_DBDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UCPD Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD. It must be written to 0 after exiting the Standby mode and before writing any UCPD registers."]
    #[inline(always)]
    pub fn ucpd_stby(&self) -> UCPD_STBY_R {
        UCPD_STBY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UCPD dead battery disable After exiting reset, the USB Type-C “dead battery” behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either to stop this pull-down or to handover control to the UCPD (the UCPD must be initialized before doing the disable)."]
    #[inline(always)]
    #[must_use]
    pub fn ucpd_dbdis(&mut self) -> UCPD_DBDIS_W<PWR_UCPDRrs> {
        UCPD_DBDIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - UCPD Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD. It must be written to 0 after exiting the Standby mode and before writing any UCPD registers."]
    #[inline(always)]
    #[must_use]
    pub fn ucpd_stby(&mut self) -> UCPD_STBY_W<PWR_UCPDRrs> {
        UCPD_STBY_W::new(self, 1)
    }
}
#[doc = "PWR USB Type-C™ and Power Delivery register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_ucpdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_ucpdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_UCPDRrs;
impl crate::RegisterSpec for PWR_UCPDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_ucpdr::R`](R) reader structure"]
impl crate::Readable for PWR_UCPDRrs {}
#[doc = "`write(|w| ..)` method takes [`pwr_ucpdr::W`](W) writer structure"]
impl crate::Writable for PWR_UCPDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_UCPDR to value 0"]
impl crate::Resettable for PWR_UCPDRrs {
    const RESET_VALUE: u32 = 0;
}
