#[doc = "Register `DMASBMR` reader"]
pub type R = crate::R<DMASBMRrs>;
#[doc = "Register `DMASBMR` writer"]
pub type W = crate::W<DMASBMRrs>;
#[doc = "Field `FB` reader - Fixed Burst Length"]
pub type FB_R = crate::BitReader;
#[doc = "Field `FB` writer - Fixed Burst Length"]
pub type FB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AAL` reader - Address-Aligned Beats"]
pub type AAL_R = crate::BitReader;
#[doc = "Field `AAL` writer - Address-Aligned Beats"]
pub type AAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB` reader - Mixed Burst"]
pub type MB_R = crate::BitReader;
#[doc = "Field `MB` writer - Mixed Burst"]
pub type MB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB` reader - Rebuild INCRx Burst"]
pub type RB_R = crate::BitReader;
#[doc = "Field `RB` writer - Rebuild INCRx Burst"]
pub type RB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fixed Burst Length"]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 12 - Address-Aligned Beats"]
    #[inline(always)]
    pub fn aal(&self) -> AAL_R {
        AAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Mixed Burst"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rebuild INCRx Burst"]
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fixed Burst Length"]
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FB_W<DMASBMRrs> {
        FB_W::new(self, 0)
    }
    #[doc = "Bit 12 - Address-Aligned Beats"]
    #[inline(always)]
    #[must_use]
    pub fn aal(&mut self) -> AAL_W<DMASBMRrs> {
        AAL_W::new(self, 12)
    }
    #[doc = "Bit 14 - Mixed Burst"]
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MB_W<DMASBMRrs> {
        MB_W::new(self, 14)
    }
    #[doc = "Bit 15 - Rebuild INCRx Burst"]
    #[inline(always)]
    #[must_use]
    pub fn rb(&mut self) -> RB_W<DMASBMRrs> {
        RB_W::new(self, 15)
    }
}
#[doc = "System bus mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmasbmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasbmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMASBMRrs;
impl crate::RegisterSpec for DMASBMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasbmr::R`](R) reader structure"]
impl crate::Readable for DMASBMRrs {}
#[doc = "`write(|w| ..)` method takes [`dmasbmr::W`](W) writer structure"]
impl crate::Writable for DMASBMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMASBMR to value 0x0101_0000"]
impl crate::Resettable for DMASBMRrs {
    const RESET_VALUE: u32 = 0x0101_0000;
}
