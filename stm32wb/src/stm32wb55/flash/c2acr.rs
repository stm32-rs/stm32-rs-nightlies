#[doc = "Register `C2ACR` reader"]
pub type R = crate::R<C2ACRrs>;
#[doc = "Register `C2ACR` writer"]
pub type W = crate::W<C2ACRrs>;
#[doc = "Field `PRFTEN` reader - CPU2 cortex M0 prefetch enable"]
pub type PRFTEN_R = crate::BitReader;
#[doc = "Field `PRFTEN` writer - CPU2 cortex M0 prefetch enable"]
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEN` reader - CPU2 cortex M0 instruction cache enable"]
pub type ICEN_R = crate::BitReader;
#[doc = "Field `ICEN` writer - CPU2 cortex M0 instruction cache enable"]
pub type ICEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICRST` reader - CPU2 cortex M0 instruction cache reset"]
pub type ICRST_R = crate::BitReader;
#[doc = "Field `ICRST` writer - CPU2 cortex M0 instruction cache reset"]
pub type ICRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PES` reader - CPU2 cortex M0 program erase suspend request"]
pub type PES_R = crate::BitReader;
#[doc = "Field `PES` writer - CPU2 cortex M0 program erase suspend request"]
pub type PES_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - CPU2 cortex M0 prefetch enable"]
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU2 cortex M0 instruction cache enable"]
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU2 cortex M0 instruction cache reset"]
    #[inline(always)]
    pub fn icrst(&self) -> ICRST_R {
        ICRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU2 cortex M0 program erase suspend request"]
    #[inline(always)]
    pub fn pes(&self) -> PES_R {
        PES_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - CPU2 cortex M0 prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn prften(&mut self) -> PRFTEN_W<C2ACRrs> {
        PRFTEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - CPU2 cortex M0 instruction cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn icen(&mut self) -> ICEN_W<C2ACRrs> {
        ICEN_W::new(self, 9)
    }
    #[doc = "Bit 11 - CPU2 cortex M0 instruction cache reset"]
    #[inline(always)]
    #[must_use]
    pub fn icrst(&mut self) -> ICRST_W<C2ACRrs> {
        ICRST_W::new(self, 11)
    }
    #[doc = "Bit 15 - CPU2 cortex M0 program erase suspend request"]
    #[inline(always)]
    #[must_use]
    pub fn pes(&mut self) -> PES_W<C2ACRrs> {
        PES_W::new(self, 15)
    }
}
#[doc = "CPU2 cortex M0 access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2ACRrs;
impl crate::RegisterSpec for C2ACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2acr::R`](R) reader structure"]
impl crate::Readable for C2ACRrs {}
#[doc = "`write(|w| ..)` method takes [`c2acr::W`](W) writer structure"]
impl crate::Writable for C2ACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2ACR to value 0x0600"]
impl crate::Resettable for C2ACRrs {
    const RESET_VALUE: u32 = 0x0600;
}
