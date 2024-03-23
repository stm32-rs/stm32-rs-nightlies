#[doc = "Register `MPCBB2_CR` reader"]
pub type R = crate::R<MPCBB2_CRrs>;
#[doc = "Register `MPCBB2_CR` writer"]
pub type W = crate::W<MPCBB2_CRrs>;
#[doc = "Field `LCK` reader - LCK"]
pub type LCK_R = crate::BitReader;
#[doc = "Field `LCK` writer - LCK"]
pub type LCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVSECSTATE` reader - INVSECSTATE"]
pub type INVSECSTATE_R = crate::BitReader;
#[doc = "Field `INVSECSTATE` writer - INVSECSTATE"]
pub type INVSECSTATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRWILADIS` reader - SRWILADIS"]
pub type SRWILADIS_R = crate::BitReader;
#[doc = "Field `SRWILADIS` writer - SRWILADIS"]
pub type SRWILADIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LCK"]
    #[inline(always)]
    pub fn lck(&self) -> LCK_R {
        LCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 30 - INVSECSTATE"]
    #[inline(always)]
    pub fn invsecstate(&self) -> INVSECSTATE_R {
        INVSECSTATE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SRWILADIS"]
    #[inline(always)]
    pub fn srwiladis(&self) -> SRWILADIS_R {
        SRWILADIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCK"]
    #[inline(always)]
    #[must_use]
    pub fn lck(&mut self) -> LCK_W<MPCBB2_CRrs> {
        LCK_W::new(self, 0)
    }
    #[doc = "Bit 30 - INVSECSTATE"]
    #[inline(always)]
    #[must_use]
    pub fn invsecstate(&mut self) -> INVSECSTATE_W<MPCBB2_CRrs> {
        INVSECSTATE_W::new(self, 30)
    }
    #[doc = "Bit 31 - SRWILADIS"]
    #[inline(always)]
    #[must_use]
    pub fn srwiladis(&mut self) -> SRWILADIS_W<MPCBB2_CRrs> {
        SRWILADIS_W::new(self, 31)
    }
}
#[doc = "MPCBB control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb2_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb2_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPCBB2_CRrs;
impl crate::RegisterSpec for MPCBB2_CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpcbb2_cr::R`](R) reader structure"]
impl crate::Readable for MPCBB2_CRrs {}
#[doc = "`write(|w| ..)` method takes [`mpcbb2_cr::W`](W) writer structure"]
impl crate::Writable for MPCBB2_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPCBB2_CR to value 0"]
impl crate::Resettable for MPCBB2_CRrs {
    const RESET_VALUE: u32 = 0;
}
