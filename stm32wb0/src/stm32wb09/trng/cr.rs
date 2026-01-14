///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `DISABLE` reader - Disable Bit DISABLE can be used for reading or setting the state of the TRNG core. The value read is always the one available at the rng core clock domain. When changing the value, the change is effective when the value read is the same as the one written.
pub type DISABLE_R = crate::BitReader;
///Field `DISABLE` writer - Disable Bit DISABLE can be used for reading or setting the state of the TRNG core. The value read is always the one available at the rng core clock domain. When changing the value, the change is effective when the value read is the same as the one written.
pub type DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLR_REVCLK_FLAG` reader - Reset reveal clock error flags when writing a '1' without resetting the whole TRNG. When writing a 1, the value remains until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset.
pub type CLR_REVCLK_FLAG_R = crate::BitReader;
///Field `CLR_REVCLK_FLAG` writer - Reset reveal clock error flags when writing a '1' without resetting the whole TRNG. When writing a 1, the value remains until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset.
pub type CLR_REVCLK_FLAG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RST_HEALTH_FLAGS` reader - Reset Health error flags when writing a '1' without resetting the whole TRNG. When writing a 1, the value remains until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset.
pub type RST_HEALTH_FLAGS_R = crate::BitReader;
///Field `RST_HEALTH_FLAGS` writer - Reset Health error flags when writing a '1' without resetting the whole TRNG. When writing a 1, the value remains until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset.
pub type RST_HEALTH_FLAGS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLKDIV` reader - Sampling Clock Enable Divider. CLKDIV\[15:0\] control the sampling clock enable divider, dividing by a factor equal to CLKDIV\[15:0\] + 1, values being in the range of 1 to 65536.
pub type CLKDIV_R = crate::FieldReader<u16>;
///Field `CLKDIV` writer - Sampling Clock Enable Divider. CLKDIV\[15:0\] control the sampling clock enable divider, dividing by a factor equal to CLKDIV\[15:0\] + 1, values being in the range of 1 to 65536.
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bit 0 - Disable Bit DISABLE can be used for reading or setting the state of the TRNG core. The value read is always the one available at the rng core clock domain. When changing the value, the change is effective when the value read is the same as the one written.
    #[inline(always)]
    pub fn disable(&self) -> DISABLE_R {
        DISABLE_R::new((self.bits & 1) != 0)
    }
    ///Bit 6 - Reset reveal clock error flags when writing a '1' without resetting the whole TRNG. When writing a 1, the value remains until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset.
    #[inline(always)]
    pub fn clr_revclk_flag(&self) -> CLR_REVCLK_FLAG_R {
        CLR_REVCLK_FLAG_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Reset Health error flags when writing a '1' without resetting the whole TRNG. When writing a 1, the value remains until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset.
    #[inline(always)]
    pub fn rst_health_flags(&self) -> RST_HEALTH_FLAGS_R {
        RST_HEALTH_FLAGS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:23 - Sampling Clock Enable Divider. CLKDIV\[15:0\] control the sampling clock enable divider, dividing by a factor equal to CLKDIV\[15:0\] + 1, values being in the range of 1 to 65536.
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("disable", &self.disable())
            .field("clr_revclk_flag", &self.clr_revclk_flag())
            .field("rst_health_flags", &self.rst_health_flags())
            .field("clkdiv", &self.clkdiv())
            .finish()
    }
}
impl W {
    ///Bit 0 - Disable Bit DISABLE can be used for reading or setting the state of the TRNG core. The value read is always the one available at the rng core clock domain. When changing the value, the change is effective when the value read is the same as the one written.
    #[inline(always)]
    pub fn disable(&mut self) -> DISABLE_W<'_, CRrs> {
        DISABLE_W::new(self, 0)
    }
    ///Bit 6 - Reset reveal clock error flags when writing a '1' without resetting the whole TRNG. When writing a 1, the value remains until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset.
    #[inline(always)]
    pub fn clr_revclk_flag(&mut self) -> CLR_REVCLK_FLAG_W<'_, CRrs> {
        CLR_REVCLK_FLAG_W::new(self, 6)
    }
    ///Bit 7 - Reset Health error flags when writing a '1' without resetting the whole TRNG. When writing a 1, the value remains until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset.
    #[inline(always)]
    pub fn rst_health_flags(&mut self) -> RST_HEALTH_FLAGS_W<'_, CRrs> {
        RST_HEALTH_FLAGS_W::new(self, 7)
    }
    ///Bits 8:23 - Sampling Clock Enable Divider. CLKDIV\[15:0\] control the sampling clock enable divider, dividing by a factor equal to CLKDIV\[15:0\] + 1, values being in the range of 1 to 65536.
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<'_, CRrs> {
        CLKDIV_W::new(self, 8)
    }
}
/**TRNG_CR register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:CR)*/
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
