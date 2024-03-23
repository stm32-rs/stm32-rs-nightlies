#[doc = "Register `GOTGCTL` reader"]
pub type R = crate::R<GOTGCTLrs>;
#[doc = "Register `GOTGCTL` writer"]
pub type W = crate::W<GOTGCTLrs>;
#[doc = "Field `SRQSCS` reader - Session request success"]
pub type SRQSCS_R = crate::BitReader;
#[doc = "Field `SRQ` reader - Session request"]
pub type SRQ_R = crate::BitReader;
#[doc = "Field `SRQ` writer - Session request"]
pub type SRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBVALOEN` reader - VBUS valid override enable"]
pub type VBVALOEN_R = crate::BitReader;
#[doc = "Field `VBVALOEN` writer - VBUS valid override enable"]
pub type VBVALOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBVALOVA` reader - VBUS valid override value"]
pub type VBVALOVA_R = crate::BitReader;
#[doc = "Field `VBVALOVA` writer - VBUS valid override value"]
pub type VBVALOVA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVALOEN` reader - A-peripheral session valid override enable"]
pub type AVALOEN_R = crate::BitReader;
#[doc = "Field `AVALOEN` writer - A-peripheral session valid override enable"]
pub type AVALOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVALOVAL` reader - A-peripheral session valid override value"]
pub type AVALOVAL_R = crate::BitReader;
#[doc = "Field `AVALOVAL` writer - A-peripheral session valid override value"]
pub type AVALOVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BVALOEN` reader - B-peripheral session valid override enable"]
pub type BVALOEN_R = crate::BitReader;
#[doc = "Field `BVALOEN` writer - B-peripheral session valid override enable"]
pub type BVALOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BVALOVAL` reader - B-peripheral session valid override value"]
pub type BVALOVAL_R = crate::BitReader;
#[doc = "Field `BVALOVAL` writer - B-peripheral session valid override value"]
pub type BVALOVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNGSCS` reader - Host negotiation success"]
pub type HNGSCS_R = crate::BitReader;
#[doc = "Field `HNPRQ` reader - HNP request"]
pub type HNPRQ_R = crate::BitReader;
#[doc = "Field `HNPRQ` writer - HNP request"]
pub type HNPRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSHNPEN` reader - Host set HNP enable"]
pub type HSHNPEN_R = crate::BitReader;
#[doc = "Field `HSHNPEN` writer - Host set HNP enable"]
pub type HSHNPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DHNPEN` reader - Device HNP enabled"]
pub type DHNPEN_R = crate::BitReader;
#[doc = "Field `DHNPEN` writer - Device HNP enabled"]
pub type DHNPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EHEN` reader - Embedded host enable"]
pub type EHEN_R = crate::BitReader;
#[doc = "Field `EHEN` writer - Embedded host enable"]
pub type EHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIDSTS` reader - Connector ID status"]
pub type CIDSTS_R = crate::BitReader;
#[doc = "Field `DBCT` reader - Long/short debounce time"]
pub type DBCT_R = crate::BitReader;
#[doc = "Field `ASVLD` reader - A-session valid"]
pub type ASVLD_R = crate::BitReader;
#[doc = "Field `BSVLD` reader - B-session valid"]
pub type BSVLD_R = crate::BitReader;
#[doc = "Field `OTGVER` reader - OTG version"]
pub type OTGVER_R = crate::BitReader;
#[doc = "Field `OTGVER` writer - OTG version"]
pub type OTGVER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURMOD` reader - Current mode of operation"]
pub type CURMOD_R = crate::BitReader;
#[doc = "Field `CURMOD` writer - Current mode of operation"]
pub type CURMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Session request success"]
    #[inline(always)]
    pub fn srqscs(&self) -> SRQSCS_R {
        SRQSCS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Session request"]
    #[inline(always)]
    pub fn srq(&self) -> SRQ_R {
        SRQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBUS valid override enable"]
    #[inline(always)]
    pub fn vbvaloen(&self) -> VBVALOEN_R {
        VBVALOEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VBUS valid override value"]
    #[inline(always)]
    pub fn vbvalova(&self) -> VBVALOVA_R {
        VBVALOVA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A-peripheral session valid override enable"]
    #[inline(always)]
    pub fn avaloen(&self) -> AVALOEN_R {
        AVALOEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A-peripheral session valid override value"]
    #[inline(always)]
    pub fn avaloval(&self) -> AVALOVAL_R {
        AVALOVAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - B-peripheral session valid override enable"]
    #[inline(always)]
    pub fn bvaloen(&self) -> BVALOEN_R {
        BVALOEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - B-peripheral session valid override value"]
    #[inline(always)]
    pub fn bvaloval(&self) -> BVALOVAL_R {
        BVALOVAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Host negotiation success"]
    #[inline(always)]
    pub fn hngscs(&self) -> HNGSCS_R {
        HNGSCS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    pub fn hnprq(&self) -> HNPRQ_R {
        HNPRQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Host set HNP enable"]
    #[inline(always)]
    pub fn hshnpen(&self) -> HSHNPEN_R {
        HSHNPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    pub fn dhnpen(&self) -> DHNPEN_R {
        DHNPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Embedded host enable"]
    #[inline(always)]
    pub fn ehen(&self) -> EHEN_R {
        EHEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Connector ID status"]
    #[inline(always)]
    pub fn cidsts(&self) -> CIDSTS_R {
        CIDSTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Long/short debounce time"]
    #[inline(always)]
    pub fn dbct(&self) -> DBCT_R {
        DBCT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A-session valid"]
    #[inline(always)]
    pub fn asvld(&self) -> ASVLD_R {
        ASVLD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - B-session valid"]
    #[inline(always)]
    pub fn bsvld(&self) -> BSVLD_R {
        BSVLD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OTG version"]
    #[inline(always)]
    pub fn otgver(&self) -> OTGVER_R {
        OTGVER_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Current mode of operation"]
    #[inline(always)]
    pub fn curmod(&self) -> CURMOD_R {
        CURMOD_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Session request"]
    #[inline(always)]
    #[must_use]
    pub fn srq(&mut self) -> SRQ_W<GOTGCTLrs> {
        SRQ_W::new(self, 1)
    }
    #[doc = "Bit 2 - VBUS valid override enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbvaloen(&mut self) -> VBVALOEN_W<GOTGCTLrs> {
        VBVALOEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - VBUS valid override value"]
    #[inline(always)]
    #[must_use]
    pub fn vbvalova(&mut self) -> VBVALOVA_W<GOTGCTLrs> {
        VBVALOVA_W::new(self, 3)
    }
    #[doc = "Bit 4 - A-peripheral session valid override enable"]
    #[inline(always)]
    #[must_use]
    pub fn avaloen(&mut self) -> AVALOEN_W<GOTGCTLrs> {
        AVALOEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - A-peripheral session valid override value"]
    #[inline(always)]
    #[must_use]
    pub fn avaloval(&mut self) -> AVALOVAL_W<GOTGCTLrs> {
        AVALOVAL_W::new(self, 5)
    }
    #[doc = "Bit 6 - B-peripheral session valid override enable"]
    #[inline(always)]
    #[must_use]
    pub fn bvaloen(&mut self) -> BVALOEN_W<GOTGCTLrs> {
        BVALOEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - B-peripheral session valid override value"]
    #[inline(always)]
    #[must_use]
    pub fn bvaloval(&mut self) -> BVALOVAL_W<GOTGCTLrs> {
        BVALOVAL_W::new(self, 7)
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    #[must_use]
    pub fn hnprq(&mut self) -> HNPRQ_W<GOTGCTLrs> {
        HNPRQ_W::new(self, 9)
    }
    #[doc = "Bit 10 - Host set HNP enable"]
    #[inline(always)]
    #[must_use]
    pub fn hshnpen(&mut self) -> HSHNPEN_W<GOTGCTLrs> {
        HSHNPEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dhnpen(&mut self) -> DHNPEN_W<GOTGCTLrs> {
        DHNPEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Embedded host enable"]
    #[inline(always)]
    #[must_use]
    pub fn ehen(&mut self) -> EHEN_W<GOTGCTLrs> {
        EHEN_W::new(self, 12)
    }
    #[doc = "Bit 20 - OTG version"]
    #[inline(always)]
    #[must_use]
    pub fn otgver(&mut self) -> OTGVER_W<GOTGCTLrs> {
        OTGVER_W::new(self, 20)
    }
    #[doc = "Bit 21 - Current mode of operation"]
    #[inline(always)]
    #[must_use]
    pub fn curmod(&mut self) -> CURMOD_W<GOTGCTLrs> {
        CURMOD_W::new(self, 21)
    }
}
#[doc = "OTG_FS control and status register (OTG_FS_GOTGCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GOTGCTLrs;
impl crate::RegisterSpec for GOTGCTLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gotgctl::R`](R) reader structure"]
impl crate::Readable for GOTGCTLrs {}
#[doc = "`write(|w| ..)` method takes [`gotgctl::W`](W) writer structure"]
impl crate::Writable for GOTGCTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GOTGCTL to value 0x0800"]
impl crate::Resettable for GOTGCTLrs {
    const RESET_VALUE: u32 = 0x0800;
}
