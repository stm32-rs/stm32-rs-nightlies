///Register `HEALTH_OSC2_SR` reader
pub type R = crate::R<HEALTH_OSC2_SRrs>;
///Field `TO1_REPET_ERROR` reader - Repetition error flag of first oscillator of first triple-oscillator cell.
pub type TO1_REPET_ERROR_R = crate::BitReader;
///Field `TO1_ADAPT_ERROR` reader - Adaptive error flag of first oscillator of first triple-oscillator cell.
pub type TO1_ADAPT_ERROR_R = crate::BitReader;
///Field `TO2_REPET_ERROR` reader - Repetition error flag of first oscillator of second triple-oscillator cell.
pub type TO2_REPET_ERROR_R = crate::BitReader;
///Field `TO2_ADAPT_ERROR` reader - Adaptive error flag of first oscillator of second triple-oscillator cell.
pub type TO2_ADAPT_ERROR_R = crate::BitReader;
///Field `TO3_REPET_ERROR` reader - Repetition error flag of first oscillator of third triple-oscillator cell.
pub type TO3_REPET_ERROR_R = crate::BitReader;
///Field `TO3_ADAPT_ERROR` reader - Adaptive error flag of first oscillator of third triple-oscillator cell.
pub type TO3_ADAPT_ERROR_R = crate::BitReader;
impl R {
    ///Bit 0 - Repetition error flag of first oscillator of first triple-oscillator cell.
    #[inline(always)]
    pub fn to1_repet_error(&self) -> TO1_REPET_ERROR_R {
        TO1_REPET_ERROR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Adaptive error flag of first oscillator of first triple-oscillator cell.
    #[inline(always)]
    pub fn to1_adapt_error(&self) -> TO1_ADAPT_ERROR_R {
        TO1_ADAPT_ERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Repetition error flag of first oscillator of second triple-oscillator cell.
    #[inline(always)]
    pub fn to2_repet_error(&self) -> TO2_REPET_ERROR_R {
        TO2_REPET_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Adaptive error flag of first oscillator of second triple-oscillator cell.
    #[inline(always)]
    pub fn to2_adapt_error(&self) -> TO2_ADAPT_ERROR_R {
        TO2_ADAPT_ERROR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Repetition error flag of first oscillator of third triple-oscillator cell.
    #[inline(always)]
    pub fn to3_repet_error(&self) -> TO3_REPET_ERROR_R {
        TO3_REPET_ERROR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Adaptive error flag of first oscillator of third triple-oscillator cell.
    #[inline(always)]
    pub fn to3_adapt_error(&self) -> TO3_ADAPT_ERROR_R {
        TO3_ADAPT_ERROR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HEALTH_OSC2_SR")
            .field("to1_repet_error", &self.to1_repet_error())
            .field("to1_adapt_error", &self.to1_adapt_error())
            .field("to2_repet_error", &self.to2_repet_error())
            .field("to2_adapt_error", &self.to2_adapt_error())
            .field("to3_repet_error", &self.to3_repet_error())
            .field("to3_adapt_error", &self.to3_adapt_error())
            .finish()
    }
}
/**TRNG_HEALTH_OSC2_SR register

You can [`read`](crate::Reg::read) this register and get [`health_osc2_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:HEALTH_OSC2_SR)*/
pub struct HEALTH_OSC2_SRrs;
impl crate::RegisterSpec for HEALTH_OSC2_SRrs {
    type Ux = u32;
}
///`read()` method returns [`health_osc2_sr::R`](R) reader structure
impl crate::Readable for HEALTH_OSC2_SRrs {}
///`reset()` method sets HEALTH_OSC2_SR to value 0
impl crate::Resettable for HEALTH_OSC2_SRrs {}
