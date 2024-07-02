///Register `RCC_RCK12SELR` reader
pub type R = crate::R<RCC_RCK12SELRrs>;
///Register `RCC_RCK12SELR` writer
pub type W = crate::W<RCC_RCK12SELRrs>;
///Field `PLL12SRC` reader - PLL12SRC
pub type PLL12SRC_R = crate::FieldReader;
///Field `PLL12SRC` writer - PLL12SRC
pub type PLL12SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PLL12SRCRDY` reader - PLL12SRCRDY
pub type PLL12SRCRDY_R = crate::BitReader;
impl R {
    ///Bits 0:1 - PLL12SRC
    #[inline(always)]
    pub fn pll12src(&self) -> PLL12SRC_R {
        PLL12SRC_R::new((self.bits & 3) as u8)
    }
    ///Bit 31 - PLL12SRCRDY
    #[inline(always)]
    pub fn pll12srcrdy(&self) -> PLL12SRCRDY_R {
        PLL12SRCRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_RCK12SELR")
            .field("pll12src", &self.pll12src())
            .field("pll12srcrdy", &self.pll12srcrdy())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - PLL12SRC
    #[inline(always)]
    #[must_use]
    pub fn pll12src(&mut self) -> PLL12SRC_W<RCC_RCK12SELRrs> {
        PLL12SRC_W::new(self, 0)
    }
}
/**This register is used to select the reference clock for PLL1 and PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`rcc_rck12selr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_rck12selr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:RCC_RCK12SELR)*/
pub struct RCC_RCK12SELRrs;
impl crate::RegisterSpec for RCC_RCK12SELRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_rck12selr::R`](R) reader structure
impl crate::Readable for RCC_RCK12SELRrs {}
///`write(|w| ..)` method takes [`rcc_rck12selr::W`](W) writer structure
impl crate::Writable for RCC_RCK12SELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_RCK12SELR to value 0x8000_0000
impl crate::Resettable for RCC_RCK12SELRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
