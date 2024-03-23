#[doc = "Register `RCC_MCO2CFGR` reader"]
pub type R = crate::R<RCC_MCO2CFGRrs>;
#[doc = "Register `RCC_MCO2CFGR` writer"]
pub type W = crate::W<RCC_MCO2CFGRrs>;
#[doc = "Field `MCO2SEL` reader - MCO2SEL"]
pub type MCO2SEL_R = crate::FieldReader;
#[doc = "Field `MCO2SEL` writer - MCO2SEL"]
pub type MCO2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MCO2DIV` reader - MCO2DIV"]
pub type MCO2DIV_R = crate::FieldReader;
#[doc = "Field `MCO2DIV` writer - MCO2DIV"]
pub type MCO2DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCO2ON` reader - MCO2ON"]
pub type MCO2ON_R = crate::BitReader;
#[doc = "Field `MCO2ON` writer - MCO2ON"]
pub type MCO2ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - MCO2SEL"]
    #[inline(always)]
    pub fn mco2sel(&self) -> MCO2SEL_R {
        MCO2SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - MCO2DIV"]
    #[inline(always)]
    pub fn mco2div(&self) -> MCO2DIV_R {
        MCO2DIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - MCO2ON"]
    #[inline(always)]
    pub fn mco2on(&self) -> MCO2ON_R {
        MCO2ON_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCO2SEL"]
    #[inline(always)]
    #[must_use]
    pub fn mco2sel(&mut self) -> MCO2SEL_W<RCC_MCO2CFGRrs> {
        MCO2SEL_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - MCO2DIV"]
    #[inline(always)]
    #[must_use]
    pub fn mco2div(&mut self) -> MCO2DIV_W<RCC_MCO2CFGRrs> {
        MCO2DIV_W::new(self, 4)
    }
    #[doc = "Bit 12 - MCO2ON"]
    #[inline(always)]
    #[must_use]
    pub fn mco2on(&mut self) -> MCO2ON_W<RCC_MCO2CFGRrs> {
        MCO2ON_W::new(self, 12)
    }
}
#[doc = "This register is used to select the clock generated on MCO2 output.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mco2cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mco2cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MCO2CFGRrs;
impl crate::RegisterSpec for RCC_MCO2CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mco2cfgr::R`](R) reader structure"]
impl crate::Readable for RCC_MCO2CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mco2cfgr::W`](W) writer structure"]
impl crate::Writable for RCC_MCO2CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MCO2CFGR to value 0"]
impl crate::Resettable for RCC_MCO2CFGRrs {
    const RESET_VALUE: u32 = 0;
}
