///Register `PLL2FRACR` reader
pub type R = crate::R<PLL2FRACRrs>;
///Register `PLL2FRACR` writer
pub type W = crate::W<PLL2FRACRrs>;
///Field `PLL2FRACN` reader - fractional part of the multiplication factor for PLL2 VCO
pub type PLL2FRACN_R = crate::FieldReader<u16>;
///Field `PLL2FRACN` writer - fractional part of the multiplication factor for PLL2 VCO
pub type PLL2FRACN_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16, crate::Safe>;
impl R {
    ///Bits 3:15 - fractional part of the multiplication factor for PLL2 VCO
    #[inline(always)]
    pub fn pll2fracn(&self) -> PLL2FRACN_R {
        PLL2FRACN_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL2FRACR")
            .field("pll2fracn", &self.pll2fracn())
            .finish()
    }
}
impl W {
    ///Bits 3:15 - fractional part of the multiplication factor for PLL2 VCO
    #[inline(always)]
    pub fn pll2fracn(&mut self) -> PLL2FRACN_W<'_, PLL2FRACRrs> {
        PLL2FRACN_W::new(self, 3)
    }
}
/**RCC PLL2 fractional divider register

You can [`read`](crate::Reg::read) this register and get [`pll2fracr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2fracr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#RCC:PLL2FRACR)*/
pub struct PLL2FRACRrs;
impl crate::RegisterSpec for PLL2FRACRrs {
    type Ux = u32;
}
///`read()` method returns [`pll2fracr::R`](R) reader structure
impl crate::Readable for PLL2FRACRrs {}
///`write(|w| ..)` method takes [`pll2fracr::W`](W) writer structure
impl crate::Writable for PLL2FRACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL2FRACR to value 0
impl crate::Resettable for PLL2FRACRrs {}
