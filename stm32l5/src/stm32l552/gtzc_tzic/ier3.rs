#[doc = "Register `IER3` reader"]
pub type R = crate::R<IER3rs>;
#[doc = "Register `IER3` writer"]
pub type W = crate::W<IER3rs>;
#[doc = "Field `TZSCIE` reader - TZSCIE"]
pub type TZSCIE_R = crate::BitReader;
#[doc = "Field `TZSCIE` writer - TZSCIE"]
pub type TZSCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZICIE` reader - TZICIE"]
pub type TZICIE_R = crate::BitReader;
#[doc = "Field `TZICIE` writer - TZICIE"]
pub type TZICIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCWM1IE` reader - MPCWM1IE"]
pub type MPCWM1IE_R = crate::BitReader;
#[doc = "Field `MPCWM1IE` writer - MPCWM1IE"]
pub type MPCWM1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCWM2IE` reader - MPCWM2IE"]
pub type MPCWM2IE_R = crate::BitReader;
#[doc = "Field `MPCWM2IE` writer - MPCWM2IE"]
pub type MPCWM2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCBB1IE` reader - MPCBB1IE"]
pub type MPCBB1IE_R = crate::BitReader;
#[doc = "Field `MPCBB1IE` writer - MPCBB1IE"]
pub type MPCBB1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCBB1_REGIE` reader - MPCBB1_REGIE"]
pub type MPCBB1_REGIE_R = crate::BitReader;
#[doc = "Field `MPCBB1_REGIE` writer - MPCBB1_REGIE"]
pub type MPCBB1_REGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCBB2IE` reader - MPCBB2IE"]
pub type MPCBB2IE_R = crate::BitReader;
#[doc = "Field `MPCBB2IE` writer - MPCBB2IE"]
pub type MPCBB2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCBB2_REGIE` reader - MPCBB2_REGIE"]
pub type MPCBB2_REGIE_R = crate::BitReader;
#[doc = "Field `MPCBB2_REGIE` writer - MPCBB2_REGIE"]
pub type MPCBB2_REGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TZSCIE"]
    #[inline(always)]
    pub fn tzscie(&self) -> TZSCIE_R {
        TZSCIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TZICIE"]
    #[inline(always)]
    pub fn tzicie(&self) -> TZICIE_R {
        TZICIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MPCWM1IE"]
    #[inline(always)]
    pub fn mpcwm1ie(&self) -> MPCWM1IE_R {
        MPCWM1IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MPCWM2IE"]
    #[inline(always)]
    pub fn mpcwm2ie(&self) -> MPCWM2IE_R {
        MPCWM2IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MPCBB1IE"]
    #[inline(always)]
    pub fn mpcbb1ie(&self) -> MPCBB1IE_R {
        MPCBB1IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MPCBB1_REGIE"]
    #[inline(always)]
    pub fn mpcbb1_regie(&self) -> MPCBB1_REGIE_R {
        MPCBB1_REGIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MPCBB2IE"]
    #[inline(always)]
    pub fn mpcbb2ie(&self) -> MPCBB2IE_R {
        MPCBB2IE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MPCBB2_REGIE"]
    #[inline(always)]
    pub fn mpcbb2_regie(&self) -> MPCBB2_REGIE_R {
        MPCBB2_REGIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZSCIE"]
    #[inline(always)]
    #[must_use]
    pub fn tzscie(&mut self) -> TZSCIE_W<IER3rs> {
        TZSCIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - TZICIE"]
    #[inline(always)]
    #[must_use]
    pub fn tzicie(&mut self) -> TZICIE_W<IER3rs> {
        TZICIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - MPCWM1IE"]
    #[inline(always)]
    #[must_use]
    pub fn mpcwm1ie(&mut self) -> MPCWM1IE_W<IER3rs> {
        MPCWM1IE_W::new(self, 2)
    }
    #[doc = "Bit 3 - MPCWM2IE"]
    #[inline(always)]
    #[must_use]
    pub fn mpcwm2ie(&mut self) -> MPCWM2IE_W<IER3rs> {
        MPCWM2IE_W::new(self, 3)
    }
    #[doc = "Bit 4 - MPCBB1IE"]
    #[inline(always)]
    #[must_use]
    pub fn mpcbb1ie(&mut self) -> MPCBB1IE_W<IER3rs> {
        MPCBB1IE_W::new(self, 4)
    }
    #[doc = "Bit 5 - MPCBB1_REGIE"]
    #[inline(always)]
    #[must_use]
    pub fn mpcbb1_regie(&mut self) -> MPCBB1_REGIE_W<IER3rs> {
        MPCBB1_REGIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - MPCBB2IE"]
    #[inline(always)]
    #[must_use]
    pub fn mpcbb2ie(&mut self) -> MPCBB2IE_W<IER3rs> {
        MPCBB2IE_W::new(self, 6)
    }
    #[doc = "Bit 7 - MPCBB2_REGIE"]
    #[inline(always)]
    #[must_use]
    pub fn mpcbb2_regie(&mut self) -> MPCBB2_REGIE_W<IER3rs> {
        MPCBB2_REGIE_W::new(self, 7)
    }
}
#[doc = "TZIC interrupt enable register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER3rs;
impl crate::RegisterSpec for IER3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier3::R`](R) reader structure"]
impl crate::Readable for IER3rs {}
#[doc = "`write(|w| ..)` method takes [`ier3::W`](W) writer structure"]
impl crate::Writable for IER3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER3 to value 0"]
impl crate::Resettable for IER3rs {
    const RESET_VALUE: u32 = 0;
}
