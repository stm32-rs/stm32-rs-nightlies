#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `DEN` reader - Operational amplifier Enable"]
pub type DEN_R = crate::BitReader;
#[doc = "Field `DEN` writer - Operational amplifier Enable"]
pub type DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEN` reader - OPALPM"]
pub type SEN_R = crate::BitReader;
#[doc = "Field `SEN` writer - OPALPM"]
pub type SEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OPALPM"]
    #[inline(always)]
    pub fn sen(&self) -> SEN_R {
        SEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    #[must_use]
    pub fn den(&mut self) -> DEN_W<CRrs> {
        DEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - OPALPM"]
    #[inline(always)]
    #[must_use]
    pub fn sen(&mut self) -> SEN_W<CRrs> {
        SEN_W::new(self, 1)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
