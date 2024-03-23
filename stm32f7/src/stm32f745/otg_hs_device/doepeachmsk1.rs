#[doc = "Register `DOEPEACHMSK1` reader"]
pub type R = crate::R<DOEPEACHMSK1rs>;
#[doc = "Register `DOEPEACHMSK1` writer"]
pub type W = crate::W<DOEPEACHMSK1rs>;
#[doc = "Field `XFRCM` reader - Transfer completed interrupt mask"]
pub type XFRCM_R = crate::BitReader;
#[doc = "Field `XFRCM` writer - Transfer completed interrupt mask"]
pub type XFRCM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDM` reader - Endpoint disabled interrupt mask"]
pub type EPDM_R = crate::BitReader;
#[doc = "Field `EPDM` writer - Endpoint disabled interrupt mask"]
pub type EPDM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERRM` reader - AHB error mask"]
pub type AHBERRM_R = crate::BitReader;
#[doc = "Field `AHBERRM` writer - AHB error mask"]
pub type AHBERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUPM` reader - SETUP phase done mask"]
pub type STUPM_R = crate::BitReader;
#[doc = "Field `STUPM` writer - SETUP phase done mask"]
pub type STUPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTEPDM` reader - OUT token received when endpoint disabled mask"]
pub type OTEPDM_R = crate::BitReader;
#[doc = "Field `OTEPDM` writer - OUT token received when endpoint disabled mask"]
pub type OTEPDM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B2BSTUPM` reader - Back-to-back SETUP packets received mask"]
pub type B2BSTUPM_R = crate::BitReader;
#[doc = "Field `B2BSTUPM` writer - Back-to-back SETUP packets received mask"]
pub type B2BSTUPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPKTERRM` reader - Out packet error mask"]
pub type OUTPKTERRM_R = crate::BitReader;
#[doc = "Field `OUTPKTERRM` writer - Out packet error mask"]
pub type OUTPKTERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAM` reader - BNA interrupt mask"]
pub type BNAM_R = crate::BitReader;
#[doc = "Field `BNAM` writer - BNA interrupt mask"]
pub type BNAM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERRM` reader - Babble error interrupt mask"]
pub type BERRM_R = crate::BitReader;
#[doc = "Field `BERRM` writer - Babble error interrupt mask"]
pub type BERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKMSK` reader - NAK interrupt mask"]
pub type NAKMSK_R = crate::BitReader;
#[doc = "Field `NAKMSK` writer - NAK interrupt mask"]
pub type NAKMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYETMSK` reader - NYET interrupt mask"]
pub type NYETMSK_R = crate::BitReader;
#[doc = "Field `NYETMSK` writer - NYET interrupt mask"]
pub type NYETMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - AHB error mask"]
    #[inline(always)]
    pub fn ahberrm(&self) -> AHBERRM_R {
        AHBERRM_R::new(((self.bits >> 2) & 1) != 0)
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
    #[doc = "Bit 6 - Back-to-back SETUP packets received mask"]
    #[inline(always)]
    pub fn b2bstupm(&self) -> B2BSTUPM_R {
        B2BSTUPM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Out packet error mask"]
    #[inline(always)]
    pub fn outpkterrm(&self) -> OUTPKTERRM_R {
        OUTPKTERRM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    pub fn bnam(&self) -> BNAM_R {
        BNAM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Babble error interrupt mask"]
    #[inline(always)]
    pub fn berrm(&self) -> BERRM_R {
        BERRM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt mask"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NYET interrupt mask"]
    #[inline(always)]
    pub fn nyetmsk(&self) -> NYETMSK_R {
        NYETMSK_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfrcm(&mut self) -> XFRCM_W<DOEPEACHMSK1rs> {
        XFRCM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn epdm(&mut self) -> EPDM_W<DOEPEACHMSK1rs> {
        EPDM_W::new(self, 1)
    }
    #[doc = "Bit 2 - AHB error mask"]
    #[inline(always)]
    #[must_use]
    pub fn ahberrm(&mut self) -> AHBERRM_W<DOEPEACHMSK1rs> {
        AHBERRM_W::new(self, 2)
    }
    #[doc = "Bit 3 - SETUP phase done mask"]
    #[inline(always)]
    #[must_use]
    pub fn stupm(&mut self) -> STUPM_W<DOEPEACHMSK1rs> {
        STUPM_W::new(self, 3)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled mask"]
    #[inline(always)]
    #[must_use]
    pub fn otepdm(&mut self) -> OTEPDM_W<DOEPEACHMSK1rs> {
        OTEPDM_W::new(self, 4)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received mask"]
    #[inline(always)]
    #[must_use]
    pub fn b2bstupm(&mut self) -> B2BSTUPM_W<DOEPEACHMSK1rs> {
        B2BSTUPM_W::new(self, 6)
    }
    #[doc = "Bit 8 - Out packet error mask"]
    #[inline(always)]
    #[must_use]
    pub fn outpkterrm(&mut self) -> OUTPKTERRM_W<DOEPEACHMSK1rs> {
        OUTPKTERRM_W::new(self, 8)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn bnam(&mut self) -> BNAM_W<DOEPEACHMSK1rs> {
        BNAM_W::new(self, 9)
    }
    #[doc = "Bit 12 - Babble error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn berrm(&mut self) -> BERRM_W<DOEPEACHMSK1rs> {
        BERRM_W::new(self, 12)
    }
    #[doc = "Bit 13 - NAK interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn nakmsk(&mut self) -> NAKMSK_W<DOEPEACHMSK1rs> {
        NAKMSK_W::new(self, 13)
    }
    #[doc = "Bit 14 - NYET interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn nyetmsk(&mut self) -> NYETMSK_W<DOEPEACHMSK1rs> {
        NYETMSK_W::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepeachmsk1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepeachmsk1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPEACHMSK1rs;
impl crate::RegisterSpec for DOEPEACHMSK1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepeachmsk1::R`](R) reader structure"]
impl crate::Readable for DOEPEACHMSK1rs {}
#[doc = "`write(|w| ..)` method takes [`doepeachmsk1::W`](W) writer structure"]
impl crate::Writable for DOEPEACHMSK1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPEACHMSK1 to value 0"]
impl crate::Resettable for DOEPEACHMSK1rs {
    const RESET_VALUE: u32 = 0;
}
