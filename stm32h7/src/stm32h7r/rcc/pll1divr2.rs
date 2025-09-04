///Register `PLL1DIVR2` reader
pub type R = crate::R<PLL1DIVR2rs>;
///Register `PLL1DIVR2` writer
pub type W = crate::W<PLL1DIVR2rs>;
///Field `DIVS` reader - PLL1 DIVS division factor Set and reset by software to control the frequency of the pll1_s_ck clock. This post-divider performs divisions with 50% duty-cycle. The duty-cycle of 50% is guaranteed only in the following conditions: With VCOL, if (DIVS+1) is even, With VCOH, for all DIVS values These bits can be written only when the PLL1DIVSEN = 0.
pub type DIVS_R = crate::FieldReader;
///Field `DIVS` writer - PLL1 DIVS division factor Set and reset by software to control the frequency of the pll1_s_ck clock. This post-divider performs divisions with 50% duty-cycle. The duty-cycle of 50% is guaranteed only in the following conditions: With VCOL, if (DIVS+1) is even, With VCOH, for all DIVS values These bits can be written only when the PLL1DIVSEN = 0.
pub type DIVS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DIVT` reader - PLL1 DIVT division factor Set and reset by software to control the frequency of the pll1_t_ck clock. This post-divider performs divisions with 50% duty-cycle. The duty-cycle of 50% is guaranteed only in the following conditions: With VCOL, if (DIVT+1) is even, With VCOH, for all DIVT values These bits can be written only when the PLL1DIVTEN = 0.
pub type DIVT_R = crate::FieldReader;
///Field `DIVT` writer - PLL1 DIVT division factor Set and reset by software to control the frequency of the pll1_t_ck clock. This post-divider performs divisions with 50% duty-cycle. The duty-cycle of 50% is guaranteed only in the following conditions: With VCOL, if (DIVT+1) is even, With VCOH, for all DIVT values These bits can be written only when the PLL1DIVTEN = 0.
pub type DIVT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - PLL1 DIVS division factor Set and reset by software to control the frequency of the pll1_s_ck clock. This post-divider performs divisions with 50% duty-cycle. The duty-cycle of 50% is guaranteed only in the following conditions: With VCOL, if (DIVS+1) is even, With VCOH, for all DIVS values These bits can be written only when the PLL1DIVSEN = 0.
    #[inline(always)]
    pub fn divs(&self) -> DIVS_R {
        DIVS_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:10 - PLL1 DIVT division factor Set and reset by software to control the frequency of the pll1_t_ck clock. This post-divider performs divisions with 50% duty-cycle. The duty-cycle of 50% is guaranteed only in the following conditions: With VCOL, if (DIVT+1) is even, With VCOH, for all DIVT values These bits can be written only when the PLL1DIVTEN = 0.
    #[inline(always)]
    pub fn divt(&self) -> DIVT_R {
        DIVT_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1DIVR2")
            .field("divs", &self.divs())
            .field("divt", &self.divt())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - PLL1 DIVS division factor Set and reset by software to control the frequency of the pll1_s_ck clock. This post-divider performs divisions with 50% duty-cycle. The duty-cycle of 50% is guaranteed only in the following conditions: With VCOL, if (DIVS+1) is even, With VCOH, for all DIVS values These bits can be written only when the PLL1DIVSEN = 0.
    #[inline(always)]
    pub fn divs(&mut self) -> DIVS_W<PLL1DIVR2rs> {
        DIVS_W::new(self, 0)
    }
    ///Bits 8:10 - PLL1 DIVT division factor Set and reset by software to control the frequency of the pll1_t_ck clock. This post-divider performs divisions with 50% duty-cycle. The duty-cycle of 50% is guaranteed only in the following conditions: With VCOL, if (DIVT+1) is even, With VCOH, for all DIVT values These bits can be written only when the PLL1DIVTEN = 0.
    #[inline(always)]
    pub fn divt(&mut self) -> DIVT_W<PLL1DIVR2rs> {
        DIVT_W::new(self, 8)
    }
}
/**RCC PLL1 dividers configuration register 2

You can [`read`](crate::Reg::read) this register and get [`pll1divr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1divr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:PLL1DIVR2)*/
pub struct PLL1DIVR2rs;
impl crate::RegisterSpec for PLL1DIVR2rs {
    type Ux = u32;
}
///`read()` method returns [`pll1divr2::R`](R) reader structure
impl crate::Readable for PLL1DIVR2rs {}
///`write(|w| ..)` method takes [`pll1divr2::W`](W) writer structure
impl crate::Writable for PLL1DIVR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL1DIVR2 to value 0x0101
impl crate::Resettable for PLL1DIVR2rs {
    const RESET_VALUE: u32 = 0x0101;
}
