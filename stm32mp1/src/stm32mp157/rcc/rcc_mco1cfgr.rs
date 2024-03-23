#[doc = "Register `RCC_MCO1CFGR` reader"]
pub type R = crate::R<RCC_MCO1CFGRrs>;
#[doc = "Register `RCC_MCO1CFGR` writer"]
pub type W = crate::W<RCC_MCO1CFGRrs>;
#[doc = "Field `MCO1SEL` reader - MCO1SEL"]
pub type MCO1SEL_R = crate::FieldReader;
#[doc = "Field `MCO1SEL` writer - MCO1SEL"]
pub type MCO1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MCO1DIV` reader - MCO1DIV"]
pub type MCO1DIV_R = crate::FieldReader;
#[doc = "Field `MCO1DIV` writer - MCO1DIV"]
pub type MCO1DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCO1ON` reader - MCO1ON"]
pub type MCO1ON_R = crate::BitReader;
#[doc = "Field `MCO1ON` writer - MCO1ON"]
pub type MCO1ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - MCO1SEL"]
    #[inline(always)]
    pub fn mco1sel(&self) -> MCO1SEL_R {
        MCO1SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - MCO1DIV"]
    #[inline(always)]
    pub fn mco1div(&self) -> MCO1DIV_R {
        MCO1DIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - MCO1ON"]
    #[inline(always)]
    pub fn mco1on(&self) -> MCO1ON_R {
        MCO1ON_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCO1SEL"]
    #[inline(always)]
    #[must_use]
    pub fn mco1sel(&mut self) -> MCO1SEL_W<RCC_MCO1CFGRrs> {
        MCO1SEL_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - MCO1DIV"]
    #[inline(always)]
    #[must_use]
    pub fn mco1div(&mut self) -> MCO1DIV_W<RCC_MCO1CFGRrs> {
        MCO1DIV_W::new(self, 4)
    }
    #[doc = "Bit 12 - MCO1ON"]
    #[inline(always)]
    #[must_use]
    pub fn mco1on(&mut self) -> MCO1ON_W<RCC_MCO1CFGRrs> {
        MCO1ON_W::new(self, 12)
    }
}
#[doc = "This register is used to select the clock generated on MCO1 output.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mco1cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mco1cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MCO1CFGRrs;
impl crate::RegisterSpec for RCC_MCO1CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mco1cfgr::R`](R) reader structure"]
impl crate::Readable for RCC_MCO1CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mco1cfgr::W`](W) writer structure"]
impl crate::Writable for RCC_MCO1CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MCO1CFGR to value 0"]
impl crate::Resettable for RCC_MCO1CFGRrs {
    const RESET_VALUE: u32 = 0;
}
