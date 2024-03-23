#[doc = "Register `IER` reader"]
pub type R = crate::R<IERrs>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IERrs>;
#[doc = "Field `TAMP1IE` reader - TAMP1IE"]
pub type TAMP1IE_R = crate::BitReader;
#[doc = "Field `TAMP1IE` writer - TAMP1IE"]
pub type TAMP1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2IE` reader - TAMP2IE"]
pub type TAMP2IE_R = crate::BitReader;
#[doc = "Field `TAMP2IE` writer - TAMP2IE"]
pub type TAMP2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP1IE` reader - ITAMP1IE"]
pub type ITAMP1IE_R = crate::BitReader;
#[doc = "Field `ITAMP1IE` writer - ITAMP1IE"]
pub type ITAMP1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP3IE` reader - ITAMP3IE"]
pub type ITAMP3IE_R = crate::BitReader;
#[doc = "Field `ITAMP3IE` writer - ITAMP3IE"]
pub type ITAMP3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP4IE` reader - ITAMP4IE"]
pub type ITAMP4IE_R = crate::BitReader;
#[doc = "Field `ITAMP4IE` writer - ITAMP4IE"]
pub type ITAMP4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP5IE` reader - ITAMP5IE"]
pub type ITAMP5IE_R = crate::BitReader;
#[doc = "Field `ITAMP5IE` writer - ITAMP5IE"]
pub type ITAMP5IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP6IE` reader - ITAMP6IE"]
pub type ITAMP6IE_R = crate::BitReader;
#[doc = "Field `ITAMP6IE` writer - ITAMP6IE"]
pub type ITAMP6IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TAMP1IE"]
    #[inline(always)]
    pub fn tamp1ie(&self) -> TAMP1IE_R {
        TAMP1IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2IE"]
    #[inline(always)]
    pub fn tamp2ie(&self) -> TAMP2IE_R {
        TAMP2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - ITAMP1IE"]
    #[inline(always)]
    pub fn itamp1ie(&self) -> ITAMP1IE_R {
        ITAMP1IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - ITAMP3IE"]
    #[inline(always)]
    pub fn itamp3ie(&self) -> ITAMP3IE_R {
        ITAMP3IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ITAMP4IE"]
    #[inline(always)]
    pub fn itamp4ie(&self) -> ITAMP4IE_R {
        ITAMP4IE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ITAMP5IE"]
    #[inline(always)]
    pub fn itamp5ie(&self) -> ITAMP5IE_R {
        ITAMP5IE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ITAMP6IE"]
    #[inline(always)]
    pub fn itamp6ie(&self) -> ITAMP6IE_R {
        ITAMP6IE_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMP1IE"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1ie(&mut self) -> TAMP1IE_W<IERrs> {
        TAMP1IE_W::new(self, 0)
    }
    #[doc = "Bit 1 - TAMP2IE"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2ie(&mut self) -> TAMP2IE_W<IERrs> {
        TAMP2IE_W::new(self, 1)
    }
    #[doc = "Bit 16 - ITAMP1IE"]
    #[inline(always)]
    #[must_use]
    pub fn itamp1ie(&mut self) -> ITAMP1IE_W<IERrs> {
        ITAMP1IE_W::new(self, 16)
    }
    #[doc = "Bit 18 - ITAMP3IE"]
    #[inline(always)]
    #[must_use]
    pub fn itamp3ie(&mut self) -> ITAMP3IE_W<IERrs> {
        ITAMP3IE_W::new(self, 18)
    }
    #[doc = "Bit 19 - ITAMP4IE"]
    #[inline(always)]
    #[must_use]
    pub fn itamp4ie(&mut self) -> ITAMP4IE_W<IERrs> {
        ITAMP4IE_W::new(self, 19)
    }
    #[doc = "Bit 20 - ITAMP5IE"]
    #[inline(always)]
    #[must_use]
    pub fn itamp5ie(&mut self) -> ITAMP5IE_W<IERrs> {
        ITAMP5IE_W::new(self, 20)
    }
    #[doc = "Bit 21 - ITAMP6IE"]
    #[inline(always)]
    #[must_use]
    pub fn itamp6ie(&mut self) -> ITAMP6IE_W<IERrs> {
        ITAMP6IE_W::new(self, 21)
    }
}
#[doc = "TAMP interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
