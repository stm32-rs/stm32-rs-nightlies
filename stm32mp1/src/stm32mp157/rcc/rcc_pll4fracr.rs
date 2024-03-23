#[doc = "Register `RCC_PLL4FRACR` reader"]
pub type R = crate::R<RCC_PLL4FRACRrs>;
#[doc = "Register `RCC_PLL4FRACR` writer"]
pub type W = crate::W<RCC_PLL4FRACRrs>;
#[doc = "Field `FRACV` reader - FRACV"]
pub type FRACV_R = crate::FieldReader<u16>;
#[doc = "Field `FRACV` writer - FRACV"]
pub type FRACV_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `FRACLE` reader - FRACLE"]
pub type FRACLE_R = crate::BitReader;
#[doc = "Field `FRACLE` writer - FRACLE"]
pub type FRACLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 3:15 - FRACV"]
    #[inline(always)]
    pub fn fracv(&self) -> FRACV_R {
        FRACV_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    #[doc = "Bit 16 - FRACLE"]
    #[inline(always)]
    pub fn fracle(&self) -> FRACLE_R {
        FRACLE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 3:15 - FRACV"]
    #[inline(always)]
    #[must_use]
    pub fn fracv(&mut self) -> FRACV_W<RCC_PLL4FRACRrs> {
        FRACV_W::new(self, 3)
    }
    #[doc = "Bit 16 - FRACLE"]
    #[inline(always)]
    #[must_use]
    pub fn fracle(&mut self) -> FRACLE_W<RCC_PLL4FRACRrs> {
        FRACLE_W::new(self, 16)
    }
}
#[doc = "This register is used to fine-tune the frequency of the PLL4 VCO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll4fracr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll4fracr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_PLL4FRACRrs;
impl crate::RegisterSpec for RCC_PLL4FRACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_pll4fracr::R`](R) reader structure"]
impl crate::Readable for RCC_PLL4FRACRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_pll4fracr::W`](W) writer structure"]
impl crate::Writable for RCC_PLL4FRACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_PLL4FRACR to value 0"]
impl crate::Resettable for RCC_PLL4FRACRrs {
    const RESET_VALUE: u32 = 0;
}
