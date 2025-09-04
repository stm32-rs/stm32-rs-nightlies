///Register `CCCSR` reader
pub type R = crate::R<CCCSRrs>;
///Register `CCCSR` writer
pub type W = crate::W<CCCSRrs>;
///Field `EN1` reader - VDD I/Os compensation cell enable This bit enables the compensation cell of the I/Os supplied by VsubDD/sub.
pub type EN1_R = crate::BitReader;
///Field `EN1` writer - VDD I/Os compensation cell enable This bit enables the compensation cell of the I/Os supplied by VsubDD/sub.
pub type EN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CS1` reader - VDD I/Os code selection This bit selects the code to be applied for the compensation cell of the I/Os supplied by VsubDD/sub.
pub type CS1_R = crate::BitReader;
///Field `CS1` writer - VDD I/Os code selection This bit selects the code to be applied for the compensation cell of the I/Os supplied by VsubDD/sub.
pub type CS1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDY1` reader - VDD I/Os compensation cell ready flag This bit provides the compensation cell status of the I/Os supplied by VsubDD/sub. Note: The HSI16 clock is required for the compensation cell to work properly. The compensation cell ready bit (RDY1) is not set if the HSI16 clock is not enabled (HSION).
pub type RDY1_R = crate::BitReader;
impl R {
    ///Bit 0 - VDD I/Os compensation cell enable This bit enables the compensation cell of the I/Os supplied by VsubDD/sub.
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VDD I/Os code selection This bit selects the code to be applied for the compensation cell of the I/Os supplied by VsubDD/sub.
    #[inline(always)]
    pub fn cs1(&self) -> CS1_R {
        CS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - VDD I/Os compensation cell ready flag This bit provides the compensation cell status of the I/Os supplied by VsubDD/sub. Note: The HSI16 clock is required for the compensation cell to work properly. The compensation cell ready bit (RDY1) is not set if the HSI16 clock is not enabled (HSION).
    #[inline(always)]
    pub fn rdy1(&self) -> RDY1_R {
        RDY1_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCCSR")
            .field("en1", &self.en1())
            .field("cs1", &self.cs1())
            .field("rdy1", &self.rdy1())
            .finish()
    }
}
impl W {
    ///Bit 0 - VDD I/Os compensation cell enable This bit enables the compensation cell of the I/Os supplied by VsubDD/sub.
    #[inline(always)]
    pub fn en1(&mut self) -> EN1_W<CCCSRrs> {
        EN1_W::new(self, 0)
    }
    ///Bit 1 - VDD I/Os code selection This bit selects the code to be applied for the compensation cell of the I/Os supplied by VsubDD/sub.
    #[inline(always)]
    pub fn cs1(&mut self) -> CS1_W<CCCSRrs> {
        CS1_W::new(self, 1)
    }
}
/**SYSCFG compensation cell control/status register

You can [`read`](crate::Reg::read) this register and get [`cccsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#SYSCFG:CCCSR)*/
pub struct CCCSRrs;
impl crate::RegisterSpec for CCCSRrs {
    type Ux = u32;
}
///`read()` method returns [`cccsr::R`](R) reader structure
impl crate::Readable for CCCSRrs {}
///`write(|w| ..)` method takes [`cccsr::W`](W) writer structure
impl crate::Writable for CCCSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCCSR to value 0x02
impl crate::Resettable for CCCSRrs {
    const RESET_VALUE: u32 = 0x02;
}
