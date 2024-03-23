#[doc = "Register `IER` reader"]
pub type R = crate::R<IERrs>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IERrs>;
#[doc = "Field `LIE` reader - Line Interrupt Enable"]
pub type LIE_R = crate::BitReader;
#[doc = "Field `LIE` writer - Line Interrupt Enable"]
pub type LIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUIE` reader - FIFO Underrun Interrupt Enable"]
pub type FUIE_R = crate::BitReader;
#[doc = "Field `FUIE` writer - FIFO Underrun Interrupt Enable"]
pub type FUIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERRIE` reader - Transfer Error Interrupt Enable"]
pub type TERRIE_R = crate::BitReader;
#[doc = "Field `TERRIE` writer - Transfer Error Interrupt Enable"]
pub type TERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRIE` reader - Register Reload interrupt enable"]
pub type RRIE_R = crate::BitReader;
#[doc = "Field `RRIE` writer - Register Reload interrupt enable"]
pub type RRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Line Interrupt Enable"]
    #[inline(always)]
    pub fn lie(&self) -> LIE_R {
        LIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn fuie(&self) -> FUIE_R {
        FUIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer Error Interrupt Enable"]
    #[inline(always)]
    pub fn terrie(&self) -> TERRIE_R {
        TERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Register Reload interrupt enable"]
    #[inline(always)]
    pub fn rrie(&self) -> RRIE_R {
        RRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Line Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lie(&mut self) -> LIE_W<IERrs> {
        LIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO Underrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fuie(&mut self) -> FUIE_W<IERrs> {
        FUIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transfer Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn terrie(&mut self) -> TERRIE_W<IERrs> {
        TERRIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Register Reload interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rrie(&mut self) -> RRIE_W<IERrs> {
        RRIE_W::new(self, 3)
    }
}
#[doc = "LTDC Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
