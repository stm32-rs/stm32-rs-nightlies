///Register `FRAME_CONFIG0` reader
pub type R = crate::R<FRAME_CONFIG0rs>;
///Register `FRAME_CONFIG0` writer
pub type W = crate::W<FRAME_CONFIG0rs>;
///Field `PREAMBLE_THRESHOLD_COUNT` reader - The number of transitions for preamble detection when receiving the manchester encoded preamble.
pub type PREAMBLE_THRESHOLD_COUNT_R = crate::FieldReader;
///Field `PREAMBLE_THRESHOLD_COUNT` writer - The number of transitions for preamble detection when receiving the manchester encoded preamble.
pub type PREAMBLE_THRESHOLD_COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SYNC_LENGTH` reader - Frame sync pattern length ( Manchester encoded ).
pub type SYNC_LENGTH_R = crate::BitReader;
///Field `SYNC_LENGTH` writer - Frame sync pattern length ( Manchester encoded ).
pub type SYNC_LENGTH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNC_THRESHOLD_COUNT` reader - detection threshold when receivng the Frame sync ( Manchester encoded).
pub type SYNC_THRESHOLD_COUNT_R = crate::FieldReader;
///Field `SYNC_THRESHOLD_COUNT` writer - detection threshold when receivng the Frame sync ( Manchester encoded).
pub type SYNC_THRESHOLD_COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `PAYLOAD_LENGTH` reader - The number of data Bytes in the payload ( decoded ).
pub type PAYLOAD_LENGTH_R = crate::FieldReader;
///Field `PAYLOAD_LENGTH` writer - The number of data Bytes in the payload ( decoded ).
pub type PAYLOAD_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SLOW_CLK_CYCLE_PER_BIT_CNT` reader - The number of expected slow clock cycle per each manchester coded bit.
pub type SLOW_CLK_CYCLE_PER_BIT_CNT_R = crate::FieldReader;
///Field `SLOW_CLK_CYCLE_PER_BIT_CNT` writer - The number of expected slow clock cycle per each manchester coded bit.
pub type SLOW_CLK_CYCLE_PER_BIT_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:7 - The number of transitions for preamble detection when receiving the manchester encoded preamble.
    #[inline(always)]
    pub fn preamble_threshold_count(&self) -> PREAMBLE_THRESHOLD_COUNT_R {
        PREAMBLE_THRESHOLD_COUNT_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - Frame sync pattern length ( Manchester encoded ).
    #[inline(always)]
    pub fn sync_length(&self) -> SYNC_LENGTH_R {
        SYNC_LENGTH_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 10:15 - detection threshold when receivng the Frame sync ( Manchester encoded).
    #[inline(always)]
    pub fn sync_threshold_count(&self) -> SYNC_THRESHOLD_COUNT_R {
        SYNC_THRESHOLD_COUNT_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    ///Bits 16:19 - The number of data Bytes in the payload ( decoded ).
    #[inline(always)]
    pub fn payload_length(&self) -> PAYLOAD_LENGTH_R {
        PAYLOAD_LENGTH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 21:25 - The number of expected slow clock cycle per each manchester coded bit.
    #[inline(always)]
    pub fn slow_clk_cycle_per_bit_cnt(&self) -> SLOW_CLK_CYCLE_PER_BIT_CNT_R {
        SLOW_CLK_CYCLE_PER_BIT_CNT_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRAME_CONFIG0")
            .field("preamble_threshold_count", &self.preamble_threshold_count())
            .field("sync_length", &self.sync_length())
            .field("sync_threshold_count", &self.sync_threshold_count())
            .field("payload_length", &self.payload_length())
            .field(
                "slow_clk_cycle_per_bit_cnt",
                &self.slow_clk_cycle_per_bit_cnt(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:7 - The number of transitions for preamble detection when receiving the manchester encoded preamble.
    #[inline(always)]
    pub fn preamble_threshold_count(&mut self) -> PREAMBLE_THRESHOLD_COUNT_W<'_, FRAME_CONFIG0rs> {
        PREAMBLE_THRESHOLD_COUNT_W::new(self, 0)
    }
    ///Bit 8 - Frame sync pattern length ( Manchester encoded ).
    #[inline(always)]
    pub fn sync_length(&mut self) -> SYNC_LENGTH_W<'_, FRAME_CONFIG0rs> {
        SYNC_LENGTH_W::new(self, 8)
    }
    ///Bits 10:15 - detection threshold when receivng the Frame sync ( Manchester encoded).
    #[inline(always)]
    pub fn sync_threshold_count(&mut self) -> SYNC_THRESHOLD_COUNT_W<'_, FRAME_CONFIG0rs> {
        SYNC_THRESHOLD_COUNT_W::new(self, 10)
    }
    ///Bits 16:19 - The number of data Bytes in the payload ( decoded ).
    #[inline(always)]
    pub fn payload_length(&mut self) -> PAYLOAD_LENGTH_W<'_, FRAME_CONFIG0rs> {
        PAYLOAD_LENGTH_W::new(self, 16)
    }
    ///Bits 21:25 - The number of expected slow clock cycle per each manchester coded bit.
    #[inline(always)]
    pub fn slow_clk_cycle_per_bit_cnt(
        &mut self,
    ) -> SLOW_CLK_CYCLE_PER_BIT_CNT_W<'_, FRAME_CONFIG0rs> {
        SLOW_CLK_CYCLE_PER_BIT_CNT_W::new(self, 21)
    }
}
/**FRAME_CONFIG0 register

You can [`read`](crate::Reg::read) this register and get [`frame_config0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_config0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LPAWUR:FRAME_CONFIG0)*/
pub struct FRAME_CONFIG0rs;
impl crate::RegisterSpec for FRAME_CONFIG0rs {
    type Ux = u32;
}
///`read()` method returns [`frame_config0::R`](R) reader structure
impl crate::Readable for FRAME_CONFIG0rs {}
///`write(|w| ..)` method takes [`frame_config0::W`](W) writer structure
impl crate::Writable for FRAME_CONFIG0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FRAME_CONFIG0 to value 0x0207_4012
impl crate::Resettable for FRAME_CONFIG0rs {
    const RESET_VALUE: u32 = 0x0207_4012;
}
