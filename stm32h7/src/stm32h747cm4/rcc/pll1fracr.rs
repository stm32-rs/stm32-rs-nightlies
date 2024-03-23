#[doc = "Register `PLL1FRACR` reader"]
pub type R = crate::R<PLL1FRACRrs>;
#[doc = "Register `PLL1FRACR` writer"]
pub type W = crate::W<PLL1FRACRrs>;
#[doc = "Field `FRACN1` reader - Fractional part of the multiplication factor for PLL1 VCO"]
pub type FRACN1_R = crate::FieldReader<u16>;
#[doc = "Field `FRACN1` writer - Fractional part of the multiplication factor for PLL1 VCO"]
pub type FRACN1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn fracn1(&self) -> FRACN1_R {
        FRACN1_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL1 VCO"]
    #[inline(always)]
    #[must_use]
    pub fn fracn1(&mut self) -> FRACN1_W<PLL1FRACRrs> {
        FRACN1_W::new(self, 3)
    }
}
#[doc = "RCC PLL1 Fractional Divider Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll1fracr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll1fracr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL1FRACRrs;
impl crate::RegisterSpec for PLL1FRACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll1fracr::R`](R) reader structure"]
impl crate::Readable for PLL1FRACRrs {}
#[doc = "`write(|w| ..)` method takes [`pll1fracr::W`](W) writer structure"]
impl crate::Writable for PLL1FRACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL1FRACR to value 0"]
impl crate::Resettable for PLL1FRACRrs {
    const RESET_VALUE: u32 = 0;
}
