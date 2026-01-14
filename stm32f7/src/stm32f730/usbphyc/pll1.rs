///Register `PLL1` reader
pub type R = crate::R<PLL1rs>;
///Register `PLL1` writer
pub type W = crate::W<PLL1rs>;
///Field `PLL1EN` reader - Enable the PLL1 inside PHY
pub type PLL1EN_R = crate::BitReader;
///Field `PLL1EN` writer - Enable the PLL1 inside PHY
pub type PLL1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1SEL` reader - : Controls the PHY PLL1 input clock frequency selection
pub type PLL1SEL_R = crate::FieldReader;
///Field `PLL1SEL` writer - : Controls the PHY PLL1 input clock frequency selection
pub type PLL1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - Enable the PLL1 inside PHY
    #[inline(always)]
    pub fn pll1en(&self) -> PLL1EN_R {
        PLL1EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - : Controls the PHY PLL1 input clock frequency selection
    #[inline(always)]
    pub fn pll1sel(&self) -> PLL1SEL_R {
        PLL1SEL_R::new(((self.bits >> 1) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1")
            .field("pll1en", &self.pll1en())
            .field("pll1sel", &self.pll1sel())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable the PLL1 inside PHY
    #[inline(always)]
    pub fn pll1en(&mut self) -> PLL1EN_W<'_, PLL1rs> {
        PLL1EN_W::new(self, 0)
    }
    ///Bits 1:3 - : Controls the PHY PLL1 input clock frequency selection
    #[inline(always)]
    pub fn pll1sel(&mut self) -> PLL1SEL_W<'_, PLL1rs> {
        PLL1SEL_W::new(self, 1)
    }
}
/**USBPHYC PLL1 control register

You can [`read`](crate::Reg::read) this register and get [`pll1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F730.html#USBPHYC:PLL1)*/
pub struct PLL1rs;
impl crate::RegisterSpec for PLL1rs {
    type Ux = u32;
}
///`read()` method returns [`pll1::R`](R) reader structure
impl crate::Readable for PLL1rs {}
///`write(|w| ..)` method takes [`pll1::W`](W) writer structure
impl crate::Writable for PLL1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL1 to value 0
impl crate::Resettable for PLL1rs {}
