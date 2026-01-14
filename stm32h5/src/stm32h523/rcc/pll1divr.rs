///Register `PLL1DIVR` reader
pub type R = crate::R<PLL1DIVRrs>;
///Register `PLL1DIVR` writer
pub type W = crate::W<PLL1DIVRrs>;
///Field `PLL1N` reader - Multiplication factor for PLL1VCO
pub type PLL1N_R = crate::FieldReader<u16>;
///Field `PLL1N` writer - Multiplication factor for PLL1VCO
pub type PLL1N_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `PLL1P` reader - PLL1 DIVP division factor
pub type PLL1P_R = crate::FieldReader;
///Field `PLL1P` writer - PLL1 DIVP division factor
pub type PLL1P_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
///Field `PLL1Q` reader - PLL1 DIVQ division factor
pub type PLL1Q_R = crate::FieldReader;
///Field `PLL1Q` writer - PLL1 DIVQ division factor
pub type PLL1Q_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
///Field `PLL1R` reader - PLL1 DIVR division factor
pub type PLL1R_R = crate::FieldReader;
///Field `PLL1R` writer - PLL1 DIVR division factor
pub type PLL1R_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
impl R {
    ///Bits 0:8 - Multiplication factor for PLL1VCO
    #[inline(always)]
    pub fn pll1n(&self) -> PLL1N_R {
        PLL1N_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:15 - PLL1 DIVP division factor
    #[inline(always)]
    pub fn pll1p(&self) -> PLL1P_R {
        PLL1P_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    ///Bits 16:22 - PLL1 DIVQ division factor
    #[inline(always)]
    pub fn pll1q(&self) -> PLL1Q_R {
        PLL1Q_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bits 24:30 - PLL1 DIVR division factor
    #[inline(always)]
    pub fn pll1r(&self) -> PLL1R_R {
        PLL1R_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1DIVR")
            .field("pll1n", &self.pll1n())
            .field("pll1p", &self.pll1p())
            .field("pll1q", &self.pll1q())
            .field("pll1r", &self.pll1r())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Multiplication factor for PLL1VCO
    #[inline(always)]
    pub fn pll1n(&mut self) -> PLL1N_W<'_, PLL1DIVRrs> {
        PLL1N_W::new(self, 0)
    }
    ///Bits 9:15 - PLL1 DIVP division factor
    #[inline(always)]
    pub fn pll1p(&mut self) -> PLL1P_W<'_, PLL1DIVRrs> {
        PLL1P_W::new(self, 9)
    }
    ///Bits 16:22 - PLL1 DIVQ division factor
    #[inline(always)]
    pub fn pll1q(&mut self) -> PLL1Q_W<'_, PLL1DIVRrs> {
        PLL1Q_W::new(self, 16)
    }
    ///Bits 24:30 - PLL1 DIVR division factor
    #[inline(always)]
    pub fn pll1r(&mut self) -> PLL1R_W<'_, PLL1DIVRrs> {
        PLL1R_W::new(self, 24)
    }
}
/**RCC PLL1 dividers register

You can [`read`](crate::Reg::read) this register and get [`pll1divr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1divr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#RCC:PLL1DIVR)*/
pub struct PLL1DIVRrs;
impl crate::RegisterSpec for PLL1DIVRrs {
    type Ux = u32;
}
///`read()` method returns [`pll1divr::R`](R) reader structure
impl crate::Readable for PLL1DIVRrs {}
///`write(|w| ..)` method takes [`pll1divr::W`](W) writer structure
impl crate::Writable for PLL1DIVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL1DIVR to value 0x0101_0280
impl crate::Resettable for PLL1DIVRrs {
    const RESET_VALUE: u32 = 0x0101_0280;
}
