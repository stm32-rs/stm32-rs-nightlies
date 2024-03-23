#[doc = "Register `DOEPINT2` reader"]
pub type R = crate::R<DOEPINT2rs>;
#[doc = "Register `DOEPINT2` writer"]
pub type W = crate::W<DOEPINT2rs>;
#[doc = "Field `XFRC` reader - XFRC"]
pub type XFRC_R = crate::BitReader;
#[doc = "Field `XFRC` writer - XFRC"]
pub type XFRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISD` reader - EPDISD"]
pub type EPDISD_R = crate::BitReader;
#[doc = "Field `EPDISD` writer - EPDISD"]
pub type EPDISD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUP` reader - STUP"]
pub type STUP_R = crate::BitReader;
#[doc = "Field `STUP` writer - STUP"]
pub type STUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTEPDIS` reader - OTEPDIS"]
pub type OTEPDIS_R = crate::BitReader;
#[doc = "Field `OTEPDIS` writer - OTEPDIS"]
pub type OTEPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STSPHSRX` reader - Status phase received for control write"]
pub type STSPHSRX_R = crate::BitReader;
#[doc = "Field `STSPHSRX` writer - Status phase received for control write"]
pub type STSPHSRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERR` reader - Babble error interrupt"]
pub type BERR_R = crate::BitReader;
#[doc = "Field `BERR` writer - Babble error interrupt"]
pub type BERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK` reader - NAK input"]
pub type NAK_R = crate::BitReader;
#[doc = "Field `NAK` writer - NAK input"]
pub type NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EPDISD"]
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - STUP"]
    #[inline(always)]
    pub fn stup(&self) -> STUP_R {
        STUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OTEPDIS"]
    #[inline(always)]
    pub fn otepdis(&self) -> OTEPDIS_R {
        OTEPDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status phase received for control write"]
    #[inline(always)]
    pub fn stsphsrx(&self) -> STSPHSRX_R {
        STSPHSRX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - Babble error interrupt"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK input"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    #[must_use]
    pub fn xfrc(&mut self) -> XFRC_W<DOEPINT2rs> {
        XFRC_W::new(self, 0)
    }
    #[doc = "Bit 1 - EPDISD"]
    #[inline(always)]
    #[must_use]
    pub fn epdisd(&mut self) -> EPDISD_W<DOEPINT2rs> {
        EPDISD_W::new(self, 1)
    }
    #[doc = "Bit 3 - STUP"]
    #[inline(always)]
    #[must_use]
    pub fn stup(&mut self) -> STUP_W<DOEPINT2rs> {
        STUP_W::new(self, 3)
    }
    #[doc = "Bit 4 - OTEPDIS"]
    #[inline(always)]
    #[must_use]
    pub fn otepdis(&mut self) -> OTEPDIS_W<DOEPINT2rs> {
        OTEPDIS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Status phase received for control write"]
    #[inline(always)]
    #[must_use]
    pub fn stsphsrx(&mut self) -> STSPHSRX_W<DOEPINT2rs> {
        STSPHSRX_W::new(self, 5)
    }
    #[doc = "Bit 12 - Babble error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn berr(&mut self) -> BERR_W<DOEPINT2rs> {
        BERR_W::new(self, 12)
    }
    #[doc = "Bit 13 - NAK input"]
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NAK_W<DOEPINT2rs> {
        NAK_W::new(self, 13)
    }
}
#[doc = "device endpoint-2 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPINT2rs;
impl crate::RegisterSpec for DOEPINT2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepint2::R`](R) reader structure"]
impl crate::Readable for DOEPINT2rs {}
#[doc = "`write(|w| ..)` method takes [`doepint2::W`](W) writer structure"]
impl crate::Writable for DOEPINT2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPINT2 to value 0x80"]
impl crate::Resettable for DOEPINT2rs {
    const RESET_VALUE: u32 = 0x80;
}
