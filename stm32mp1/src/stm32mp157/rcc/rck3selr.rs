///Register `RCK3SELR` reader
pub type R = crate::R<RCK3SELRrs>;
///Register `RCK3SELR` writer
pub type W = crate::W<RCK3SELRrs>;
///Field `PLL3SRC` reader - PLL3SRC
pub type PLL3SRC_R = crate::FieldReader;
///Field `PLL3SRC` writer - PLL3SRC
pub type PLL3SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PLL3SRCRDY` reader - PLL3SRCRDY
pub type PLL3SRCRDY_R = crate::BitReader;
impl R {
    ///Bits 0:1 - PLL3SRC
    #[inline(always)]
    pub fn pll3src(&self) -> PLL3SRC_R {
        PLL3SRC_R::new((self.bits & 3) as u8)
    }
    ///Bit 31 - PLL3SRCRDY
    #[inline(always)]
    pub fn pll3srcrdy(&self) -> PLL3SRCRDY_R {
        PLL3SRCRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCK3SELR")
            .field("pll3src", &self.pll3src())
            .field("pll3srcrdy", &self.pll3srcrdy())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - PLL3SRC
    #[inline(always)]
    pub fn pll3src(&mut self) -> PLL3SRC_W<'_, RCK3SELRrs> {
        PLL3SRC_W::new(self, 0)
    }
}
/**This register is used to select the reference clock for PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`rck3selr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rck3selr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:RCK3SELR)*/
pub struct RCK3SELRrs;
impl crate::RegisterSpec for RCK3SELRrs {
    type Ux = u32;
}
///`read()` method returns [`rck3selr::R`](R) reader structure
impl crate::Readable for RCK3SELRrs {}
///`write(|w| ..)` method takes [`rck3selr::W`](W) writer structure
impl crate::Writable for RCK3SELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCK3SELR to value 0x8000_0000
impl crate::Resettable for RCK3SELRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
