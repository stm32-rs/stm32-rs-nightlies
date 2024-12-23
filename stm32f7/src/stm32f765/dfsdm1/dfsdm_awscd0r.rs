///Register `DFSDM_AWSCD0R` reader
pub type R = crate::R<DFSDM_AWSCD0Rrs>;
///Register `DFSDM_AWSCD0R` writer
pub type W = crate::W<DFSDM_AWSCD0Rrs>;
///Field `SCDT` reader - short-circuit detector threshold for channel 0
pub type SCDT_R = crate::FieldReader;
///Field `SCDT` writer - short-circuit detector threshold for channel 0
pub type SCDT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BKSCD` reader - Break signal assignment for short-circuit detector on channel 0
pub type BKSCD_R = crate::FieldReader;
///Field `BKSCD` writer - Break signal assignment for short-circuit detector on channel 0
pub type BKSCD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AWFOSR` reader - Analog watchdog filter oversampling ratio (decimation rate) on channel 0
pub type AWFOSR_R = crate::FieldReader;
///Field `AWFOSR` writer - Analog watchdog filter oversampling ratio (decimation rate) on channel 0
pub type AWFOSR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `AWFORD` reader - Analog watchdog Sinc filter order on channel 0
pub type AWFORD_R = crate::FieldReader;
///Field `AWFORD` writer - Analog watchdog Sinc filter order on channel 0
pub type AWFORD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:7 - short-circuit detector threshold for channel 0
    #[inline(always)]
    pub fn scdt(&self) -> SCDT_R {
        SCDT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 12:15 - Break signal assignment for short-circuit detector on channel 0
    #[inline(always)]
    pub fn bkscd(&self) -> BKSCD_R {
        BKSCD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:20 - Analog watchdog filter oversampling ratio (decimation rate) on channel 0
    #[inline(always)]
    pub fn awfosr(&self) -> AWFOSR_R {
        AWFOSR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 22:23 - Analog watchdog Sinc filter order on channel 0
    #[inline(always)]
    pub fn awford(&self) -> AWFORD_R {
        AWFORD_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFSDM_AWSCD0R")
            .field("scdt", &self.scdt())
            .field("bkscd", &self.bkscd())
            .field("awfosr", &self.awfosr())
            .field("awford", &self.awford())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - short-circuit detector threshold for channel 0
    #[inline(always)]
    pub fn scdt(&mut self) -> SCDT_W<DFSDM_AWSCD0Rrs> {
        SCDT_W::new(self, 0)
    }
    ///Bits 12:15 - Break signal assignment for short-circuit detector on channel 0
    #[inline(always)]
    pub fn bkscd(&mut self) -> BKSCD_W<DFSDM_AWSCD0Rrs> {
        BKSCD_W::new(self, 12)
    }
    ///Bits 16:20 - Analog watchdog filter oversampling ratio (decimation rate) on channel 0
    #[inline(always)]
    pub fn awfosr(&mut self) -> AWFOSR_W<DFSDM_AWSCD0Rrs> {
        AWFOSR_W::new(self, 16)
    }
    ///Bits 22:23 - Analog watchdog Sinc filter order on channel 0
    #[inline(always)]
    pub fn awford(&mut self) -> AWFORD_W<DFSDM_AWSCD0Rrs> {
        AWFORD_W::new(self, 22)
    }
}
/**DFSDM analog watchdog and short-circuit detector register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_awscd0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_awscd0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F765.html#DFSDM1:DFSDM_AWSCD0R)*/
pub struct DFSDM_AWSCD0Rrs;
impl crate::RegisterSpec for DFSDM_AWSCD0Rrs {
    type Ux = u32;
}
///`read()` method returns [`dfsdm_awscd0r::R`](R) reader structure
impl crate::Readable for DFSDM_AWSCD0Rrs {}
///`write(|w| ..)` method takes [`dfsdm_awscd0r::W`](W) writer structure
impl crate::Writable for DFSDM_AWSCD0Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DFSDM_AWSCD0R to value 0
impl crate::Resettable for DFSDM_AWSCD0Rrs {
    const RESET_VALUE: u32 = 0;
}
