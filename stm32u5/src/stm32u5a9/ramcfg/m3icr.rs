#[doc = "Register `M3ICR` reader"]
pub type R = crate::R<M3ICRrs>;
#[doc = "Register `M3ICR` writer"]
pub type W = crate::W<M3ICRrs>;
#[doc = "Field `CSEDC` reader - CSEDC"]
pub type CSEDC_R = crate::BitReader;
#[doc = "Field `CSEDC` writer - CSEDC"]
pub type CSEDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDED` reader - CDED"]
pub type CDED_R = crate::BitReader;
#[doc = "Field `CDED` writer - CDED"]
pub type CDED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CSEDC"]
    #[inline(always)]
    pub fn csedc(&self) -> CSEDC_R {
        CSEDC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CDED"]
    #[inline(always)]
    pub fn cded(&self) -> CDED_R {
        CDED_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CSEDC"]
    #[inline(always)]
    #[must_use]
    pub fn csedc(&mut self) -> CSEDC_W<M3ICRrs> {
        CSEDC_W::new(self, 0)
    }
    #[doc = "Bit 1 - CDED"]
    #[inline(always)]
    #[must_use]
    pub fn cded(&mut self) -> CDED_W<M3ICRrs> {
        CDED_W::new(self, 1)
    }
}
#[doc = "RAMCFG RAM x interrupt clear register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m3icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M3ICRrs;
impl crate::RegisterSpec for M3ICRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m3icr::R`](R) reader structure"]
impl crate::Readable for M3ICRrs {}
#[doc = "`write(|w| ..)` method takes [`m3icr::W`](W) writer structure"]
impl crate::Writable for M3ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M3ICR to value 0"]
impl crate::Resettable for M3ICRrs {
    const RESET_VALUE: u32 = 0;
}
