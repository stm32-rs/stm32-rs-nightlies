#[doc = "Register `M2IER` reader"]
pub type R = crate::R<M2IERrs>;
#[doc = "Register `M2IER` writer"]
pub type W = crate::W<M2IERrs>;
#[doc = "Field `SEIE` reader - SEIE"]
pub type SEIE_R = crate::BitReader;
#[doc = "Field `SEIE` writer - SEIE"]
pub type SEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEIE` reader - DEIE"]
pub type DEIE_R = crate::BitReader;
#[doc = "Field `DEIE` writer - DEIE"]
pub type DEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCNMI` reader - ECCNMI"]
pub type ECCNMI_R = crate::BitReader;
#[doc = "Field `ECCNMI` writer - ECCNMI"]
pub type ECCNMI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SEIE"]
    #[inline(always)]
    pub fn seie(&self) -> SEIE_R {
        SEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DEIE"]
    #[inline(always)]
    pub fn deie(&self) -> DEIE_R {
        DEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - ECCNMI"]
    #[inline(always)]
    pub fn eccnmi(&self) -> ECCNMI_R {
        ECCNMI_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SEIE"]
    #[inline(always)]
    #[must_use]
    pub fn seie(&mut self) -> SEIE_W<M2IERrs> {
        SEIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - DEIE"]
    #[inline(always)]
    #[must_use]
    pub fn deie(&mut self) -> DEIE_W<M2IERrs> {
        DEIE_W::new(self, 1)
    }
    #[doc = "Bit 3 - ECCNMI"]
    #[inline(always)]
    #[must_use]
    pub fn eccnmi(&mut self) -> ECCNMI_W<M2IERrs> {
        ECCNMI_W::new(self, 3)
    }
}
#[doc = "RAMCFG SRAM x interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M2IERrs;
impl crate::RegisterSpec for M2IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m2ier::R`](R) reader structure"]
impl crate::Readable for M2IERrs {}
#[doc = "`write(|w| ..)` method takes [`m2ier::W`](W) writer structure"]
impl crate::Writable for M2IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M2IER to value 0"]
impl crate::Resettable for M2IERrs {
    const RESET_VALUE: u32 = 0;
}
