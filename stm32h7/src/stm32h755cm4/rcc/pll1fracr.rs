///Register `PLL1FRACR` reader
pub type R = crate::R<PLL1FRACRrs>;
///Register `PLL1FRACR` writer
pub type W = crate::W<PLL1FRACRrs>;
///Field `FRACN1` reader - Fractional part of the multiplication factor for PLL1 VCO
pub type FRACN1_R = crate::FieldReader<u16>;
///Field `FRACN1` writer - Fractional part of the multiplication factor for PLL1 VCO
pub type FRACN1_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16, crate::Safe>;
impl R {
    ///Bits 3:15 - Fractional part of the multiplication factor for PLL1 VCO
    #[inline(always)]
    pub fn fracn1(&self) -> FRACN1_R {
        FRACN1_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1FRACR")
            .field("fracn1", &self.fracn1())
            .finish()
    }
}
impl W {
    ///Bits 3:15 - Fractional part of the multiplication factor for PLL1 VCO
    #[inline(always)]
    pub fn fracn1(&mut self) -> FRACN1_W<'_, PLL1FRACRrs> {
        FRACN1_W::new(self, 3)
    }
}
/**RCC PLL1 Fractional Divider Register

You can [`read`](crate::Reg::read) this register and get [`pll1fracr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1fracr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#RCC:PLL1FRACR)*/
pub struct PLL1FRACRrs;
impl crate::RegisterSpec for PLL1FRACRrs {
    type Ux = u32;
}
///`read()` method returns [`pll1fracr::R`](R) reader structure
impl crate::Readable for PLL1FRACRrs {}
///`write(|w| ..)` method takes [`pll1fracr::W`](W) writer structure
impl crate::Writable for PLL1FRACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL1FRACR to value 0
impl crate::Resettable for PLL1FRACRrs {}
