#[doc = "Register `IER` reader"]
pub type R = crate::R<IERrs>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IERrs>;
#[doc = "Field `GIE` reader - Global interrupt enable"]
pub type GIE_R = crate::BitReader;
#[doc = "Field `GIE` writer - Global interrupt enable"]
pub type GIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GECCSEIE_` reader - Global ECC single error interrupt enable"]
pub type GECCSEIE__R = crate::BitReader;
#[doc = "Field `GECCSEIE_` writer - Global ECC single error interrupt enable"]
pub type GECCSEIE__W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GECCDEIE` reader - Global ECC double error interrupt enable"]
pub type GECCDEIE_R = crate::BitReader;
#[doc = "Field `GECCDEIE` writer - Global ECC double error interrupt enable"]
pub type GECCDEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GECCDEBWIE` reader - Global ECC double error on byte write (BW) interrupt enable"]
pub type GECCDEBWIE_R = crate::BitReader;
#[doc = "Field `GECCDEBWIE` writer - Global ECC double error on byte write (BW) interrupt enable"]
pub type GECCDEBWIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Global interrupt enable"]
    #[inline(always)]
    pub fn gie(&self) -> GIE_R {
        GIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Global ECC single error interrupt enable"]
    #[inline(always)]
    pub fn geccseie_(&self) -> GECCSEIE__R {
        GECCSEIE__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global ECC double error interrupt enable"]
    #[inline(always)]
    pub fn geccdeie(&self) -> GECCDEIE_R {
        GECCDEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global ECC double error on byte write (BW) interrupt enable"]
    #[inline(always)]
    pub fn geccdebwie(&self) -> GECCDEBWIE_R {
        GECCDEBWIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gie(&mut self) -> GIE_W<IERrs> {
        GIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Global ECC single error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn geccseie_(&mut self) -> GECCSEIE__W<IERrs> {
        GECCSEIE__W::new(self, 1)
    }
    #[doc = "Bit 2 - Global ECC double error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn geccdeie(&mut self) -> GECCDEIE_W<IERrs> {
        GECCDEIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Global ECC double error on byte write (BW) interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn geccdebwie(&mut self) -> GECCDEBWIE_W<IERrs> {
        GECCDEBWIE_W::new(self, 3)
    }
}
#[doc = "RAMECC interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IERrs {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}
