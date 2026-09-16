///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `RNG_DIS` reader - This bit enables or disables the random number generator. 0: RNG is enabled (default) 1: RNG is disabled. The internal free-running oscillators are put in power-down mode and the RNG clock is stopped at the input of the block.
pub type RNG_DIS_R = crate::BitReader;
///Field `RNG_DIS` writer - This bit enables or disables the random number generator. 0: RNG is enabled (default) 1: RNG is disabled. The internal free-running oscillators are put in power-down mode and the RNG clock is stopped at the input of the block.
pub type RNG_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TST_CLK` reader - Reset reveal clock error flags when writing a '1' without resetting the whole TRNG. When writing a 1, the value remains until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset.
pub type TST_CLK_R = crate::BitReader;
///Field `TST_CLK` writer - Reset reveal clock error flags when writing a '1' without resetting the whole TRNG. When writing a 1, the value remains until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset.
pub type TST_CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - This bit enables or disables the random number generator. 0: RNG is enabled (default) 1: RNG is disabled. The internal free-running oscillators are put in power-down mode and the RNG clock is stopped at the input of the block.
    #[inline(always)]
    pub fn rng_dis(&self) -> RNG_DIS_R {
        RNG_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Reset reveal clock error flags when writing a '1' without resetting the whole TRNG. When writing a 1, the value remains until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset.
    #[inline(always)]
    pub fn tst_clk(&self) -> TST_CLK_R {
        TST_CLK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("rng_dis", &self.rng_dis())
            .field("tst_clk", &self.tst_clk())
            .finish()
    }
}
impl W {
    ///Bit 1 - This bit enables or disables the random number generator. 0: RNG is enabled (default) 1: RNG is disabled. The internal free-running oscillators are put in power-down mode and the RNG clock is stopped at the input of the block.
    #[inline(always)]
    pub fn rng_dis(&mut self) -> RNG_DIS_W<'_, CRrs> {
        RNG_DIS_W::new(self, 1)
    }
    ///Bit 3 - Reset reveal clock error flags when writing a '1' without resetting the whole TRNG. When writing a 1, the value remains until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset.
    #[inline(always)]
    pub fn tst_clk(&mut self) -> TST_CLK_W<'_, CRrs> {
        TST_CLK_W::new(self, 3)
    }
}
/**RNG_CR register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RNG:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0xff00
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0xff00;
}
