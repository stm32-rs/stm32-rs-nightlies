#[doc = "Register `SRDCFGR` reader"]
pub type R = crate::R<SRDCFGRrs>;
#[doc = "Register `SRDCFGR` writer"]
pub type W = crate::W<SRDCFGRrs>;
#[doc = "Field `SRDPPRE` reader - SmartRun domain APB4 prescaler Set and reset by software to control the SmartRun domain APB4 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk4 after SRDPPRE write. 0xx: rcc_pclk4 = rcc_hclk4 (default after reset)"]
pub type SRDPPRE_R = crate::FieldReader;
#[doc = "Field `SRDPPRE` writer - SmartRun domain APB4 prescaler Set and reset by software to control the SmartRun domain APB4 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk4 after SRDPPRE write. 0xx: rcc_pclk4 = rcc_hclk4 (default after reset)"]
pub type SRDPPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 4:6 - SmartRun domain APB4 prescaler Set and reset by software to control the SmartRun domain APB4 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk4 after SRDPPRE write. 0xx: rcc_pclk4 = rcc_hclk4 (default after reset)"]
    #[inline(always)]
    pub fn srdppre(&self) -> SRDPPRE_R {
        SRDPPRE_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - SmartRun domain APB4 prescaler Set and reset by software to control the SmartRun domain APB4 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk4 after SRDPPRE write. 0xx: rcc_pclk4 = rcc_hclk4 (default after reset)"]
    #[inline(always)]
    #[must_use]
    pub fn srdppre(&mut self) -> SRDPPRE_W<SRDCFGRrs> {
        SRDPPRE_W::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srdcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srdcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRDCFGRrs;
impl crate::RegisterSpec for SRDCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srdcfgr::R`](R) reader structure"]
impl crate::Readable for SRDCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`srdcfgr::W`](W) writer structure"]
impl crate::Writable for SRDCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRDCFGR to value 0"]
impl crate::Resettable for SRDCFGRrs {
    const RESET_VALUE: u32 = 0;
}
