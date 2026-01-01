///Register `PCKT_CONFIG` reader
pub type R = crate::R<PCKT_CONFIGrs>;
///Register `PCKT_CONFIG` writer
pub type W = crate::W<PCKT_CONFIGrs>;
///Field `CRC_MODE` reader - CRC type (0, 8, 16, 16 802.
pub type CRC_MODE_R = crate::FieldReader;
///Field `CRC_MODE` writer - CRC type (0, 8, 16, 16 802.
pub type CRC_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SECONDARY_SYNC_SEL` reader - In TX mode: this bit selects which synchro word is sent on the frame between SYNC and SEC_SYNC
pub type SECONDARY_SYNC_SEL_R = crate::BitReader;
///Field `SECONDARY_SYNC_SEL` writer - In TX mode: this bit selects which synchro word is sent on the frame between SYNC and SEC_SYNC
pub type SECONDARY_SYNC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNC_LEN` reader - Length of the SYNC (and secondary) SYNC word in 1-bit granularity
pub type SYNC_LEN_R = crate::FieldReader;
///Field `SYNC_LEN` writer - Length of the SYNC (and secondary) SYNC word in 1-bit granularity
pub type SYNC_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SYNC_PRESENT` reader - Indicate if a SYNC word is present on the frame or not (null length)
pub type SYNC_PRESENT_R = crate::BitReader;
///Field `SYNC_PRESENT` writer - Indicate if a SYNC word is present on the frame or not (null length)
pub type SYNC_PRESENT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LEN_WIDTH` reader - Indicates if the LENGTH field is defined on 1 byte or 2 bytes
pub type LEN_WIDTH_R = crate::BitReader;
///Field `LEN_WIDTH` writer - Indicates if the LENGTH field is defined on 1 byte or 2 bytes
pub type LEN_WIDTH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIX_VAR_LEN` reader - Select the length mode
pub type FIX_VAR_LEN_R = crate::BitReader;
///Field `FIX_VAR_LEN` writer - Select the length mode
pub type FIX_VAR_LEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PREAMBLE_LENGTH` reader - Length of the PREAMBLE in pairs of bits (0 to 2046)
pub type PREAMBLE_LENGTH_R = crate::FieldReader<u16>;
///Field `PREAMBLE_LENGTH` writer - Length of the PREAMBLE in pairs of bits (0 to 2046)
pub type PREAMBLE_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `PREAMBLE_SEQ` reader - Select the PREAMBLE pattern to be applied
pub type PREAMBLE_SEQ_R = crate::FieldReader;
///Field `PREAMBLE_SEQ` writer - Select the PREAMBLE pattern to be applied
pub type PREAMBLE_SEQ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `POSTAMBLE_LENGTH` reader - Length of the POSTAMBLE in pair of bits (0 to 126 bits)
pub type POSTAMBLE_LENGTH_R = crate::FieldReader;
///Field `POSTAMBLE_LENGTH` writer - Length of the POSTAMBLE in pair of bits (0 to 126 bits)
pub type POSTAMBLE_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `POSTAMBLE_SEQ` reader - Packet postamble control: postamble bit sequence selection
pub type POSTAMBLE_SEQ_R = crate::FieldReader;
///Field `POSTAMBLE_SEQ` writer - Packet postamble control: postamble bit sequence selection
pub type POSTAMBLE_SEQ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - CRC type (0, 8, 16, 16 802.
    #[inline(always)]
    pub fn crc_mode(&self) -> CRC_MODE_R {
        CRC_MODE_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - In TX mode: this bit selects which synchro word is sent on the frame between SYNC and SEC_SYNC
    #[inline(always)]
    pub fn secondary_sync_sel(&self) -> SECONDARY_SYNC_SEL_R {
        SECONDARY_SYNC_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:8 - Length of the SYNC (and secondary) SYNC word in 1-bit granularity
    #[inline(always)]
    pub fn sync_len(&self) -> SYNC_LEN_R {
        SYNC_LEN_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    ///Bit 9 - Indicate if a SYNC word is present on the frame or not (null length)
    #[inline(always)]
    pub fn sync_present(&self) -> SYNC_PRESENT_R {
        SYNC_PRESENT_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Indicates if the LENGTH field is defined on 1 byte or 2 bytes
    #[inline(always)]
    pub fn len_width(&self) -> LEN_WIDTH_R {
        LEN_WIDTH_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Select the length mode
    #[inline(always)]
    pub fn fix_var_len(&self) -> FIX_VAR_LEN_R {
        FIX_VAR_LEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:21 - Length of the PREAMBLE in pairs of bits (0 to 2046)
    #[inline(always)]
    pub fn preamble_length(&self) -> PREAMBLE_LENGTH_R {
        PREAMBLE_LENGTH_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    ///Bits 22:23 - Select the PREAMBLE pattern to be applied
    #[inline(always)]
    pub fn preamble_seq(&self) -> PREAMBLE_SEQ_R {
        PREAMBLE_SEQ_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:29 - Length of the POSTAMBLE in pair of bits (0 to 126 bits)
    #[inline(always)]
    pub fn postamble_length(&self) -> POSTAMBLE_LENGTH_R {
        POSTAMBLE_LENGTH_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    ///Bits 30:31 - Packet postamble control: postamble bit sequence selection
    #[inline(always)]
    pub fn postamble_seq(&self) -> POSTAMBLE_SEQ_R {
        POSTAMBLE_SEQ_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCKT_CONFIG")
            .field("crc_mode", &self.crc_mode())
            .field("secondary_sync_sel", &self.secondary_sync_sel())
            .field("sync_len", &self.sync_len())
            .field("sync_present", &self.sync_present())
            .field("len_width", &self.len_width())
            .field("fix_var_len", &self.fix_var_len())
            .field("preamble_length", &self.preamble_length())
            .field("preamble_seq", &self.preamble_seq())
            .field("postamble_length", &self.postamble_length())
            .field("postamble_seq", &self.postamble_seq())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - CRC type (0, 8, 16, 16 802.
    #[inline(always)]
    pub fn crc_mode(&mut self) -> CRC_MODE_W<'_, PCKT_CONFIGrs> {
        CRC_MODE_W::new(self, 0)
    }
    ///Bit 3 - In TX mode: this bit selects which synchro word is sent on the frame between SYNC and SEC_SYNC
    #[inline(always)]
    pub fn secondary_sync_sel(&mut self) -> SECONDARY_SYNC_SEL_W<'_, PCKT_CONFIGrs> {
        SECONDARY_SYNC_SEL_W::new(self, 3)
    }
    ///Bits 4:8 - Length of the SYNC (and secondary) SYNC word in 1-bit granularity
    #[inline(always)]
    pub fn sync_len(&mut self) -> SYNC_LEN_W<'_, PCKT_CONFIGrs> {
        SYNC_LEN_W::new(self, 4)
    }
    ///Bit 9 - Indicate if a SYNC word is present on the frame or not (null length)
    #[inline(always)]
    pub fn sync_present(&mut self) -> SYNC_PRESENT_W<'_, PCKT_CONFIGrs> {
        SYNC_PRESENT_W::new(self, 9)
    }
    ///Bit 10 - Indicates if the LENGTH field is defined on 1 byte or 2 bytes
    #[inline(always)]
    pub fn len_width(&mut self) -> LEN_WIDTH_W<'_, PCKT_CONFIGrs> {
        LEN_WIDTH_W::new(self, 10)
    }
    ///Bit 11 - Select the length mode
    #[inline(always)]
    pub fn fix_var_len(&mut self) -> FIX_VAR_LEN_W<'_, PCKT_CONFIGrs> {
        FIX_VAR_LEN_W::new(self, 11)
    }
    ///Bits 12:21 - Length of the PREAMBLE in pairs of bits (0 to 2046)
    #[inline(always)]
    pub fn preamble_length(&mut self) -> PREAMBLE_LENGTH_W<'_, PCKT_CONFIGrs> {
        PREAMBLE_LENGTH_W::new(self, 12)
    }
    ///Bits 22:23 - Select the PREAMBLE pattern to be applied
    #[inline(always)]
    pub fn preamble_seq(&mut self) -> PREAMBLE_SEQ_W<'_, PCKT_CONFIGrs> {
        PREAMBLE_SEQ_W::new(self, 22)
    }
    ///Bits 24:29 - Length of the POSTAMBLE in pair of bits (0 to 126 bits)
    #[inline(always)]
    pub fn postamble_length(&mut self) -> POSTAMBLE_LENGTH_W<'_, PCKT_CONFIGrs> {
        POSTAMBLE_LENGTH_W::new(self, 24)
    }
    ///Bits 30:31 - Packet postamble control: postamble bit sequence selection
    #[inline(always)]
    pub fn postamble_seq(&mut self) -> POSTAMBLE_SEQ_W<'_, PCKT_CONFIGrs> {
        POSTAMBLE_SEQ_W::new(self, 30)
    }
}
/**PCKT_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`pckt_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pckt_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:PCKT_CONFIG)*/
pub struct PCKT_CONFIGrs;
impl crate::RegisterSpec for PCKT_CONFIGrs {
    type Ux = u32;
}
///`read()` method returns [`pckt_config::R`](R) reader structure
impl crate::Readable for PCKT_CONFIGrs {}
///`write(|w| ..)` method takes [`pckt_config::W`](W) writer structure
impl crate::Writable for PCKT_CONFIGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCKT_CONFIG to value 0x0001_03f1
impl crate::Resettable for PCKT_CONFIGrs {
    const RESET_VALUE: u32 = 0x0001_03f1;
}
