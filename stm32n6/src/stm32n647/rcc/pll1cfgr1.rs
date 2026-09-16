///Register `PLL1CFGR1` reader
pub type R = crate::R<PLL1CFGR1rs>;
///Register `PLL1CFGR1` writer
pub type W = crate::W<PLL1CFGR1rs>;
///Field `PLL1DIVN` reader - PLL1 Integer part for the VCO multiplication factor
pub type PLL1DIVN_R = crate::FieldReader<u16>;
///Field `PLL1DIVN` writer - PLL1 Integer part for the VCO multiplication factor
pub type PLL1DIVN_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `PLL1DIVM` reader - PLL1 reference input clock divide frequency ratio
pub type PLL1DIVM_R = crate::FieldReader;
///Field `PLL1DIVM` writer - PLL1 reference input clock divide frequency ratio
pub type PLL1DIVM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `PLL1BYP` reader - PLL1 bypass
pub type PLL1BYP_R = crate::BitReader;
///Field `PLL1BYP` writer - PLL1 bypass
pub type PLL1BYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1SEL` reader - PLL1 source selection of the reference clock
pub type PLL1SEL_R = crate::FieldReader;
///Field `PLL1SEL` writer - PLL1 source selection of the reference clock
pub type PLL1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 8:19 - PLL1 Integer part for the VCO multiplication factor
    #[inline(always)]
    pub fn pll1divn(&self) -> PLL1DIVN_R {
        PLL1DIVN_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    ///Bits 20:25 - PLL1 reference input clock divide frequency ratio
    #[inline(always)]
    pub fn pll1divm(&self) -> PLL1DIVM_R {
        PLL1DIVM_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    ///Bit 27 - PLL1 bypass
    #[inline(always)]
    pub fn pll1byp(&self) -> PLL1BYP_R {
        PLL1BYP_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:30 - PLL1 source selection of the reference clock
    #[inline(always)]
    pub fn pll1sel(&self) -> PLL1SEL_R {
        PLL1SEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1CFGR1")
            .field("pll1divn", &self.pll1divn())
            .field("pll1divm", &self.pll1divm())
            .field("pll1byp", &self.pll1byp())
            .field("pll1sel", &self.pll1sel())
            .finish()
    }
}
impl W {
    ///Bits 8:19 - PLL1 Integer part for the VCO multiplication factor
    #[inline(always)]
    pub fn pll1divn(&mut self) -> PLL1DIVN_W<'_, PLL1CFGR1rs> {
        PLL1DIVN_W::new(self, 8)
    }
    ///Bits 20:25 - PLL1 reference input clock divide frequency ratio
    #[inline(always)]
    pub fn pll1divm(&mut self) -> PLL1DIVM_W<'_, PLL1CFGR1rs> {
        PLL1DIVM_W::new(self, 20)
    }
    ///Bit 27 - PLL1 bypass
    #[inline(always)]
    pub fn pll1byp(&mut self) -> PLL1BYP_W<'_, PLL1CFGR1rs> {
        PLL1BYP_W::new(self, 27)
    }
    ///Bits 28:30 - PLL1 source selection of the reference clock
    #[inline(always)]
    pub fn pll1sel(&mut self) -> PLL1SEL_W<'_, PLL1CFGR1rs> {
        PLL1SEL_W::new(self, 28)
    }
}
/**RCC PLL1 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`pll1cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PLL1CFGR1)*/
pub struct PLL1CFGR1rs;
impl crate::RegisterSpec for PLL1CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`pll1cfgr1::R`](R) reader structure
impl crate::Readable for PLL1CFGR1rs {}
///`write(|w| ..)` method takes [`pll1cfgr1::W`](W) writer structure
impl crate::Writable for PLL1CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL1CFGR1 to value 0x0820_2500
impl crate::Resettable for PLL1CFGR1rs {
    const RESET_VALUE: u32 = 0x0820_2500;
}
