#[doc = "Register `OTG_GOTGCTL` reader"]
pub type R = crate::R<OTG_GOTGCTLrs>;
#[doc = "Register `OTG_GOTGCTL` writer"]
pub type W = crate::W<OTG_GOTGCTLrs>;
#[doc = "Field `SRQSCS` reader - SRQSCS"]
pub type SRQSCS_R = crate::BitReader;
#[doc = "Field `SRQ` reader - SRQ"]
pub type SRQ_R = crate::BitReader;
#[doc = "Field `SRQ` writer - SRQ"]
pub type SRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBVALOEN` reader - VBVALOEN"]
pub type VBVALOEN_R = crate::BitReader;
#[doc = "Field `VBVALOEN` writer - VBVALOEN"]
pub type VBVALOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBVALOVAL` reader - VBVALOVAL"]
pub type VBVALOVAL_R = crate::BitReader;
#[doc = "Field `VBVALOVAL` writer - VBVALOVAL"]
pub type VBVALOVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVALOEN` reader - AVALOEN"]
pub type AVALOEN_R = crate::BitReader;
#[doc = "Field `AVALOEN` writer - AVALOEN"]
pub type AVALOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVALOVAL` reader - AVALOVAL"]
pub type AVALOVAL_R = crate::BitReader;
#[doc = "Field `AVALOVAL` writer - AVALOVAL"]
pub type AVALOVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BVALOEN` reader - BVALOEN"]
pub type BVALOEN_R = crate::BitReader;
#[doc = "Field `BVALOEN` writer - BVALOEN"]
pub type BVALOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BVALOVAL` reader - BVALOVAL"]
pub type BVALOVAL_R = crate::BitReader;
#[doc = "Field `BVALOVAL` writer - BVALOVAL"]
pub type BVALOVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNGSCS` reader - HNGSCS"]
pub type HNGSCS_R = crate::BitReader;
#[doc = "Field `HNPRQ` reader - HNPRQ"]
pub type HNPRQ_R = crate::BitReader;
#[doc = "Field `HNPRQ` writer - HNPRQ"]
pub type HNPRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSHNPEN` reader - HSHNPEN"]
pub type HSHNPEN_R = crate::BitReader;
#[doc = "Field `HSHNPEN` writer - HSHNPEN"]
pub type HSHNPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DHNPEN` reader - DHNPEN"]
pub type DHNPEN_R = crate::BitReader;
#[doc = "Field `DHNPEN` writer - DHNPEN"]
pub type DHNPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EHEN` reader - EHEN"]
pub type EHEN_R = crate::BitReader;
#[doc = "Field `EHEN` writer - EHEN"]
pub type EHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIDSTS` reader - CIDSTS"]
pub type CIDSTS_R = crate::BitReader;
#[doc = "Field `DBCT` reader - DBCT"]
pub type DBCT_R = crate::BitReader;
#[doc = "Field `ASVLD` reader - ASVLD"]
pub type ASVLD_R = crate::BitReader;
#[doc = "Field `BSVLD` reader - BSVLD"]
pub type BSVLD_R = crate::BitReader;
#[doc = "Field `OTGVER` reader - OTGVER"]
pub type OTGVER_R = crate::BitReader;
#[doc = "Field `OTGVER` writer - OTGVER"]
pub type OTGVER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURMOD` reader - CURMOD"]
pub type CURMOD_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SRQSCS"]
    #[inline(always)]
    pub fn srqscs(&self) -> SRQSCS_R {
        SRQSCS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRQ"]
    #[inline(always)]
    pub fn srq(&self) -> SRQ_R {
        SRQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBVALOEN"]
    #[inline(always)]
    pub fn vbvaloen(&self) -> VBVALOEN_R {
        VBVALOEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VBVALOVAL"]
    #[inline(always)]
    pub fn vbvaloval(&self) -> VBVALOVAL_R {
        VBVALOVAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AVALOEN"]
    #[inline(always)]
    pub fn avaloen(&self) -> AVALOEN_R {
        AVALOEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AVALOVAL"]
    #[inline(always)]
    pub fn avaloval(&self) -> AVALOVAL_R {
        AVALOVAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BVALOEN"]
    #[inline(always)]
    pub fn bvaloen(&self) -> BVALOEN_R {
        BVALOEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BVALOVAL"]
    #[inline(always)]
    pub fn bvaloval(&self) -> BVALOVAL_R {
        BVALOVAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - HNGSCS"]
    #[inline(always)]
    pub fn hngscs(&self) -> HNGSCS_R {
        HNGSCS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNPRQ"]
    #[inline(always)]
    pub fn hnprq(&self) -> HNPRQ_R {
        HNPRQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSHNPEN"]
    #[inline(always)]
    pub fn hshnpen(&self) -> HSHNPEN_R {
        HSHNPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DHNPEN"]
    #[inline(always)]
    pub fn dhnpen(&self) -> DHNPEN_R {
        DHNPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EHEN"]
    #[inline(always)]
    pub fn ehen(&self) -> EHEN_R {
        EHEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - CIDSTS"]
    #[inline(always)]
    pub fn cidsts(&self) -> CIDSTS_R {
        CIDSTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DBCT"]
    #[inline(always)]
    pub fn dbct(&self) -> DBCT_R {
        DBCT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ASVLD"]
    #[inline(always)]
    pub fn asvld(&self) -> ASVLD_R {
        ASVLD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - BSVLD"]
    #[inline(always)]
    pub fn bsvld(&self) -> BSVLD_R {
        BSVLD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OTGVER"]
    #[inline(always)]
    pub fn otgver(&self) -> OTGVER_R {
        OTGVER_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CURMOD"]
    #[inline(always)]
    pub fn curmod(&self) -> CURMOD_R {
        CURMOD_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SRQ"]
    #[inline(always)]
    #[must_use]
    pub fn srq(&mut self) -> SRQ_W<OTG_GOTGCTLrs> {
        SRQ_W::new(self, 1)
    }
    #[doc = "Bit 2 - VBVALOEN"]
    #[inline(always)]
    #[must_use]
    pub fn vbvaloen(&mut self) -> VBVALOEN_W<OTG_GOTGCTLrs> {
        VBVALOEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - VBVALOVAL"]
    #[inline(always)]
    #[must_use]
    pub fn vbvaloval(&mut self) -> VBVALOVAL_W<OTG_GOTGCTLrs> {
        VBVALOVAL_W::new(self, 3)
    }
    #[doc = "Bit 4 - AVALOEN"]
    #[inline(always)]
    #[must_use]
    pub fn avaloen(&mut self) -> AVALOEN_W<OTG_GOTGCTLrs> {
        AVALOEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - AVALOVAL"]
    #[inline(always)]
    #[must_use]
    pub fn avaloval(&mut self) -> AVALOVAL_W<OTG_GOTGCTLrs> {
        AVALOVAL_W::new(self, 5)
    }
    #[doc = "Bit 6 - BVALOEN"]
    #[inline(always)]
    #[must_use]
    pub fn bvaloen(&mut self) -> BVALOEN_W<OTG_GOTGCTLrs> {
        BVALOEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - BVALOVAL"]
    #[inline(always)]
    #[must_use]
    pub fn bvaloval(&mut self) -> BVALOVAL_W<OTG_GOTGCTLrs> {
        BVALOVAL_W::new(self, 7)
    }
    #[doc = "Bit 9 - HNPRQ"]
    #[inline(always)]
    #[must_use]
    pub fn hnprq(&mut self) -> HNPRQ_W<OTG_GOTGCTLrs> {
        HNPRQ_W::new(self, 9)
    }
    #[doc = "Bit 10 - HSHNPEN"]
    #[inline(always)]
    #[must_use]
    pub fn hshnpen(&mut self) -> HSHNPEN_W<OTG_GOTGCTLrs> {
        HSHNPEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - DHNPEN"]
    #[inline(always)]
    #[must_use]
    pub fn dhnpen(&mut self) -> DHNPEN_W<OTG_GOTGCTLrs> {
        DHNPEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - EHEN"]
    #[inline(always)]
    #[must_use]
    pub fn ehen(&mut self) -> EHEN_W<OTG_GOTGCTLrs> {
        EHEN_W::new(self, 12)
    }
    #[doc = "Bit 20 - OTGVER"]
    #[inline(always)]
    #[must_use]
    pub fn otgver(&mut self) -> OTGVER_W<OTG_GOTGCTLrs> {
        OTGVER_W::new(self, 20)
    }
}
#[doc = "The OTG_GOTGCTL register controls the behavior and reflects the status of the OTG function of the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_gotgctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_gotgctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_GOTGCTLrs;
impl crate::RegisterSpec for OTG_GOTGCTLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_gotgctl::R`](R) reader structure"]
impl crate::Readable for OTG_GOTGCTLrs {}
#[doc = "`write(|w| ..)` method takes [`otg_gotgctl::W`](W) writer structure"]
impl crate::Writable for OTG_GOTGCTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_GOTGCTL to value 0x0001_0000"]
impl crate::Resettable for OTG_GOTGCTLrs {
    const RESET_VALUE: u32 = 0x0001_0000;
}
