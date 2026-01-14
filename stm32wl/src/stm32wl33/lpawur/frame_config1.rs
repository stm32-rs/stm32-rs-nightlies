///Register `FRAME_CONFIG1` reader
pub type R = crate::R<FRAME_CONFIG1rs>;
///Register `FRAME_CONFIG1` writer
pub type W = crate::W<FRAME_CONFIG1rs>;
///Field `KI` reader - ki gain value for the timing recovery loop.
pub type KI_R = crate::FieldReader;
///Field `KI` writer - ki gain value for the timing recovery loop.
pub type KI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `KP` reader - kp gain value for the timing recovery loop.
pub type KP_R = crate::FieldReader;
///Field `KP` writer - kp gain value for the timing recovery loop.
pub type KP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FRAME_SYNC_COUNTER_TIMEOUT` reader - The timeout in manchester encoded bits for the Frame Sync,it represents the number of samples after which in case the frame sync is not detected a sync_error is raised.
pub type FRAME_SYNC_COUNTER_TIMEOUT_R = crate::FieldReader;
///Field `FRAME_SYNC_COUNTER_TIMEOUT` writer - The timeout in manchester encoded bits for the Frame Sync,it represents the number of samples after which in case the frame sync is not detected a sync_error is raised.
pub type FRAME_SYNC_COUNTER_TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PREAMBLE_ENABLE` reader - Preamble detection enable
pub type PREAMBLE_ENABLE_R = crate::BitReader;
///Field `PREAMBLE_ENABLE` writer - Preamble detection enable
pub type PREAMBLE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TREC_LOOP_ALGO_SEL` reader - Timing recovery loop algorithm selection:
pub type TREC_LOOP_ALGO_SEL_R = crate::BitReader;
///Field `TREC_LOOP_ALGO_SEL` writer - Timing recovery loop algorithm selection:
pub type TREC_LOOP_ALGO_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - ki gain value for the timing recovery loop.
    #[inline(always)]
    pub fn ki(&self) -> KI_R {
        KI_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - kp gain value for the timing recovery loop.
    #[inline(always)]
    pub fn kp(&self) -> KP_R {
        KP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:15 - The timeout in manchester encoded bits for the Frame Sync,it represents the number of samples after which in case the frame sync is not detected a sync_error is raised.
    #[inline(always)]
    pub fn frame_sync_counter_timeout(&self) -> FRAME_SYNC_COUNTER_TIMEOUT_R {
        FRAME_SYNC_COUNTER_TIMEOUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 17 - Preamble detection enable
    #[inline(always)]
    pub fn preamble_enable(&self) -> PREAMBLE_ENABLE_R {
        PREAMBLE_ENABLE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Timing recovery loop algorithm selection:
    #[inline(always)]
    pub fn trec_loop_algo_sel(&self) -> TREC_LOOP_ALGO_SEL_R {
        TREC_LOOP_ALGO_SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRAME_CONFIG1")
            .field("ki", &self.ki())
            .field("kp", &self.kp())
            .field(
                "frame_sync_counter_timeout",
                &self.frame_sync_counter_timeout(),
            )
            .field("preamble_enable", &self.preamble_enable())
            .field("trec_loop_algo_sel", &self.trec_loop_algo_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - ki gain value for the timing recovery loop.
    #[inline(always)]
    pub fn ki(&mut self) -> KI_W<'_, FRAME_CONFIG1rs> {
        KI_W::new(self, 0)
    }
    ///Bits 4:7 - kp gain value for the timing recovery loop.
    #[inline(always)]
    pub fn kp(&mut self) -> KP_W<'_, FRAME_CONFIG1rs> {
        KP_W::new(self, 4)
    }
    ///Bits 8:15 - The timeout in manchester encoded bits for the Frame Sync,it represents the number of samples after which in case the frame sync is not detected a sync_error is raised.
    #[inline(always)]
    pub fn frame_sync_counter_timeout(
        &mut self,
    ) -> FRAME_SYNC_COUNTER_TIMEOUT_W<'_, FRAME_CONFIG1rs> {
        FRAME_SYNC_COUNTER_TIMEOUT_W::new(self, 8)
    }
    ///Bit 17 - Preamble detection enable
    #[inline(always)]
    pub fn preamble_enable(&mut self) -> PREAMBLE_ENABLE_W<'_, FRAME_CONFIG1rs> {
        PREAMBLE_ENABLE_W::new(self, 17)
    }
    ///Bit 18 - Timing recovery loop algorithm selection:
    #[inline(always)]
    pub fn trec_loop_algo_sel(&mut self) -> TREC_LOOP_ALGO_SEL_W<'_, FRAME_CONFIG1rs> {
        TREC_LOOP_ALGO_SEL_W::new(self, 18)
    }
}
/**FRAME_CONFIG1 register

You can [`read`](crate::Reg::read) this register and get [`frame_config1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_config1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LPAWUR:FRAME_CONFIG1)*/
pub struct FRAME_CONFIG1rs;
impl crate::RegisterSpec for FRAME_CONFIG1rs {
    type Ux = u32;
}
///`read()` method returns [`frame_config1::R`](R) reader structure
impl crate::Readable for FRAME_CONFIG1rs {}
///`write(|w| ..)` method takes [`frame_config1::W`](W) writer structure
impl crate::Writable for FRAME_CONFIG1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FRAME_CONFIG1 to value 0x0002_4669
impl crate::Resettable for FRAME_CONFIG1rs {
    const RESET_VALUE: u32 = 0x0002_4669;
}
