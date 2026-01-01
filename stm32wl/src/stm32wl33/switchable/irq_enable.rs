///Register `IRQ_ENABLE` reader
pub type R = crate::R<IRQ_ENABLErs>;
///Register `IRQ_ENABLE` writer
pub type W = crate::W<IRQ_ENABLErs>;
///Field `BIT_SYNC_DETECTED_E` reader - Preamble has been detected, the content of the PAYLOAD_X registers is not yet valid.
pub type BIT_SYNC_DETECTED_E_R = crate::BitReader;
///Field `BIT_SYNC_DETECTED_E` writer - Preamble has been detected, the content of the PAYLOAD_X registers is not yet valid.
pub type BIT_SYNC_DETECTED_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRAME_SYNC_COMPLETE_E` reader - Frame Sync has been detected, the content of the PAYLOAD_X registers is not yet valid.
pub type FRAME_SYNC_COMPLETE_E_R = crate::BitReader;
///Field `FRAME_SYNC_COMPLETE_E` writer - Frame Sync has been detected, the content of the PAYLOAD_X registers is not yet valid.
pub type FRAME_SYNC_COMPLETE_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRAME_COMPLETE_E` reader - Frame ( payload + CRC) received, the content of the PAYLOAD_X registers is valid.
pub type FRAME_COMPLETE_E_R = crate::BitReader;
///Field `FRAME_COMPLETE_E` writer - Frame ( payload + CRC) received, the content of the PAYLOAD_X registers is valid.
pub type FRAME_COMPLETE_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRAME_VALID_E` reader - Frame ( payload + CRC) received wthout error (the CRC has been checked and is matching with the received CRC).
pub type FRAME_VALID_E_R = crate::BitReader;
///Field `FRAME_VALID_E` writer - Frame ( payload + CRC) received wthout error (the CRC has been checked and is matching with the received CRC).
pub type FRAME_VALID_E_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Preamble has been detected, the content of the PAYLOAD_X registers is not yet valid.
    #[inline(always)]
    pub fn bit_sync_detected_e(&self) -> BIT_SYNC_DETECTED_E_R {
        BIT_SYNC_DETECTED_E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Frame Sync has been detected, the content of the PAYLOAD_X registers is not yet valid.
    #[inline(always)]
    pub fn frame_sync_complete_e(&self) -> FRAME_SYNC_COMPLETE_E_R {
        FRAME_SYNC_COMPLETE_E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Frame ( payload + CRC) received, the content of the PAYLOAD_X registers is valid.
    #[inline(always)]
    pub fn frame_complete_e(&self) -> FRAME_COMPLETE_E_R {
        FRAME_COMPLETE_E_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Frame ( payload + CRC) received wthout error (the CRC has been checked and is matching with the received CRC).
    #[inline(always)]
    pub fn frame_valid_e(&self) -> FRAME_VALID_E_R {
        FRAME_VALID_E_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQ_ENABLE")
            .field("bit_sync_detected_e", &self.bit_sync_detected_e())
            .field("frame_sync_complete_e", &self.frame_sync_complete_e())
            .field("frame_complete_e", &self.frame_complete_e())
            .field("frame_valid_e", &self.frame_valid_e())
            .finish()
    }
}
impl W {
    ///Bit 0 - Preamble has been detected, the content of the PAYLOAD_X registers is not yet valid.
    #[inline(always)]
    pub fn bit_sync_detected_e(&mut self) -> BIT_SYNC_DETECTED_E_W<'_, IRQ_ENABLErs> {
        BIT_SYNC_DETECTED_E_W::new(self, 0)
    }
    ///Bit 1 - Frame Sync has been detected, the content of the PAYLOAD_X registers is not yet valid.
    #[inline(always)]
    pub fn frame_sync_complete_e(&mut self) -> FRAME_SYNC_COMPLETE_E_W<'_, IRQ_ENABLErs> {
        FRAME_SYNC_COMPLETE_E_W::new(self, 1)
    }
    ///Bit 2 - Frame ( payload + CRC) received, the content of the PAYLOAD_X registers is valid.
    #[inline(always)]
    pub fn frame_complete_e(&mut self) -> FRAME_COMPLETE_E_W<'_, IRQ_ENABLErs> {
        FRAME_COMPLETE_E_W::new(self, 2)
    }
    ///Bit 3 - Frame ( payload + CRC) received wthout error (the CRC has been checked and is matching with the received CRC).
    #[inline(always)]
    pub fn frame_valid_e(&mut self) -> FRAME_VALID_E_W<'_, IRQ_ENABLErs> {
        FRAME_VALID_E_W::new(self, 3)
    }
}
/**IRQ_ENABLE register

You can [`read`](crate::Reg::read) this register and get [`irq_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SWITCHABLE:IRQ_ENABLE)*/
pub struct IRQ_ENABLErs;
impl crate::RegisterSpec for IRQ_ENABLErs {
    type Ux = u32;
}
///`read()` method returns [`irq_enable::R`](R) reader structure
impl crate::Readable for IRQ_ENABLErs {}
///`write(|w| ..)` method takes [`irq_enable::W`](W) writer structure
impl crate::Writable for IRQ_ENABLErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IRQ_ENABLE to value 0
impl crate::Resettable for IRQ_ENABLErs {}
