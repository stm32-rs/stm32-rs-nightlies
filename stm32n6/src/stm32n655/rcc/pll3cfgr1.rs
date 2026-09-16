///Register `PLL3CFGR1` reader
pub type R = crate::R<PLL3CFGR1rs>;
///Register `PLL3CFGR1` writer
pub type W = crate::W<PLL3CFGR1rs>;
///Field `PLL3DIVN` reader - PLL3 Integer part for the VCO multiplication factor
pub type PLL3DIVN_R = crate::FieldReader<u16>;
///Field `PLL3DIVN` writer - PLL3 Integer part for the VCO multiplication factor
pub type PLL3DIVN_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `PLL3DIVM` reader - PLL3 reference input clock divide frequency ratio
pub type PLL3DIVM_R = crate::FieldReader;
///Field `PLL3DIVM` writer - PLL3 reference input clock divide frequency ratio
pub type PLL3DIVM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `PLL3BYP` reader - PLL3 bypass
pub type PLL3BYP_R = crate::BitReader;
///Field `PLL3BYP` writer - PLL3 bypass
pub type PLL3BYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3SEL` reader - PLL3 source selection of the reference clock
pub type PLL3SEL_R = crate::FieldReader;
///Field `PLL3SEL` writer - PLL3 source selection of the reference clock
pub type PLL3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 8:19 - PLL3 Integer part for the VCO multiplication factor
    #[inline(always)]
    pub fn pll3divn(&self) -> PLL3DIVN_R {
        PLL3DIVN_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    ///Bits 20:25 - PLL3 reference input clock divide frequency ratio
    #[inline(always)]
    pub fn pll3divm(&self) -> PLL3DIVM_R {
        PLL3DIVM_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    ///Bit 27 - PLL3 bypass
    #[inline(always)]
    pub fn pll3byp(&self) -> PLL3BYP_R {
        PLL3BYP_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:30 - PLL3 source selection of the reference clock
    #[inline(always)]
    pub fn pll3sel(&self) -> PLL3SEL_R {
        PLL3SEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL3CFGR1")
            .field("pll3divn", &self.pll3divn())
            .field("pll3divm", &self.pll3divm())
            .field("pll3byp", &self.pll3byp())
            .field("pll3sel", &self.pll3sel())
            .finish()
    }
}
impl W {
    ///Bits 8:19 - PLL3 Integer part for the VCO multiplication factor
    #[inline(always)]
    pub fn pll3divn(&mut self) -> PLL3DIVN_W<'_, PLL3CFGR1rs> {
        PLL3DIVN_W::new(self, 8)
    }
    ///Bits 20:25 - PLL3 reference input clock divide frequency ratio
    #[inline(always)]
    pub fn pll3divm(&mut self) -> PLL3DIVM_W<'_, PLL3CFGR1rs> {
        PLL3DIVM_W::new(self, 20)
    }
    ///Bit 27 - PLL3 bypass
    #[inline(always)]
    pub fn pll3byp(&mut self) -> PLL3BYP_W<'_, PLL3CFGR1rs> {
        PLL3BYP_W::new(self, 27)
    }
    ///Bits 28:30 - PLL3 source selection of the reference clock
    #[inline(always)]
    pub fn pll3sel(&mut self) -> PLL3SEL_W<'_, PLL3CFGR1rs> {
        PLL3SEL_W::new(self, 28)
    }
}
/**RCC PLL3 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`pll3cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:PLL3CFGR1)*/
pub struct PLL3CFGR1rs;
impl crate::RegisterSpec for PLL3CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`pll3cfgr1::R`](R) reader structure
impl crate::Readable for PLL3CFGR1rs {}
///`write(|w| ..)` method takes [`pll3cfgr1::W`](W) writer structure
impl crate::Writable for PLL3CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL3CFGR1 to value 0x0800_0000
impl crate::Resettable for PLL3CFGR1rs {
    const RESET_VALUE: u32 = 0x0800_0000;
}
