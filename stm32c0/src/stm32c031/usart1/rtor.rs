///Register `RTOR` reader
pub type R = crate::R<RTORrs>;
///Register `RTOR` writer
pub type W = crate::W<RTORrs>;
///Field `RTO` reader - Receiver timeout value This bitfield gives the Receiver timeout value in terms of number of bits during which there is no activity on the RX line. In standard mode, the RTOF flag is set if, after the last received character, no new start bit is detected for more than the RTO value. In Smartcard mode, this value is used to implement the CWT and BWT. See Smartcard chapter for more details. In the standard, the CWT/BWT measurement is done starting from the start bit of the last received character. Note: This value must only be programmed once per received character.
pub type RTO_R = crate::FieldReader<u32>;
///Field `RTO` writer - Receiver timeout value This bitfield gives the Receiver timeout value in terms of number of bits during which there is no activity on the RX line. In standard mode, the RTOF flag is set if, after the last received character, no new start bit is detected for more than the RTO value. In Smartcard mode, this value is used to implement the CWT and BWT. See Smartcard chapter for more details. In the standard, the CWT/BWT measurement is done starting from the start bit of the last received character. Note: This value must only be programmed once per received character.
pub type RTO_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
///Field `BLEN` reader - Block Length This bitfield gives the Block length in Smartcard T = 1 Reception. Its value equals the number of information characters + the length of the Epilogue Field (1-LEC/2-CRC) - 1. Examples: BLEN = 0: 0 information characters + LEC BLEN = 1: 0 information characters + CRC BLEN = 255: 254 information characters + CRC (total 256 characters)) In Smartcard mode, the Block length counter is reset when TXE = 0 (TXFE = 0 in case FIFO mode is enabled). This bitfield can be used also in other modes. In this case, the Block length counter is reset when RE = 0 (receiver disabled) and/or when the EOBCF bit is written to 1. Note: This value can be programmed after the start of the block reception (using the data from the LEN character in the Prologue Field). It must be programmed only once per received block.
pub type BLEN_R = crate::FieldReader;
///Field `BLEN` writer - Block Length This bitfield gives the Block length in Smartcard T = 1 Reception. Its value equals the number of information characters + the length of the Epilogue Field (1-LEC/2-CRC) - 1. Examples: BLEN = 0: 0 information characters + LEC BLEN = 1: 0 information characters + CRC BLEN = 255: 254 information characters + CRC (total 256 characters)) In Smartcard mode, the Block length counter is reset when TXE = 0 (TXFE = 0 in case FIFO mode is enabled). This bitfield can be used also in other modes. In this case, the Block length counter is reset when RE = 0 (receiver disabled) and/or when the EOBCF bit is written to 1. Note: This value can be programmed after the start of the block reception (using the data from the LEN character in the Prologue Field). It must be programmed only once per received block.
pub type BLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:23 - Receiver timeout value This bitfield gives the Receiver timeout value in terms of number of bits during which there is no activity on the RX line. In standard mode, the RTOF flag is set if, after the last received character, no new start bit is detected for more than the RTO value. In Smartcard mode, this value is used to implement the CWT and BWT. See Smartcard chapter for more details. In the standard, the CWT/BWT measurement is done starting from the start bit of the last received character. Note: This value must only be programmed once per received character.
    #[inline(always)]
    pub fn rto(&self) -> RTO_R {
        RTO_R::new(self.bits & 0x00ff_ffff)
    }
    ///Bits 24:31 - Block Length This bitfield gives the Block length in Smartcard T = 1 Reception. Its value equals the number of information characters + the length of the Epilogue Field (1-LEC/2-CRC) - 1. Examples: BLEN = 0: 0 information characters + LEC BLEN = 1: 0 information characters + CRC BLEN = 255: 254 information characters + CRC (total 256 characters)) In Smartcard mode, the Block length counter is reset when TXE = 0 (TXFE = 0 in case FIFO mode is enabled). This bitfield can be used also in other modes. In this case, the Block length counter is reset when RE = 0 (receiver disabled) and/or when the EOBCF bit is written to 1. Note: This value can be programmed after the start of the block reception (using the data from the LEN character in the Prologue Field). It must be programmed only once per received block.
    #[inline(always)]
    pub fn blen(&self) -> BLEN_R {
        BLEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTOR")
            .field("rto", &self.rto())
            .field("blen", &self.blen())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - Receiver timeout value This bitfield gives the Receiver timeout value in terms of number of bits during which there is no activity on the RX line. In standard mode, the RTOF flag is set if, after the last received character, no new start bit is detected for more than the RTO value. In Smartcard mode, this value is used to implement the CWT and BWT. See Smartcard chapter for more details. In the standard, the CWT/BWT measurement is done starting from the start bit of the last received character. Note: This value must only be programmed once per received character.
    #[inline(always)]
    pub fn rto(&mut self) -> RTO_W<RTORrs> {
        RTO_W::new(self, 0)
    }
    ///Bits 24:31 - Block Length This bitfield gives the Block length in Smartcard T = 1 Reception. Its value equals the number of information characters + the length of the Epilogue Field (1-LEC/2-CRC) - 1. Examples: BLEN = 0: 0 information characters + LEC BLEN = 1: 0 information characters + CRC BLEN = 255: 254 information characters + CRC (total 256 characters)) In Smartcard mode, the Block length counter is reset when TXE = 0 (TXFE = 0 in case FIFO mode is enabled). This bitfield can be used also in other modes. In this case, the Block length counter is reset when RE = 0 (receiver disabled) and/or when the EOBCF bit is written to 1. Note: This value can be programmed after the start of the block reception (using the data from the LEN character in the Prologue Field). It must be programmed only once per received block.
    #[inline(always)]
    pub fn blen(&mut self) -> BLEN_W<RTORrs> {
        BLEN_W::new(self, 24)
    }
}
/**USART receiver timeout register

You can [`read`](crate::Reg::read) this register and get [`rtor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#USART1:RTOR)*/
pub struct RTORrs;
impl crate::RegisterSpec for RTORrs {
    type Ux = u32;
}
///`read()` method returns [`rtor::R`](R) reader structure
impl crate::Readable for RTORrs {}
///`write(|w| ..)` method takes [`rtor::W`](W) writer structure
impl crate::Writable for RTORrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RTOR to value 0
impl crate::Resettable for RTORrs {
    const RESET_VALUE: u32 = 0;
}
