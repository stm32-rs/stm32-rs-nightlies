///Register `PLL2DIVR2` reader
pub type R = crate::R<PLL2DIVR2rs>;
///Register `PLL2DIVR2` writer
pub type W = crate::W<PLL2DIVR2rs>;
///Field `DIVS` reader - PLL2 DIVS division factor
pub type DIVS_R = crate::FieldReader;
///Field `DIVS` writer - PLL2 DIVS division factor
pub type DIVS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DIVT` reader - PLL2 DIVT division factor
pub type DIVT_R = crate::FieldReader;
///Field `DIVT` writer - PLL2 DIVT division factor
pub type DIVT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - PLL2 DIVS division factor
    #[inline(always)]
    pub fn divs(&self) -> DIVS_R {
        DIVS_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:10 - PLL2 DIVT division factor
    #[inline(always)]
    pub fn divt(&self) -> DIVT_R {
        DIVT_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL2DIVR2")
            .field("divs", &self.divs())
            .field("divt", &self.divt())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - PLL2 DIVS division factor
    #[inline(always)]
    pub fn divs(&mut self) -> DIVS_W<'_, PLL2DIVR2rs> {
        DIVS_W::new(self, 0)
    }
    ///Bits 8:10 - PLL2 DIVT division factor
    #[inline(always)]
    pub fn divt(&mut self) -> DIVT_W<'_, PLL2DIVR2rs> {
        DIVT_W::new(self, 8)
    }
}
/**RCC PLL2 dividers configuration register 2

You can [`read`](crate::Reg::read) this register and get [`pll2divr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2divr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:PLL2DIVR2)*/
pub struct PLL2DIVR2rs;
impl crate::RegisterSpec for PLL2DIVR2rs {
    type Ux = u32;
}
///`read()` method returns [`pll2divr2::R`](R) reader structure
impl crate::Readable for PLL2DIVR2rs {}
///`write(|w| ..)` method takes [`pll2divr2::W`](W) writer structure
impl crate::Writable for PLL2DIVR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL2DIVR2 to value 0x0101
impl crate::Resettable for PLL2DIVR2rs {
    const RESET_VALUE: u32 = 0x0101;
}
