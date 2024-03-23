#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSRrs>;
#[doc = "Field `ENABLE` reader - Counter enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Counter enable"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TICKINT` reader - SysTick exception request enable"]
pub type TICKINT_R = crate::BitReader;
#[doc = "Field `TICKINT` writer - SysTick exception request enable"]
pub type TICKINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSOURCE` reader - Clock source selection"]
pub type CLKSOURCE_R = crate::BitReader;
#[doc = "Field `CLKSOURCE` writer - Clock source selection"]
pub type CLKSOURCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNTFLAG` reader - COUNTFLAG"]
pub type COUNTFLAG_R = crate::BitReader;
#[doc = "Field `COUNTFLAG` writer - COUNTFLAG"]
pub type COUNTFLAG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SysTick exception request enable"]
    #[inline(always)]
    pub fn tickint(&self) -> TICKINT_R {
        TICKINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock source selection"]
    #[inline(always)]
    pub fn clksource(&self) -> CLKSOURCE_R {
        CLKSOURCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - COUNTFLAG"]
    #[inline(always)]
    pub fn countflag(&self) -> COUNTFLAG_R {
        COUNTFLAG_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CSRrs> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - SysTick exception request enable"]
    #[inline(always)]
    #[must_use]
    pub fn tickint(&mut self) -> TICKINT_W<CSRrs> {
        TICKINT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn clksource(&mut self) -> CLKSOURCE_W<CSRrs> {
        CLKSOURCE_W::new(self, 2)
    }
    #[doc = "Bit 16 - COUNTFLAG"]
    #[inline(always)]
    #[must_use]
    pub fn countflag(&mut self) -> COUNTFLAG_W<CSRrs> {
        COUNTFLAG_W::new(self, 16)
    }
}
#[doc = "SysTick control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSRrs {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0;
}
