///Register `RNG_SR` reader
pub type R = crate::R<RNG_SRrs>;
///Register `RNG_SR` writer
pub type W = crate::W<RNG_SRrs>;
///Field `DRDY` reader - Data ready Once the output buffer becomes empty (after reading the RNG_DR register), this bit returns to 0 until a new random value is generated. Note: The DRDY bit can rise when the peripheral is disabled (RNGEN1=10 in the RNG_CR register). If IE=1 in the RNG_CR register, an interrupt is generated when DRDY1=11.
pub type DRDY_R = crate::BitReader;
///Field `CECS` reader - Clock error current status Note: CECS bit is valid only if the CED bit in the RNG_CR register is set to 0.
pub type CECS_R = crate::BitReader;
///Field `SECS` reader - Seed error current status Runtime repetition count test failed (noise source has provided more than 24 consecutive bits at a constant value 0 or 1, or more than 32 consecutive occurrence of two bits patterns 01 or 10) Startup or continuous adaptive proportion test on noise source failed. Startup post-processing/conditioning sanity check failed.
pub type SECS_R = crate::BitReader;
///Field `CEIS` reader - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.
pub type CEIS_R = crate::BitReader;
///Field `CEIS` writer - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.
pub type CEIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEIS` reader - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0 (unless CONDRST is used). Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.
pub type SEIS_R = crate::BitReader;
///Field `SEIS` writer - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0 (unless CONDRST is used). Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.
pub type SEIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Data ready Once the output buffer becomes empty (after reading the RNG_DR register), this bit returns to 0 until a new random value is generated. Note: The DRDY bit can rise when the peripheral is disabled (RNGEN1=10 in the RNG_CR register). If IE=1 in the RNG_CR register, an interrupt is generated when DRDY1=11.
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clock error current status Note: CECS bit is valid only if the CED bit in the RNG_CR register is set to 0.
    #[inline(always)]
    pub fn cecs(&self) -> CECS_R {
        CECS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Seed error current status Runtime repetition count test failed (noise source has provided more than 24 consecutive bits at a constant value 0 or 1, or more than 32 consecutive occurrence of two bits patterns 01 or 10) Startup or continuous adaptive proportion test on noise source failed. Startup post-processing/conditioning sanity check failed.
    #[inline(always)]
    pub fn secs(&self) -> SECS_R {
        SECS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.
    #[inline(always)]
    pub fn ceis(&self) -> CEIS_R {
        CEIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0 (unless CONDRST is used). Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.
    #[inline(always)]
    pub fn seis(&self) -> SEIS_R {
        SEIS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RNG_SR")
            .field("drdy", &self.drdy())
            .field("cecs", &self.cecs())
            .field("secs", &self.secs())
            .field("ceis", &self.ceis())
            .field("seis", &self.seis())
            .finish()
    }
}
impl W {
    ///Bit 5 - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.
    #[inline(always)]
    pub fn ceis(&mut self) -> CEIS_W<RNG_SRrs> {
        CEIS_W::new(self, 5)
    }
    ///Bit 6 - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0 (unless CONDRST is used). Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.
    #[inline(always)]
    pub fn seis(&mut self) -> SEIS_W<RNG_SRrs> {
        SEIS_W::new(self, 6)
    }
}
/**RNG status register

You can [`read`](crate::Reg::read) this register and get [`rng_sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RNG:RNG_SR)*/
pub struct RNG_SRrs;
impl crate::RegisterSpec for RNG_SRrs {
    type Ux = u32;
}
///`read()` method returns [`rng_sr::R`](R) reader structure
impl crate::Readable for RNG_SRrs {}
///`write(|w| ..)` method takes [`rng_sr::W`](W) writer structure
impl crate::Writable for RNG_SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RNG_SR to value 0
impl crate::Resettable for RNG_SRrs {
    const RESET_VALUE: u32 = 0;
}
