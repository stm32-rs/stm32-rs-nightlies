///Register `PLL2CFGR1` reader
pub type R = crate::R<PLL2CFGR1rs>;
///Register `PLL2CFGR1` writer
pub type W = crate::W<PLL2CFGR1rs>;
///Field `PLL2DIVN` reader - PLL2 Integer part for the VCO multiplication factor
pub type PLL2DIVN_R = crate::FieldReader<u16>;
///Field `PLL2DIVN` writer - PLL2 Integer part for the VCO multiplication factor
pub type PLL2DIVN_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `PLL2DIVM` reader - PLL2 reference input clock divide frequency ratio
pub type PLL2DIVM_R = crate::FieldReader;
///Field `PLL2DIVM` writer - PLL2 reference input clock divide frequency ratio
pub type PLL2DIVM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `PLL2BYP` reader - PLL2 bypass
pub type PLL2BYP_R = crate::BitReader;
///Field `PLL2BYP` writer - PLL2 bypass
pub type PLL2BYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2SEL` reader - PLL2 source selection of the reference clock
pub type PLL2SEL_R = crate::FieldReader;
///Field `PLL2SEL` writer - PLL2 source selection of the reference clock
pub type PLL2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 8:19 - PLL2 Integer part for the VCO multiplication factor
    #[inline(always)]
    pub fn pll2divn(&self) -> PLL2DIVN_R {
        PLL2DIVN_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    ///Bits 20:25 - PLL2 reference input clock divide frequency ratio
    #[inline(always)]
    pub fn pll2divm(&self) -> PLL2DIVM_R {
        PLL2DIVM_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    ///Bit 27 - PLL2 bypass
    #[inline(always)]
    pub fn pll2byp(&self) -> PLL2BYP_R {
        PLL2BYP_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:30 - PLL2 source selection of the reference clock
    #[inline(always)]
    pub fn pll2sel(&self) -> PLL2SEL_R {
        PLL2SEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL2CFGR1")
            .field("pll2divn", &self.pll2divn())
            .field("pll2divm", &self.pll2divm())
            .field("pll2byp", &self.pll2byp())
            .field("pll2sel", &self.pll2sel())
            .finish()
    }
}
impl W {
    ///Bits 8:19 - PLL2 Integer part for the VCO multiplication factor
    #[inline(always)]
    pub fn pll2divn(&mut self) -> PLL2DIVN_W<'_, PLL2CFGR1rs> {
        PLL2DIVN_W::new(self, 8)
    }
    ///Bits 20:25 - PLL2 reference input clock divide frequency ratio
    #[inline(always)]
    pub fn pll2divm(&mut self) -> PLL2DIVM_W<'_, PLL2CFGR1rs> {
        PLL2DIVM_W::new(self, 20)
    }
    ///Bit 27 - PLL2 bypass
    #[inline(always)]
    pub fn pll2byp(&mut self) -> PLL2BYP_W<'_, PLL2CFGR1rs> {
        PLL2BYP_W::new(self, 27)
    }
    ///Bits 28:30 - PLL2 source selection of the reference clock
    #[inline(always)]
    pub fn pll2sel(&mut self) -> PLL2SEL_W<'_, PLL2CFGR1rs> {
        PLL2SEL_W::new(self, 28)
    }
}
/**RCC PLL2 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`pll2cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:PLL2CFGR1)*/
pub struct PLL2CFGR1rs;
impl crate::RegisterSpec for PLL2CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`pll2cfgr1::R`](R) reader structure
impl crate::Readable for PLL2CFGR1rs {}
///`write(|w| ..)` method takes [`pll2cfgr1::W`](W) writer structure
impl crate::Writable for PLL2CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL2CFGR1 to value 0x0800_0000
impl crate::Resettable for PLL2CFGR1rs {
    const RESET_VALUE: u32 = 0x0800_0000;
}
