///Register `PLL4CFGR1` reader
pub type R = crate::R<PLL4CFGR1rs>;
///Register `PLL4CFGR1` writer
pub type W = crate::W<PLL4CFGR1rs>;
///Field `PLL4DIVN` reader - PLL4 Integer part for the VCO multiplication factor
pub type PLL4DIVN_R = crate::FieldReader<u16>;
///Field `PLL4DIVN` writer - PLL4 Integer part for the VCO multiplication factor
pub type PLL4DIVN_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `PLL4DIVM` reader - PLL4 reference input clock divide frequency ratio
pub type PLL4DIVM_R = crate::FieldReader;
///Field `PLL4DIVM` writer - PLL4 reference input clock divide frequency ratio
pub type PLL4DIVM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `PLL4BYP` reader - PLL4 bypass
pub type PLL4BYP_R = crate::BitReader;
///Field `PLL4BYP` writer - PLL4 bypass
pub type PLL4BYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL4SEL` reader - PLL4 source selection of the reference clock
pub type PLL4SEL_R = crate::FieldReader;
///Field `PLL4SEL` writer - PLL4 source selection of the reference clock
pub type PLL4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 8:19 - PLL4 Integer part for the VCO multiplication factor
    #[inline(always)]
    pub fn pll4divn(&self) -> PLL4DIVN_R {
        PLL4DIVN_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    ///Bits 20:25 - PLL4 reference input clock divide frequency ratio
    #[inline(always)]
    pub fn pll4divm(&self) -> PLL4DIVM_R {
        PLL4DIVM_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    ///Bit 27 - PLL4 bypass
    #[inline(always)]
    pub fn pll4byp(&self) -> PLL4BYP_R {
        PLL4BYP_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:30 - PLL4 source selection of the reference clock
    #[inline(always)]
    pub fn pll4sel(&self) -> PLL4SEL_R {
        PLL4SEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL4CFGR1")
            .field("pll4divn", &self.pll4divn())
            .field("pll4divm", &self.pll4divm())
            .field("pll4byp", &self.pll4byp())
            .field("pll4sel", &self.pll4sel())
            .finish()
    }
}
impl W {
    ///Bits 8:19 - PLL4 Integer part for the VCO multiplication factor
    #[inline(always)]
    pub fn pll4divn(&mut self) -> PLL4DIVN_W<'_, PLL4CFGR1rs> {
        PLL4DIVN_W::new(self, 8)
    }
    ///Bits 20:25 - PLL4 reference input clock divide frequency ratio
    #[inline(always)]
    pub fn pll4divm(&mut self) -> PLL4DIVM_W<'_, PLL4CFGR1rs> {
        PLL4DIVM_W::new(self, 20)
    }
    ///Bit 27 - PLL4 bypass
    #[inline(always)]
    pub fn pll4byp(&mut self) -> PLL4BYP_W<'_, PLL4CFGR1rs> {
        PLL4BYP_W::new(self, 27)
    }
    ///Bits 28:30 - PLL4 source selection of the reference clock
    #[inline(always)]
    pub fn pll4sel(&mut self) -> PLL4SEL_W<'_, PLL4CFGR1rs> {
        PLL4SEL_W::new(self, 28)
    }
}
/**RCC PLL4 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`pll4cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll4cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:PLL4CFGR1)*/
pub struct PLL4CFGR1rs;
impl crate::RegisterSpec for PLL4CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`pll4cfgr1::R`](R) reader structure
impl crate::Readable for PLL4CFGR1rs {}
///`write(|w| ..)` method takes [`pll4cfgr1::W`](W) writer structure
impl crate::Writable for PLL4CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL4CFGR1 to value 0x0800_0000
impl crate::Resettable for PLL4CFGR1rs {
    const RESET_VALUE: u32 = 0x0800_0000;
}
