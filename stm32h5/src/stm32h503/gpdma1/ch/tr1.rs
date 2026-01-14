///Register `TR1` reader
pub type R = crate::R<TR1rs>;
///Register `TR1` writer
pub type W = crate::W<TR1rs>;
/**binary logarithm of the source data width of a burst in bytes Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and no transfer is issued. Note: A source burst transfer must have an aligned address with its data width (start address GPDMA_CxSAR\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and none transfer is issued.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDW_LOG2R {
    ///0: Byte
    Byte = 0,
    ///1: Half-word (2 bytes)
    HalfWord = 1,
    ///2: Word (4 bytes)
    Word = 2,
    ///3: User setting error
    Error = 3,
}
impl From<SDW_LOG2R> for u8 {
    #[inline(always)]
    fn from(variant: SDW_LOG2R) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SDW_LOG2R {
    type Ux = u8;
}
impl crate::IsEnum for SDW_LOG2R {}
///Field `SDW_LOG2` reader - binary logarithm of the source data width of a burst in bytes Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and no transfer is issued. Note: A source burst transfer must have an aligned address with its data width (start address GPDMA_CxSAR\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and none transfer is issued.
pub type SDW_LOG2_R = crate::FieldReader<SDW_LOG2R>;
impl SDW_LOG2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDW_LOG2R {
        match self.bits {
            0 => SDW_LOG2R::Byte,
            1 => SDW_LOG2R::HalfWord,
            2 => SDW_LOG2R::Word,
            3 => SDW_LOG2R::Error,
            _ => unreachable!(),
        }
    }
    ///Byte
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == SDW_LOG2R::Byte
    }
    ///Half-word (2 bytes)
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == SDW_LOG2R::HalfWord
    }
    ///Word (4 bytes)
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == SDW_LOG2R::Word
    }
    ///User setting error
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == SDW_LOG2R::Error
    }
}
/**binary logarithm of the source data width of a burst in bytes Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and no transfer is issued. Note: A source burst transfer must have an aligned address with its data width (start address GPDMA_CxSAR\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and none transfer is issued.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDW_LOG2W {
    ///0: Byte
    Byte = 0,
    ///1: Half-word (2 bytes)
    HalfWord = 1,
    ///2: Word (4 bytes)
    Word = 2,
}
impl From<SDW_LOG2W> for u8 {
    #[inline(always)]
    fn from(variant: SDW_LOG2W) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SDW_LOG2W {
    type Ux = u8;
}
impl crate::IsEnum for SDW_LOG2W {}
///Field `SDW_LOG2` writer - binary logarithm of the source data width of a burst in bytes Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and no transfer is issued. Note: A source burst transfer must have an aligned address with its data width (start address GPDMA_CxSAR\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and none transfer is issued.
pub type SDW_LOG2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SDW_LOG2W>;
impl<'a, REG> SDW_LOG2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Byte
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(SDW_LOG2W::Byte)
    }
    ///Half-word (2 bytes)
    #[inline(always)]
    pub fn half_word(self) -> &'a mut crate::W<REG> {
        self.variant(SDW_LOG2W::HalfWord)
    }
    ///Word (4 bytes)
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(SDW_LOG2W::Word)
    }
}
/**source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SINC {
    ///0: Fixed burst
    FixedBurst = 0,
    ///1: Contiguously incremented burst
    Contiguous = 1,
}
impl From<SINC> for bool {
    #[inline(always)]
    fn from(variant: SINC) -> Self {
        variant as u8 != 0
    }
}
///Field `SINC` reader - source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
pub type SINC_R = crate::BitReader<SINC>;
impl SINC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SINC {
        match self.bits {
            false => SINC::FixedBurst,
            true => SINC::Contiguous,
        }
    }
    ///Fixed burst
    #[inline(always)]
    pub fn is_fixed_burst(&self) -> bool {
        *self == SINC::FixedBurst
    }
    ///Contiguously incremented burst
    #[inline(always)]
    pub fn is_contiguous(&self) -> bool {
        *self == SINC::Contiguous
    }
}
///Field `SINC` writer - source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
pub type SINC_W<'a, REG> = crate::BitWriter<'a, REG, SINC>;
impl<'a, REG> SINC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Fixed burst
    #[inline(always)]
    pub fn fixed_burst(self) -> &'a mut crate::W<REG> {
        self.variant(SINC::FixedBurst)
    }
    ///Contiguously incremented burst
    #[inline(always)]
    pub fn contiguous(self) -> &'a mut crate::W<REG> {
        self.variant(SINC::Contiguous)
    }
}
///Field `SBL_1` reader - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed.
pub type SBL_1_R = crate::FieldReader;
///Field `SBL_1` writer - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed.
pub type SBL_1_W<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
///Field `PAM` reader - padding/alignment mode If DDW_LOG2\[1:0\] = SDW_LOG2\[1:0\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else, in the following enumerated values, the condition PAM_1 is when destination data width is higher that source data width, and the condition PAM_2 is when destination data width is higher than source data width. 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination Note: If the transfer from the source peripheral is configured with peripheral flow-control mode (SWREQ = 0 and PFREQ = 1 and DREQ = 0), and if the destination data width the source data width, packing is not supported.
pub type PAM_R = crate::FieldReader;
///Field `PAM` writer - padding/alignment mode If DDW_LOG2\[1:0\] = SDW_LOG2\[1:0\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else, in the following enumerated values, the condition PAM_1 is when destination data width is higher that source data width, and the condition PAM_2 is when destination data width is higher than source data width. 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination Note: If the transfer from the source peripheral is configured with peripheral flow-control mode (SWREQ = 0 and PFREQ = 1 and DREQ = 0), and if the destination data width the source data width, packing is not supported.
pub type PAM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
/**source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBX {
    ///0: No byte-based exchanged within word
    NotExchanged = 0,
    ///1: The two consecutive (post PAM) bytes are exchanged in each destination half-word
    Exchanged = 1,
}
impl From<SBX> for bool {
    #[inline(always)]
    fn from(variant: SBX) -> Self {
        variant as u8 != 0
    }
}
///Field `SBX` reader - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:
pub type SBX_R = crate::BitReader<SBX>;
impl SBX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SBX {
        match self.bits {
            false => SBX::NotExchanged,
            true => SBX::Exchanged,
        }
    }
    ///No byte-based exchanged within word
    #[inline(always)]
    pub fn is_not_exchanged(&self) -> bool {
        *self == SBX::NotExchanged
    }
    ///The two consecutive (post PAM) bytes are exchanged in each destination half-word
    #[inline(always)]
    pub fn is_exchanged(&self) -> bool {
        *self == SBX::Exchanged
    }
}
///Field `SBX` writer - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:
pub type SBX_W<'a, REG> = crate::BitWriter<'a, REG, SBX>;
impl<'a, REG> SBX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No byte-based exchanged within word
    #[inline(always)]
    pub fn not_exchanged(self) -> &'a mut crate::W<REG> {
        self.variant(SBX::NotExchanged)
    }
    ///The two consecutive (post PAM) bytes are exchanged in each destination half-word
    #[inline(always)]
    pub fn exchanged(self) -> &'a mut crate::W<REG> {
        self.variant(SBX::Exchanged)
    }
}
/**source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAP {
    ///0: Port 0 (AHB) allocated
    Port0 = 0,
    ///1: Port 1 (AHB) allocated
    Port1 = 1,
}
impl From<SAP> for bool {
    #[inline(always)]
    fn from(variant: SAP) -> Self {
        variant as u8 != 0
    }
}
///Field `SAP` reader - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
pub type SAP_R = crate::BitReader<SAP>;
impl SAP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SAP {
        match self.bits {
            false => SAP::Port0,
            true => SAP::Port1,
        }
    }
    ///Port 0 (AHB) allocated
    #[inline(always)]
    pub fn is_port0(&self) -> bool {
        *self == SAP::Port0
    }
    ///Port 1 (AHB) allocated
    #[inline(always)]
    pub fn is_port1(&self) -> bool {
        *self == SAP::Port1
    }
}
///Field `SAP` writer - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
pub type SAP_W<'a, REG> = crate::BitWriter<'a, REG, SAP>;
impl<'a, REG> SAP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port 0 (AHB) allocated
    #[inline(always)]
    pub fn port0(self) -> &'a mut crate::W<REG> {
        self.variant(SAP::Port0)
    }
    ///Port 1 (AHB) allocated
    #[inline(always)]
    pub fn port1(self) -> &'a mut crate::W<REG> {
        self.variant(SAP::Port1)
    }
}
///Field `DDW_LOG2` reader - binary logarithm of the destination data width of a burst, in bytes Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. Note: A destination burst transfer must have an aligned address with its data width (start address GPDMA_CxDAR\[2:0\] and address offset GPDMA_CxTR3.DAO\[2:0\], versus DDW_LOG2\[1:0\]). Otherwise a user setting error is reported and no transfer is issued.
pub use SDW_LOG2_R as DDW_LOG2_R;
///Field `DDW_LOG2` writer - binary logarithm of the destination data width of a burst, in bytes Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. Note: A destination burst transfer must have an aligned address with its data width (start address GPDMA_CxDAR\[2:0\] and address offset GPDMA_CxTR3.DAO\[2:0\], versus DDW_LOG2\[1:0\]). Otherwise a user setting error is reported and no transfer is issued.
pub use SDW_LOG2_W as DDW_LOG2_W;
///Field `DINC` reader - destination incrementing burst The destination address, pointed by GPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
pub use SINC_R as DINC_R;
///Field `DINC` writer - destination incrementing burst The destination address, pointed by GPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
pub use SINC_W as DINC_W;
///Field `DBL_1` reader - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed.
pub type DBL_1_R = crate::FieldReader;
///Field `DBL_1` writer - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed.
pub type DBL_1_W<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
///Field `DBX` reader - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:
pub use SBX_R as DBX_R;
///Field `DBX` writer - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:
pub use SBX_W as DBX_W;
/**destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DHX {
    ///0: No halfword-based exchange within word
    NotExchanged = 0,
    ///1: The two consecutive (post PAM) half-words are exchanged in each destination word
    Exchanged = 1,
}
impl From<DHX> for bool {
    #[inline(always)]
    fn from(variant: DHX) -> Self {
        variant as u8 != 0
    }
}
///Field `DHX` reader - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:
pub type DHX_R = crate::BitReader<DHX>;
impl DHX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DHX {
        match self.bits {
            false => DHX::NotExchanged,
            true => DHX::Exchanged,
        }
    }
    ///No halfword-based exchange within word
    #[inline(always)]
    pub fn is_not_exchanged(&self) -> bool {
        *self == DHX::NotExchanged
    }
    ///The two consecutive (post PAM) half-words are exchanged in each destination word
    #[inline(always)]
    pub fn is_exchanged(&self) -> bool {
        *self == DHX::Exchanged
    }
}
///Field `DHX` writer - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:
pub type DHX_W<'a, REG> = crate::BitWriter<'a, REG, DHX>;
impl<'a, REG> DHX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No halfword-based exchange within word
    #[inline(always)]
    pub fn not_exchanged(self) -> &'a mut crate::W<REG> {
        self.variant(DHX::NotExchanged)
    }
    ///The two consecutive (post PAM) half-words are exchanged in each destination word
    #[inline(always)]
    pub fn exchanged(self) -> &'a mut crate::W<REG> {
        self.variant(DHX::Exchanged)
    }
}
///Field `DAP` reader - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
pub use SAP_R as DAP_R;
///Field `DAP` writer - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
pub use SAP_W as DAP_W;
impl R {
    ///Bits 0:1 - binary logarithm of the source data width of a burst in bytes Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and no transfer is issued. Note: A source burst transfer must have an aligned address with its data width (start address GPDMA_CxSAR\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and none transfer is issued.
    #[inline(always)]
    pub fn sdw_log2(&self) -> SDW_LOG2_R {
        SDW_LOG2_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
    #[inline(always)]
    pub fn sinc(&self) -> SINC_R {
        SINC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:9 - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed.
    #[inline(always)]
    pub fn sbl_1(&self) -> SBL_1_R {
        SBL_1_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    ///Bits 11:12 - padding/alignment mode If DDW_LOG2\[1:0\] = SDW_LOG2\[1:0\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else, in the following enumerated values, the condition PAM_1 is when destination data width is higher that source data width, and the condition PAM_2 is when destination data width is higher than source data width. 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination Note: If the transfer from the source peripheral is configured with peripheral flow-control mode (SWREQ = 0 and PFREQ = 1 and DREQ = 0), and if the destination data width the source data width, packing is not supported.
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 13 - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:
    #[inline(always)]
    pub fn sbx(&self) -> SBX_R {
        SBX_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
    #[inline(always)]
    pub fn sap(&self) -> SAP_R {
        SAP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:17 - binary logarithm of the destination data width of a burst, in bytes Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. Note: A destination burst transfer must have an aligned address with its data width (start address GPDMA_CxDAR\[2:0\] and address offset GPDMA_CxTR3.DAO\[2:0\], versus DDW_LOG2\[1:0\]). Otherwise a user setting error is reported and no transfer is issued.
    #[inline(always)]
    pub fn ddw_log2(&self) -> DDW_LOG2_R {
        DDW_LOG2_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 19 - destination incrementing burst The destination address, pointed by GPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
    #[inline(always)]
    pub fn dinc(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:25 - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed.
    #[inline(always)]
    pub fn dbl_1(&self) -> DBL_1_R {
        DBL_1_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    ///Bit 26 - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:
    #[inline(always)]
    pub fn dbx(&self) -> DBX_R {
        DBX_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:
    #[inline(always)]
    pub fn dhx(&self) -> DHX_R {
        DHX_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 30 - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
    #[inline(always)]
    pub fn dap(&self) -> DAP_R {
        DAP_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TR1")
            .field("sdw_log2", &self.sdw_log2())
            .field("sinc", &self.sinc())
            .field("sbl_1", &self.sbl_1())
            .field("pam", &self.pam())
            .field("sbx", &self.sbx())
            .field("sap", &self.sap())
            .field("ddw_log2", &self.ddw_log2())
            .field("dinc", &self.dinc())
            .field("dbl_1", &self.dbl_1())
            .field("dbx", &self.dbx())
            .field("dhx", &self.dhx())
            .field("dap", &self.dap())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - binary logarithm of the source data width of a burst in bytes Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and no transfer is issued. Note: A source burst transfer must have an aligned address with its data width (start address GPDMA_CxSAR\[2:0\] versus SDW_LOG2\[1:0\]). Otherwise, a user setting error is reported and none transfer is issued.
    #[inline(always)]
    pub fn sdw_log2(&mut self) -> SDW_LOG2_W<'_, TR1rs> {
        SDW_LOG2_W::new(self, 0)
    }
    ///Bit 3 - source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
    #[inline(always)]
    pub fn sinc(&mut self) -> SINC_W<'_, TR1rs> {
        SINC_W::new(self, 3)
    }
    ///Bits 4:9 - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed.
    #[inline(always)]
    pub fn sbl_1(&mut self) -> SBL_1_W<'_, TR1rs> {
        SBL_1_W::new(self, 4)
    }
    ///Bits 11:12 - padding/alignment mode If DDW_LOG2\[1:0\] = SDW_LOG2\[1:0\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else, in the following enumerated values, the condition PAM_1 is when destination data width is higher that source data width, and the condition PAM_2 is when destination data width is higher than source data width. 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination Note: If the transfer from the source peripheral is configured with peripheral flow-control mode (SWREQ = 0 and PFREQ = 1 and DREQ = 0), and if the destination data width the source data width, packing is not supported.
    #[inline(always)]
    pub fn pam(&mut self) -> PAM_W<'_, TR1rs> {
        PAM_W::new(self, 11)
    }
    ///Bit 13 - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:
    #[inline(always)]
    pub fn sbx(&mut self) -> SBX_W<'_, TR1rs> {
        SBX_W::new(self, 13)
    }
    ///Bit 14 - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
    #[inline(always)]
    pub fn sap(&mut self) -> SAP_W<'_, TR1rs> {
        SAP_W::new(self, 14)
    }
    ///Bits 16:17 - binary logarithm of the destination data width of a burst, in bytes Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. Note: A destination burst transfer must have an aligned address with its data width (start address GPDMA_CxDAR\[2:0\] and address offset GPDMA_CxTR3.DAO\[2:0\], versus DDW_LOG2\[1:0\]). Otherwise a user setting error is reported and no transfer is issued.
    #[inline(always)]
    pub fn ddw_log2(&mut self) -> DDW_LOG2_W<'_, TR1rs> {
        DDW_LOG2_W::new(self, 16)
    }
    ///Bit 19 - destination incrementing burst The destination address, pointed by GPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.
    #[inline(always)]
    pub fn dinc(&mut self) -> DINC_W<'_, TR1rs> {
        DINC_W::new(self, 19)
    }
    ///Bits 20:25 - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\[5:0\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\[1:0\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed.
    #[inline(always)]
    pub fn dbl_1(&mut self) -> DBL_1_W<'_, TR1rs> {
        DBL_1_W::new(self, 20)
    }
    ///Bit 26 - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:
    #[inline(always)]
    pub fn dbx(&mut self) -> DBX_W<'_, TR1rs> {
        DBX_W::new(self, 26)
    }
    ///Bit 27 - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:
    #[inline(always)]
    pub fn dhx(&mut self) -> DHX_W<'_, TR1rs> {
        DHX_W::new(self, 27)
    }
    ///Bit 30 - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.
    #[inline(always)]
    pub fn dap(&mut self) -> DAP_W<'_, TR1rs> {
        DAP_W::new(self, 30)
    }
}
/**GPDMA channel 0 transfer register 1

You can [`read`](crate::Reg::read) this register and get [`tr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TR1rs;
impl crate::RegisterSpec for TR1rs {
    type Ux = u32;
}
///`read()` method returns [`tr1::R`](R) reader structure
impl crate::Readable for TR1rs {}
///`write(|w| ..)` method takes [`tr1::W`](W) writer structure
impl crate::Writable for TR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TR1 to value 0
impl crate::Resettable for TR1rs {}
