///Register `PLL3DIVR2` reader
pub type R = crate::R<PLL3DIVR2rs>;
///Register `PLL3DIVR2` writer
pub type W = crate::W<PLL3DIVR2rs>;
///Field `DIVS` reader - PLL3 DIVS division factor
pub type DIVS_R = crate::FieldReader;
///Field `DIVS` writer - PLL3 DIVS division factor
pub type DIVS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - PLL3 DIVS division factor
    #[inline(always)]
    pub fn divs(&self) -> DIVS_R {
        DIVS_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL3DIVR2")
            .field("divs", &self.divs())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - PLL3 DIVS division factor
    #[inline(always)]
    pub fn divs(&mut self) -> DIVS_W<'_, PLL3DIVR2rs> {
        DIVS_W::new(self, 0)
    }
}
/**RCC PLL3 dividers configuration register 2

You can [`read`](crate::Reg::read) this register and get [`pll3divr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3divr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RCC:PLL3DIVR2)*/
pub struct PLL3DIVR2rs;
impl crate::RegisterSpec for PLL3DIVR2rs {
    type Ux = u32;
}
///`read()` method returns [`pll3divr2::R`](R) reader structure
impl crate::Readable for PLL3DIVR2rs {}
///`write(|w| ..)` method takes [`pll3divr2::W`](W) writer structure
impl crate::Writable for PLL3DIVR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL3DIVR2 to value 0x0101
impl crate::Resettable for PLL3DIVR2rs {
    const RESET_VALUE: u32 = 0x0101;
}
