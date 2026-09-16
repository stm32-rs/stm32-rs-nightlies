///Register `STATUS` reader
pub type R = crate::R<STATUSrs>;
///Register `STATUS` writer
pub type W = crate::W<STATUSrs>;
///Field `BIT_SYNC_DETECTED_F` reader - Preamble has been detected, the content of the PAYLOAD_X registers is not yet valid.
pub type BIT_SYNC_DETECTED_F_R = crate::BitReader;
///Field `BIT_SYNC_DETECTED_F` writer - Preamble has been detected, the content of the PAYLOAD_X registers is not yet valid.
pub type BIT_SYNC_DETECTED_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRAME_SYNC_COMPLETE_F` reader - Frame Sync has been detected, the content of the PAYLOAD_X registers is not yet valid.
pub type FRAME_SYNC_COMPLETE_F_R = crate::BitReader;
///Field `FRAME_SYNC_COMPLETE_F` writer - Frame Sync has been detected, the content of the PAYLOAD_X registers is not yet valid.
pub type FRAME_SYNC_COMPLETE_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRAME_COMPLETE_F` reader - Frame ( payload + CRC) received, the content of the PAYLOAD_X registers is valid.
pub type FRAME_COMPLETE_F_R = crate::BitReader;
///Field `FRAME_COMPLETE_F` writer - Frame ( payload + CRC) received, the content of the PAYLOAD_X registers is valid.
pub type FRAME_COMPLETE_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRAME_VALID_F` reader - Frame ( payload + CRC) received wthout error (the CRC has been checked and is matching with the received CRC).
pub type FRAME_VALID_F_R = crate::BitReader;
///Field `FRAME_VALID_F` writer - Frame ( payload + CRC) received wthout error (the CRC has been checked and is matching with the received CRC).
pub type FRAME_VALID_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERROR_F` reader - - 11 : CRC error
pub type ERROR_F_R = crate::FieldReader;
///Field `ERROR_F` writer - - 11 : CRC error
pub type ERROR_F_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Preamble has been detected, the content of the PAYLOAD_X registers is not yet valid.
    #[inline(always)]
    pub fn bit_sync_detected_f(&self) -> BIT_SYNC_DETECTED_F_R {
        BIT_SYNC_DETECTED_F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Frame Sync has been detected, the content of the PAYLOAD_X registers is not yet valid.
    #[inline(always)]
    pub fn frame_sync_complete_f(&self) -> FRAME_SYNC_COMPLETE_F_R {
        FRAME_SYNC_COMPLETE_F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Frame ( payload + CRC) received, the content of the PAYLOAD_X registers is valid.
    #[inline(always)]
    pub fn frame_complete_f(&self) -> FRAME_COMPLETE_F_R {
        FRAME_COMPLETE_F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Frame ( payload + CRC) received wthout error (the CRC has been checked and is matching with the received CRC).
    #[inline(always)]
    pub fn frame_valid_f(&self) -> FRAME_VALID_F_R {
        FRAME_VALID_F_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 30:31 - - 11 : CRC error
    #[inline(always)]
    pub fn error_f(&self) -> ERROR_F_R {
        ERROR_F_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("bit_sync_detected_f", &self.bit_sync_detected_f())
            .field("frame_sync_complete_f", &self.frame_sync_complete_f())
            .field("frame_complete_f", &self.frame_complete_f())
            .field("frame_valid_f", &self.frame_valid_f())
            .field("error_f", &self.error_f())
            .finish()
    }
}
impl W {
    ///Bit 0 - Preamble has been detected, the content of the PAYLOAD_X registers is not yet valid.
    #[inline(always)]
    pub fn bit_sync_detected_f(&mut self) -> BIT_SYNC_DETECTED_F_W<'_, STATUSrs> {
        BIT_SYNC_DETECTED_F_W::new(self, 0)
    }
    ///Bit 1 - Frame Sync has been detected, the content of the PAYLOAD_X registers is not yet valid.
    #[inline(always)]
    pub fn frame_sync_complete_f(&mut self) -> FRAME_SYNC_COMPLETE_F_W<'_, STATUSrs> {
        FRAME_SYNC_COMPLETE_F_W::new(self, 1)
    }
    ///Bit 2 - Frame ( payload + CRC) received, the content of the PAYLOAD_X registers is valid.
    #[inline(always)]
    pub fn frame_complete_f(&mut self) -> FRAME_COMPLETE_F_W<'_, STATUSrs> {
        FRAME_COMPLETE_F_W::new(self, 2)
    }
    ///Bit 3 - Frame ( payload + CRC) received wthout error (the CRC has been checked and is matching with the received CRC).
    #[inline(always)]
    pub fn frame_valid_f(&mut self) -> FRAME_VALID_F_W<'_, STATUSrs> {
        FRAME_VALID_F_W::new(self, 3)
    }
    ///Bits 30:31 - - 11 : CRC error
    #[inline(always)]
    pub fn error_f(&mut self) -> ERROR_F_W<'_, STATUSrs> {
        ERROR_F_W::new(self, 30)
    }
}
/**STATUS register

You can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SWITCHABLE:STATUS)*/
pub struct STATUSrs;
impl crate::RegisterSpec for STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`status::R`](R) reader structure
impl crate::Readable for STATUSrs {}
///`write(|w| ..)` method takes [`status::W`](W) writer structure
impl crate::Writable for STATUSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STATUS to value 0
impl crate::Resettable for STATUSrs {}
