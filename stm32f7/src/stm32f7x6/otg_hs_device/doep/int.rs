#[doc = "Register `INT` reader"]
pub type R = crate::R<INTrs>;
#[doc = "Register `INT` writer"]
pub type W = crate::W<INTrs>;
#[doc = "Field `XFRC` reader - Transfer completed interrupt"]
pub type XFRC_R = crate::BitReader;
#[doc = "Field `XFRC` writer - Transfer completed interrupt"]
pub type XFRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISD` reader - Endpoint disabled interrupt"]
pub type EPDISD_R = crate::BitReader;
#[doc = "Field `EPDISD` writer - Endpoint disabled interrupt"]
pub type EPDISD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUP` reader - SETUP phase done"]
pub type STUP_R = crate::BitReader;
#[doc = "Field `STUP` writer - SETUP phase done"]
pub type STUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTEPDIS` reader - OUT token received when endpoint disabled"]
pub type OTEPDIS_R = crate::BitReader;
#[doc = "Field `OTEPDIS` writer - OUT token received when endpoint disabled"]
pub type OTEPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B2BSTUP` reader - Back-to-back SETUP packets received"]
pub type B2BSTUP_R = crate::BitReader;
#[doc = "Field `B2BSTUP` writer - Back-to-back SETUP packets received"]
pub type B2BSTUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYET` reader - NYET interrupt"]
pub type NYET_R = crate::BitReader;
#[doc = "Field `NYET` writer - NYET interrupt"]
pub type NYET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP phase done"]
    #[inline(always)]
    pub fn stup(&self) -> STUP_R {
        STUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled"]
    #[inline(always)]
    pub fn otepdis(&self) -> OTEPDIS_R {
        OTEPDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received"]
    #[inline(always)]
    pub fn b2bstup(&self) -> B2BSTUP_R {
        B2BSTUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xfrc(&mut self) -> XFRC_W<INTrs> {
        XFRC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn epdisd(&mut self) -> EPDISD_W<INTrs> {
        EPDISD_W::new(self, 1)
    }
    #[doc = "Bit 3 - SETUP phase done"]
    #[inline(always)]
    #[must_use]
    pub fn stup(&mut self) -> STUP_W<INTrs> {
        STUP_W::new(self, 3)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled"]
    #[inline(always)]
    #[must_use]
    pub fn otepdis(&mut self) -> OTEPDIS_W<INTrs> {
        OTEPDIS_W::new(self, 4)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received"]
    #[inline(always)]
    #[must_use]
    pub fn b2bstup(&mut self) -> B2BSTUP_W<INTrs> {
        B2BSTUP_W::new(self, 6)
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nyet(&mut self) -> NYET_W<INTrs> {
        NYET_W::new(self, 14)
    }
}
#[doc = "OTG_HS device endpoint-1 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTrs;
impl crate::RegisterSpec for INTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int::R`](R) reader structure"]
impl crate::Readable for INTrs {}
#[doc = "`write(|w| ..)` method takes [`int::W`](W) writer structure"]
impl crate::Writable for INTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT to value 0"]
impl crate::Resettable for INTrs {
    const RESET_VALUE: u32 = 0;
}
