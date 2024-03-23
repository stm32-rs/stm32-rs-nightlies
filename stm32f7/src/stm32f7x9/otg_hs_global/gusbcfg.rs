#[doc = "Register `GUSBCFG` reader"]
pub type R = crate::R<GUSBCFGrs>;
#[doc = "Register `GUSBCFG` writer"]
pub type W = crate::W<GUSBCFGrs>;
#[doc = "Field `TOCAL` reader - FS timeout calibration"]
pub type TOCAL_R = crate::FieldReader;
#[doc = "Field `TOCAL` writer - FS timeout calibration"]
pub type TOCAL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHYSEL` writer - USB 2.0 high-speed ULPI PHY or USB 1.1 full-speed serial transceiver select"]
pub type PHYSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRPCAP` reader - SRP-capable"]
pub type SRPCAP_R = crate::BitReader;
#[doc = "Field `SRPCAP` writer - SRP-capable"]
pub type SRPCAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNPCAP` reader - HNP-capable"]
pub type HNPCAP_R = crate::BitReader;
#[doc = "Field `HNPCAP` writer - HNP-capable"]
pub type HNPCAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRDT` reader - USB turnaround time"]
pub type TRDT_R = crate::FieldReader;
#[doc = "Field `TRDT` writer - USB turnaround time"]
pub type TRDT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHYLPCS` reader - PHY Low-power clock select"]
pub type PHYLPCS_R = crate::BitReader;
#[doc = "Field `PHYLPCS` writer - PHY Low-power clock select"]
pub type PHYLPCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULPIFSLS` reader - ULPI FS/LS select"]
pub type ULPIFSLS_R = crate::BitReader;
#[doc = "Field `ULPIFSLS` writer - ULPI FS/LS select"]
pub type ULPIFSLS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULPIAR` reader - ULPI Auto-resume"]
pub type ULPIAR_R = crate::BitReader;
#[doc = "Field `ULPIAR` writer - ULPI Auto-resume"]
pub type ULPIAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULPICSM` reader - ULPI Clock SuspendM"]
pub type ULPICSM_R = crate::BitReader;
#[doc = "Field `ULPICSM` writer - ULPI Clock SuspendM"]
pub type ULPICSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULPIEVBUSD` reader - ULPI External VBUS Drive"]
pub type ULPIEVBUSD_R = crate::BitReader;
#[doc = "Field `ULPIEVBUSD` writer - ULPI External VBUS Drive"]
pub type ULPIEVBUSD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULPIEVBUSI` reader - ULPI external VBUS indicator"]
pub type ULPIEVBUSI_R = crate::BitReader;
#[doc = "Field `ULPIEVBUSI` writer - ULPI external VBUS indicator"]
pub type ULPIEVBUSI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSDPS` reader - TermSel DLine pulsing selection"]
pub type TSDPS_R = crate::BitReader;
#[doc = "Field `TSDPS` writer - TermSel DLine pulsing selection"]
pub type TSDPS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCCI` reader - Indicator complement"]
pub type PCCI_R = crate::BitReader;
#[doc = "Field `PCCI` writer - Indicator complement"]
pub type PCCI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTCI` reader - Indicator pass through"]
pub type PTCI_R = crate::BitReader;
#[doc = "Field `PTCI` writer - Indicator pass through"]
pub type PTCI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULPIIPD` reader - ULPI interface protect disable"]
pub type ULPIIPD_R = crate::BitReader;
#[doc = "Field `ULPIIPD` writer - ULPI interface protect disable"]
pub type ULPIIPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FHMOD` reader - Forced host mode"]
pub type FHMOD_R = crate::BitReader;
#[doc = "Field `FHMOD` writer - Forced host mode"]
pub type FHMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDMOD` reader - Forced peripheral mode"]
pub type FDMOD_R = crate::BitReader;
#[doc = "Field `FDMOD` writer - Forced peripheral mode"]
pub type FDMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - FS timeout calibration"]
    #[inline(always)]
    pub fn tocal(&self) -> TOCAL_R {
        TOCAL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - SRP-capable"]
    #[inline(always)]
    pub fn srpcap(&self) -> SRPCAP_R {
        SRPCAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP-capable"]
    #[inline(always)]
    pub fn hnpcap(&self) -> HNPCAP_R {
        HNPCAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    pub fn trdt(&self) -> TRDT_R {
        TRDT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - PHY Low-power clock select"]
    #[inline(always)]
    pub fn phylpcs(&self) -> PHYLPCS_R {
        PHYLPCS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - ULPI FS/LS select"]
    #[inline(always)]
    pub fn ulpifsls(&self) -> ULPIFSLS_R {
        ULPIFSLS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ULPI Auto-resume"]
    #[inline(always)]
    pub fn ulpiar(&self) -> ULPIAR_R {
        ULPIAR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ULPI Clock SuspendM"]
    #[inline(always)]
    pub fn ulpicsm(&self) -> ULPICSM_R {
        ULPICSM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ULPI External VBUS Drive"]
    #[inline(always)]
    pub fn ulpievbusd(&self) -> ULPIEVBUSD_R {
        ULPIEVBUSD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ULPI external VBUS indicator"]
    #[inline(always)]
    pub fn ulpievbusi(&self) -> ULPIEVBUSI_R {
        ULPIEVBUSI_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TermSel DLine pulsing selection"]
    #[inline(always)]
    pub fn tsdps(&self) -> TSDPS_R {
        TSDPS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Indicator complement"]
    #[inline(always)]
    pub fn pcci(&self) -> PCCI_R {
        PCCI_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Indicator pass through"]
    #[inline(always)]
    pub fn ptci(&self) -> PTCI_R {
        PTCI_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ULPI interface protect disable"]
    #[inline(always)]
    pub fn ulpiipd(&self) -> ULPIIPD_R {
        ULPIIPD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - Forced host mode"]
    #[inline(always)]
    pub fn fhmod(&self) -> FHMOD_R {
        FHMOD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Forced peripheral mode"]
    #[inline(always)]
    pub fn fdmod(&self) -> FDMOD_R {
        FDMOD_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - FS timeout calibration"]
    #[inline(always)]
    #[must_use]
    pub fn tocal(&mut self) -> TOCAL_W<GUSBCFGrs> {
        TOCAL_W::new(self, 0)
    }
    #[doc = "Bit 6 - USB 2.0 high-speed ULPI PHY or USB 1.1 full-speed serial transceiver select"]
    #[inline(always)]
    #[must_use]
    pub fn physel(&mut self) -> PHYSEL_W<GUSBCFGrs> {
        PHYSEL_W::new(self, 6)
    }
    #[doc = "Bit 8 - SRP-capable"]
    #[inline(always)]
    #[must_use]
    pub fn srpcap(&mut self) -> SRPCAP_W<GUSBCFGrs> {
        SRPCAP_W::new(self, 8)
    }
    #[doc = "Bit 9 - HNP-capable"]
    #[inline(always)]
    #[must_use]
    pub fn hnpcap(&mut self) -> HNPCAP_W<GUSBCFGrs> {
        HNPCAP_W::new(self, 9)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    #[must_use]
    pub fn trdt(&mut self) -> TRDT_W<GUSBCFGrs> {
        TRDT_W::new(self, 10)
    }
    #[doc = "Bit 15 - PHY Low-power clock select"]
    #[inline(always)]
    #[must_use]
    pub fn phylpcs(&mut self) -> PHYLPCS_W<GUSBCFGrs> {
        PHYLPCS_W::new(self, 15)
    }
    #[doc = "Bit 17 - ULPI FS/LS select"]
    #[inline(always)]
    #[must_use]
    pub fn ulpifsls(&mut self) -> ULPIFSLS_W<GUSBCFGrs> {
        ULPIFSLS_W::new(self, 17)
    }
    #[doc = "Bit 18 - ULPI Auto-resume"]
    #[inline(always)]
    #[must_use]
    pub fn ulpiar(&mut self) -> ULPIAR_W<GUSBCFGrs> {
        ULPIAR_W::new(self, 18)
    }
    #[doc = "Bit 19 - ULPI Clock SuspendM"]
    #[inline(always)]
    #[must_use]
    pub fn ulpicsm(&mut self) -> ULPICSM_W<GUSBCFGrs> {
        ULPICSM_W::new(self, 19)
    }
    #[doc = "Bit 20 - ULPI External VBUS Drive"]
    #[inline(always)]
    #[must_use]
    pub fn ulpievbusd(&mut self) -> ULPIEVBUSD_W<GUSBCFGrs> {
        ULPIEVBUSD_W::new(self, 20)
    }
    #[doc = "Bit 21 - ULPI external VBUS indicator"]
    #[inline(always)]
    #[must_use]
    pub fn ulpievbusi(&mut self) -> ULPIEVBUSI_W<GUSBCFGrs> {
        ULPIEVBUSI_W::new(self, 21)
    }
    #[doc = "Bit 22 - TermSel DLine pulsing selection"]
    #[inline(always)]
    #[must_use]
    pub fn tsdps(&mut self) -> TSDPS_W<GUSBCFGrs> {
        TSDPS_W::new(self, 22)
    }
    #[doc = "Bit 23 - Indicator complement"]
    #[inline(always)]
    #[must_use]
    pub fn pcci(&mut self) -> PCCI_W<GUSBCFGrs> {
        PCCI_W::new(self, 23)
    }
    #[doc = "Bit 24 - Indicator pass through"]
    #[inline(always)]
    #[must_use]
    pub fn ptci(&mut self) -> PTCI_W<GUSBCFGrs> {
        PTCI_W::new(self, 24)
    }
    #[doc = "Bit 25 - ULPI interface protect disable"]
    #[inline(always)]
    #[must_use]
    pub fn ulpiipd(&mut self) -> ULPIIPD_W<GUSBCFGrs> {
        ULPIIPD_W::new(self, 25)
    }
    #[doc = "Bit 29 - Forced host mode"]
    #[inline(always)]
    #[must_use]
    pub fn fhmod(&mut self) -> FHMOD_W<GUSBCFGrs> {
        FHMOD_W::new(self, 29)
    }
    #[doc = "Bit 30 - Forced peripheral mode"]
    #[inline(always)]
    #[must_use]
    pub fn fdmod(&mut self) -> FDMOD_W<GUSBCFGrs> {
        FDMOD_W::new(self, 30)
    }
}
#[doc = "OTG_HS USB configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GUSBCFGrs;
impl crate::RegisterSpec for GUSBCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gusbcfg::R`](R) reader structure"]
impl crate::Readable for GUSBCFGrs {}
#[doc = "`write(|w| ..)` method takes [`gusbcfg::W`](W) writer structure"]
impl crate::Writable for GUSBCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GUSBCFG to value 0x0a00"]
impl crate::Resettable for GUSBCFGrs {
    const RESET_VALUE: u32 = 0x0a00;
}
