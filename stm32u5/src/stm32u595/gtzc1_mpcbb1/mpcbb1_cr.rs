#[doc = "Register `MPCBB1_CR` reader"]
pub type R = crate::R<MPCBB1_CRrs>;
#[doc = "Register `MPCBB1_CR` writer"]
pub type W = crate::W<MPCBB1_CRrs>;
#[doc = "Field `GLOCK` reader - lock the control register of the MPCBB until next reset"]
pub type GLOCK_R = crate::BitReader;
#[doc = "Field `GLOCK` writer - lock the control register of the MPCBB until next reset"]
pub type GLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVSECSTATE` reader - SRAMx clocks security state"]
pub type INVSECSTATE_R = crate::BitReader;
#[doc = "Field `INVSECSTATE` writer - SRAMx clocks security state"]
pub type INVSECSTATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRWILADIS` reader - secure read/write illegal access disable"]
pub type SRWILADIS_R = crate::BitReader;
#[doc = "Field `SRWILADIS` writer - secure read/write illegal access disable"]
pub type SRWILADIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock the control register of the MPCBB until next reset"]
    #[inline(always)]
    pub fn glock(&self) -> GLOCK_R {
        GLOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 30 - SRAMx clocks security state"]
    #[inline(always)]
    pub fn invsecstate(&self) -> INVSECSTATE_R {
        INVSECSTATE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - secure read/write illegal access disable"]
    #[inline(always)]
    pub fn srwiladis(&self) -> SRWILADIS_R {
        SRWILADIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock the control register of the MPCBB until next reset"]
    #[inline(always)]
    #[must_use]
    pub fn glock(&mut self) -> GLOCK_W<MPCBB1_CRrs> {
        GLOCK_W::new(self, 0)
    }
    #[doc = "Bit 30 - SRAMx clocks security state"]
    #[inline(always)]
    #[must_use]
    pub fn invsecstate(&mut self) -> INVSECSTATE_W<MPCBB1_CRrs> {
        INVSECSTATE_W::new(self, 30)
    }
    #[doc = "Bit 31 - secure read/write illegal access disable"]
    #[inline(always)]
    #[must_use]
    pub fn srwiladis(&mut self) -> SRWILADIS_W<MPCBB1_CRrs> {
        SRWILADIS_W::new(self, 31)
    }
}
#[doc = "MPCBB control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPCBB1_CRrs;
impl crate::RegisterSpec for MPCBB1_CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpcbb1_cr::R`](R) reader structure"]
impl crate::Readable for MPCBB1_CRrs {}
#[doc = "`write(|w| ..)` method takes [`mpcbb1_cr::W`](W) writer structure"]
impl crate::Writable for MPCBB1_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPCBB1_CR to value 0"]
impl crate::Resettable for MPCBB1_CRrs {
    const RESET_VALUE: u32 = 0;
}
