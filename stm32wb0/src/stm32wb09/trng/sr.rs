///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `TRNG_DISABLED` reader - TRNG is disabled.
pub type TRNG_DISABLED_R = crate::BitReader;
///Field `ALL_OSCS_DOWN` reader - All oscillators of the random source noise have been powered down. This can cause the rising of OEC3 flag.
pub type ALL_OSCS_DOWN_R = crate::BitReader;
///Field `REVEAL_CLK_ERR` reader - The internal clock for the RNG core is not revealed.
pub type REVEAL_CLK_ERR_R = crate::BitReader;
///Field `ENTROPY_ERR` reader - The error refers to a fault in the bit sequence detected by the Entropy Monitor. Failed test is given by REPET_ERROR, and ADAPT_ERROR, OSCS_REPET_ERROR and OSCS_ADAPT_ERROR status flags.
pub type ENTROPY_ERR_R = crate::BitReader;
///Field `VAL_READY` reader - TRNG Value ready At least one 32-bit random value is available in the data FIFO. Note that application must ensure that a random is available in internal FIFO before starting a read otherwise a bus error will be generated.
pub type VAL_READY_R = crate::BitReader;
///Field `FIFO_FULL` reader - Indicates whether random data FIFO is full.
pub type FIFO_FULL_R = crate::BitReader;
///Field `SRC_HEALTH_DONE` reader - First run of noise source health test is completed
pub type SRC_HEALTH_DONE_R = crate::BitReader;
///Field `REPET_ERROR` reader - Noise source Repetition health test error
pub type REPET_ERROR_R = crate::BitReader;
///Field `ADAPT_ERROR` reader - Noise source Adaptive 1024 health test error
pub type ADAPT_ERROR_R = crate::BitReader;
///Field `OSCS_HEALTH_DONE` reader - First run of source health tests of individual oscillators composing the noise source are completed.Reserved
pub type OSCS_HEALTH_DONE_R = crate::BitReader;
///Field `OSCS_REPET_ERROR` reader - Logical OR of repetition health test errors of individual oscillators composing the noise source.
pub type OSCS_REPET_ERROR_R = crate::BitReader;
///Field `OSCS_ADAPT_ERROR` reader - Logical OR of adaptive health test errors of individual oscillators composing the noise source.
pub type OSCS_ADAPT_ERROR_R = crate::BitReader;
impl R {
    ///Bit 0 - TRNG is disabled.
    #[inline(always)]
    pub fn trng_disabled(&self) -> TRNG_DISABLED_R {
        TRNG_DISABLED_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - All oscillators of the random source noise have been powered down. This can cause the rising of OEC3 flag.
    #[inline(always)]
    pub fn all_oscs_down(&self) -> ALL_OSCS_DOWN_R {
        ALL_OSCS_DOWN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The internal clock for the RNG core is not revealed.
    #[inline(always)]
    pub fn reveal_clk_err(&self) -> REVEAL_CLK_ERR_R {
        REVEAL_CLK_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - The error refers to a fault in the bit sequence detected by the Entropy Monitor. Failed test is given by REPET_ERROR, and ADAPT_ERROR, OSCS_REPET_ERROR and OSCS_ADAPT_ERROR status flags.
    #[inline(always)]
    pub fn entropy_err(&self) -> ENTROPY_ERR_R {
        ENTROPY_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TRNG Value ready At least one 32-bit random value is available in the data FIFO. Note that application must ensure that a random is available in internal FIFO before starting a read otherwise a bus error will be generated.
    #[inline(always)]
    pub fn val_ready(&self) -> VAL_READY_R {
        VAL_READY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Indicates whether random data FIFO is full.
    #[inline(always)]
    pub fn fifo_full(&self) -> FIFO_FULL_R {
        FIFO_FULL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 20 - First run of noise source health test is completed
    #[inline(always)]
    pub fn src_health_done(&self) -> SRC_HEALTH_DONE_R {
        SRC_HEALTH_DONE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Noise source Repetition health test error
    #[inline(always)]
    pub fn repet_error(&self) -> REPET_ERROR_R {
        REPET_ERROR_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Noise source Adaptive 1024 health test error
    #[inline(always)]
    pub fn adapt_error(&self) -> ADAPT_ERROR_R {
        ADAPT_ERROR_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - First run of source health tests of individual oscillators composing the noise source are completed.Reserved
    #[inline(always)]
    pub fn oscs_health_done(&self) -> OSCS_HEALTH_DONE_R {
        OSCS_HEALTH_DONE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Logical OR of repetition health test errors of individual oscillators composing the noise source.
    #[inline(always)]
    pub fn oscs_repet_error(&self) -> OSCS_REPET_ERROR_R {
        OSCS_REPET_ERROR_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Logical OR of adaptive health test errors of individual oscillators composing the noise source.
    #[inline(always)]
    pub fn oscs_adapt_error(&self) -> OSCS_ADAPT_ERROR_R {
        OSCS_ADAPT_ERROR_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("trng_disabled", &self.trng_disabled())
            .field("all_oscs_down", &self.all_oscs_down())
            .field("reveal_clk_err", &self.reveal_clk_err())
            .field("entropy_err", &self.entropy_err())
            .field("val_ready", &self.val_ready())
            .field("fifo_full", &self.fifo_full())
            .field("src_health_done", &self.src_health_done())
            .field("repet_error", &self.repet_error())
            .field("adapt_error", &self.adapt_error())
            .field("oscs_health_done", &self.oscs_health_done())
            .field("oscs_repet_error", &self.oscs_repet_error())
            .field("oscs_adapt_error", &self.oscs_adapt_error())
            .finish()
    }
}
/**TRNG_SR register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
