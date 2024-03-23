#[doc = "Register `DOEPMSK` reader"]
pub type R = crate::R<DOEPMSKrs>;
#[doc = "Register `DOEPMSK` writer"]
pub type W = crate::W<DOEPMSKrs>;
#[doc = "Field `XFRCM` reader - Transfer completed interrupt mask"]
pub type XFRCM_R = crate::BitReader;
#[doc = "Field `XFRCM` writer - Transfer completed interrupt mask"]
pub type XFRCM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDM` reader - Endpoint disabled interrupt mask"]
pub type EPDM_R = crate::BitReader;
#[doc = "Field `EPDM` writer - Endpoint disabled interrupt mask"]
pub type EPDM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUPM` reader - SETUP phase done mask"]
pub type STUPM_R = crate::BitReader;
#[doc = "Field `STUPM` writer - SETUP phase done mask"]
pub type STUPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTEPDM` reader - OUT token received when endpoint disabled mask"]
pub type OTEPDM_R = crate::BitReader;
#[doc = "Field `OTEPDM` writer - OUT token received when endpoint disabled mask"]
pub type OTEPDM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn epdm(&self) -> EPDM_R {
        EPDM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP phase done mask"]
    #[inline(always)]
    pub fn stupm(&self) -> STUPM_R {
        STUPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled mask"]
    #[inline(always)]
    pub fn otepdm(&self) -> OTEPDM_R {
        OTEPDM_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfrcm(&mut self) -> XFRCM_W<DOEPMSKrs> {
        XFRCM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn epdm(&mut self) -> EPDM_W<DOEPMSKrs> {
        EPDM_W::new(self, 1)
    }
    #[doc = "Bit 3 - SETUP phase done mask"]
    #[inline(always)]
    #[must_use]
    pub fn stupm(&mut self) -> STUPM_W<DOEPMSKrs> {
        STUPM_W::new(self, 3)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled mask"]
    #[inline(always)]
    #[must_use]
    pub fn otepdm(&mut self) -> OTEPDM_W<DOEPMSKrs> {
        OTEPDM_W::new(self, 4)
    }
}
#[doc = "OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPMSKrs;
impl crate::RegisterSpec for DOEPMSKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepmsk::R`](R) reader structure"]
impl crate::Readable for DOEPMSKrs {}
#[doc = "`write(|w| ..)` method takes [`doepmsk::W`](W) writer structure"]
impl crate::Writable for DOEPMSKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPMSK to value 0"]
impl crate::Resettable for DOEPMSKrs {
    const RESET_VALUE: u32 = 0;
}
