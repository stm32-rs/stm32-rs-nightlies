///Register `RCK4SELR` reader
pub type R = crate::R<RCK4SELRrs>;
///Register `RCK4SELR` writer
pub type W = crate::W<RCK4SELRrs>;
///Field `PLL4SRC` reader - PLL4SRC
pub type PLL4SRC_R = crate::FieldReader;
///Field `PLL4SRC` writer - PLL4SRC
pub type PLL4SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PLL4SRCRDY` reader - PLL4SRCRDY
pub type PLL4SRCRDY_R = crate::BitReader;
impl R {
    ///Bits 0:1 - PLL4SRC
    #[inline(always)]
    pub fn pll4src(&self) -> PLL4SRC_R {
        PLL4SRC_R::new((self.bits & 3) as u8)
    }
    ///Bit 31 - PLL4SRCRDY
    #[inline(always)]
    pub fn pll4srcrdy(&self) -> PLL4SRCRDY_R {
        PLL4SRCRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCK4SELR")
            .field("pll4src", &self.pll4src())
            .field("pll4srcrdy", &self.pll4srcrdy())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - PLL4SRC
    #[inline(always)]
    pub fn pll4src(&mut self) -> PLL4SRC_W<'_, RCK4SELRrs> {
        PLL4SRC_W::new(self, 0)
    }
}
/**This register is used to select the reference clock for PLL4.

You can [`read`](crate::Reg::read) this register and get [`rck4selr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rck4selr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:RCK4SELR)*/
pub struct RCK4SELRrs;
impl crate::RegisterSpec for RCK4SELRrs {
    type Ux = u32;
}
///`read()` method returns [`rck4selr::R`](R) reader structure
impl crate::Readable for RCK4SELRrs {}
///`write(|w| ..)` method takes [`rck4selr::W`](W) writer structure
impl crate::Writable for RCK4SELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCK4SELR to value 0x8000_0000
impl crate::Resettable for RCK4SELRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
