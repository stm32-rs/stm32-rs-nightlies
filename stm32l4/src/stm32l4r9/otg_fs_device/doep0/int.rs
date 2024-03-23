#[doc = "Register `INT` reader"]
pub type R = crate::R<INTrs>;
#[doc = "Register `INT` writer"]
pub type W = crate::W<INTrs>;
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
#[doc = "Field `B2BSTUP` reader - B2BSTUP"]
pub type B2BSTUP_R = crate::BitReader;
#[doc = "Field `B2BSTUP` writer - B2BSTUP"]
pub type B2BSTUP_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 6 - B2BSTUP"]
    #[inline(always)]
    pub fn b2bstup(&self) -> B2BSTUP_R {
        B2BSTUP_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    #[must_use]
    pub fn xfrc(&mut self) -> XFRC_W<INTrs> {
        XFRC_W::new(self, 0)
    }
    #[doc = "Bit 1 - EPDISD"]
    #[inline(always)]
    #[must_use]
    pub fn epdisd(&mut self) -> EPDISD_W<INTrs> {
        EPDISD_W::new(self, 1)
    }
    #[doc = "Bit 3 - STUP"]
    #[inline(always)]
    #[must_use]
    pub fn stup(&mut self) -> STUP_W<INTrs> {
        STUP_W::new(self, 3)
    }
    #[doc = "Bit 4 - OTEPDIS"]
    #[inline(always)]
    #[must_use]
    pub fn otepdis(&mut self) -> OTEPDIS_W<INTrs> {
        OTEPDIS_W::new(self, 4)
    }
    #[doc = "Bit 6 - B2BSTUP"]
    #[inline(always)]
    #[must_use]
    pub fn b2bstup(&mut self) -> B2BSTUP_W<INTrs> {
        B2BSTUP_W::new(self, 6)
    }
}
#[doc = "device endpoint-0 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets INT to value 0x80"]
impl crate::Resettable for INTrs {
    const RESET_VALUE: u32 = 0x80;
}
