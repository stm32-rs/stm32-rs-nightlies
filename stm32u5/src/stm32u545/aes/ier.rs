#[doc = "Register `IER` reader"]
pub type R = crate::R<IERrs>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IERrs>;
#[doc = "Field `CCFIE` reader - Computation complete flag"]
pub type CCFIE_R = crate::BitReader;
#[doc = "Field `CCFIE` writer - Computation complete flag"]
pub type CCFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWEIE` reader - Read or write error interrupt flag"]
pub type RWEIE_R = crate::BitReader;
#[doc = "Field `RWEIE` writer - Read or write error interrupt flag"]
pub type RWEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEIE` reader - Key error interrupt flag"]
pub type KEIE_R = crate::BitReader;
#[doc = "Field `KEIE` writer - Key error interrupt flag"]
pub type KEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Computation complete flag"]
    #[inline(always)]
    pub fn ccfie(&self) -> CCFIE_R {
        CCFIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read or write error interrupt flag"]
    #[inline(always)]
    pub fn rweie(&self) -> RWEIE_R {
        RWEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Key error interrupt flag"]
    #[inline(always)]
    pub fn keie(&self) -> KEIE_R {
        KEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Computation complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn ccfie(&mut self) -> CCFIE_W<IERrs> {
        CCFIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Read or write error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn rweie(&mut self) -> RWEIE_W<IERrs> {
        RWEIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Key error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn keie(&mut self) -> KEIE_W<IERrs> {
        KEIE_W::new(self, 2)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
