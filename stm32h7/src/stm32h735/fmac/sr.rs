#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Field `YEMPTY` reader - Y buffer empty flag"]
pub type YEMPTY_R = crate::BitReader;
#[doc = "Field `YEMPTY` writer - Y buffer empty flag"]
pub type YEMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X1FULL` reader - X1 buffer full flag"]
pub type X1FULL_R = crate::BitReader;
#[doc = "Field `X1FULL` writer - X1 buffer full flag"]
pub type X1FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFL` reader - Overflow error flag"]
pub type OVFL_R = crate::BitReader;
#[doc = "Field `OVFL` writer - Overflow error flag"]
pub type OVFL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNFL` reader - Underflow error flag"]
pub type UNFL_R = crate::BitReader;
#[doc = "Field `UNFL` writer - Underflow error flag"]
pub type UNFL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAT` reader - Saturation error flag"]
pub type SAT_R = crate::BitReader;
#[doc = "Field `SAT` writer - Saturation error flag"]
pub type SAT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Y buffer empty flag"]
    #[inline(always)]
    pub fn yempty(&self) -> YEMPTY_R {
        YEMPTY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - X1 buffer full flag"]
    #[inline(always)]
    pub fn x1full(&self) -> X1FULL_R {
        X1FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Overflow error flag"]
    #[inline(always)]
    pub fn ovfl(&self) -> OVFL_R {
        OVFL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Underflow error flag"]
    #[inline(always)]
    pub fn unfl(&self) -> UNFL_R {
        UNFL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Saturation error flag"]
    #[inline(always)]
    pub fn sat(&self) -> SAT_R {
        SAT_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Y buffer empty flag"]
    #[inline(always)]
    #[must_use]
    pub fn yempty(&mut self) -> YEMPTY_W<SRrs> {
        YEMPTY_W::new(self, 0)
    }
    #[doc = "Bit 1 - X1 buffer full flag"]
    #[inline(always)]
    #[must_use]
    pub fn x1full(&mut self) -> X1FULL_W<SRrs> {
        X1FULL_W::new(self, 1)
    }
    #[doc = "Bit 8 - Overflow error flag"]
    #[inline(always)]
    #[must_use]
    pub fn ovfl(&mut self) -> OVFL_W<SRrs> {
        OVFL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Underflow error flag"]
    #[inline(always)]
    #[must_use]
    pub fn unfl(&mut self) -> UNFL_W<SRrs> {
        UNFL_W::new(self, 9)
    }
    #[doc = "Bit 10 - Saturation error flag"]
    #[inline(always)]
    #[must_use]
    pub fn sat(&mut self) -> SAT_W<SRrs> {
        SAT_W::new(self, 10)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0x01"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x01;
}
