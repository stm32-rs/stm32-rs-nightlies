#[doc = "Register `C1TR1` reader"]
pub type R = crate::R<C1TR1rs>;
#[doc = "Register `C1TR1` writer"]
pub type W = crate::W<C1TR1rs>;
#[doc = "binary logarithm of the source data width of a burst in bytes Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and no transfer is issued. Note: A source burst transfer must have an aligned address with its data width (start address GPDMA_CxSAR\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and none transfer is issued.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDW_LOG2R {
    #[doc = "0: Byte"]
    Byte = 0,
    #[doc = "1: Half-word (2 bytes)"]
    HalfWord = 1,
    #[doc = "2: Word (4 bytes)"]
    Word = 2,
    #[doc = "3: User setting error"]
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
#[doc = "Field `SDW_LOG2` reader - binary logarithm of the source data width of a burst in bytes Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and no transfer is issued. Note: A source burst transfer must have an aligned address with its data width (start address GPDMA_CxSAR\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and none transfer is issued."]
pub type SDW_LOG2_R = crate::FieldReader<SDW_LOG2R>;
impl SDW_LOG2_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Byte"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == SDW_LOG2R::Byte
    }
    #[doc = "Half-word (2 bytes)"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == SDW_LOG2R::HalfWord
    }
    #[doc = "Word (4 bytes)"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == SDW_LOG2R::Word
    }
    #[doc = "User setting error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == SDW_LOG2R::Error
    }
}
#[doc = "binary logarithm of the source data width of a burst in bytes Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and no transfer is issued. Note: A source burst transfer must have an aligned address with its data width (start address GPDMA_CxSAR\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and none transfer is issued.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDW_LOG2W {
    #[doc = "0: Byte"]
    Byte = 0,
    #[doc = "1: Half-word (2 bytes)"]
    HalfWord = 1,
    #[doc = "2: Word (4 bytes)"]
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
#[doc = "Field `SDW_LOG2` writer - binary logarithm of the source data width of a burst in bytes Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and no transfer is issued. Note: A source burst transfer must have an aligned address with its data width (start address GPDMA_CxSAR\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and none transfer is issued."]
pub type SDW_LOG2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SDW_LOG2W>;
impl<'a, REG> SDW_LOG2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Byte"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(SDW_LOG2W::Byte)
    }
    #[doc = "Half-word (2 bytes)"]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut crate::W<REG> {
        self.variant(SDW_LOG2W::HalfWord)
    }
    #[doc = "Word (4 bytes)"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(SDW_LOG2W::Word)
    }
}
#[doc = "source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SINC {
    #[doc = "0: Fixed burst"]
    FixedBurst = 0,
    #[doc = "1: Contiguously incremented burst"]
    Contiguous = 1,
}
impl From<SINC> for bool {
    #[inline(always)]
    fn from(variant: SINC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SINC` reader - source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
pub type SINC_R = crate::BitReader<SINC>;
impl SINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SINC {
        match self.bits {
            false => SINC::FixedBurst,
            true => SINC::Contiguous,
        }
    }
    #[doc = "Fixed burst"]
    #[inline(always)]
    pub fn is_fixed_burst(&self) -> bool {
        *self == SINC::FixedBurst
    }
    #[doc = "Contiguously incremented burst"]
    #[inline(always)]
    pub fn is_contiguous(&self) -> bool {
        *self == SINC::Contiguous
    }
}
#[doc = "Field `SINC` writer - source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
pub type SINC_W<'a, REG> = crate::BitWriter<'a, REG, SINC>;
impl<'a, REG> SINC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fixed burst"]
    #[inline(always)]
    pub fn fixed_burst(self) -> &'a mut crate::W<REG> {
        self.variant(SINC::FixedBurst)
    }
    #[doc = "Contiguously incremented burst"]
    #[inline(always)]
    pub fn contiguous(self) -> &'a mut crate::W<REG> {
        self.variant(SINC::Contiguous)
    }
}
#[doc = "Field `SBL_1` reader - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\\[5:0\\]
=0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
pub type SBL_1_R = crate::FieldReader;
#[doc = "Field `SBL_1` writer - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\\[5:0\\]
=0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
pub type SBL_1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
#[doc = "Field `PAM` reader - padding/alignment mode If DDW_LOG2\\[1:0\\]
= SDW_LOG2\\[1:0\\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else, in the following enumerated values, the condition PAM_1 is when destination data width is higher that source data width, and the condition PAM_2 is when destination data width is higher than source data width. 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination Note: If the transfer from the source peripheral is configured with peripheral flow-control mode (SWREQ = 0 and PFREQ = 1 and DREQ = 0), and if the destination data width > the source data width, packing is not supported."]
pub type PAM_R = crate::FieldReader;
#[doc = "Field `PAM` writer - padding/alignment mode If DDW_LOG2\\[1:0\\]
= SDW_LOG2\\[1:0\\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else, in the following enumerated values, the condition PAM_1 is when destination data width is higher that source data width, and the condition PAM_2 is when destination data width is higher than source data width. 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination Note: If the transfer from the source peripheral is configured with peripheral flow-control mode (SWREQ = 0 and PFREQ = 1 and DREQ = 0), and if the destination data width > the source data width, packing is not supported."]
pub type PAM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "PAM value when destination data width is higher than source data width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PAM_1 {
    #[doc = "0: Source data is transferred as right aligned, padded with 0s up to the destination data width"]
    RightAlignedZeroPadded = 0,
    #[doc = "1: Source data is transferred as right aligned, sign extended up to the destination data width"]
    RightAlignedSignExtended = 1,
    #[doc = "2: Source data are FIFO queued and packed at the destination data width, in little endian order, before a destination transfer"]
    Fifo = 2,
}
impl From<PAM_1> for u8 {
    #[inline(always)]
    fn from(variant: PAM_1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PAM_1 {
    type Ux = u8;
}
#[doc = "Field `PAM_1` reader - PAM value when destination data width is higher than source data width"]
pub type PAM_1_R = crate::FieldReader<PAM_1>;
impl PAM_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PAM_1> {
        match self.bits {
            0 => Some(PAM_1::RightAlignedZeroPadded),
            1 => Some(PAM_1::RightAlignedSignExtended),
            2 => Some(PAM_1::Fifo),
            _ => None,
        }
    }
    #[doc = "Source data is transferred as right aligned, padded with 0s up to the destination data width"]
    #[inline(always)]
    pub fn is_right_aligned_zero_padded(&self) -> bool {
        *self == PAM_1::RightAlignedZeroPadded
    }
    #[doc = "Source data is transferred as right aligned, sign extended up to the destination data width"]
    #[inline(always)]
    pub fn is_right_aligned_sign_extended(&self) -> bool {
        *self == PAM_1::RightAlignedSignExtended
    }
    #[doc = "Source data are FIFO queued and packed at the destination data width, in little endian order, before a destination transfer"]
    #[inline(always)]
    pub fn is_fifo(&self) -> bool {
        *self == PAM_1::Fifo
    }
}
#[doc = "Field `PAM_1` writer - PAM value when destination data width is higher than source data width"]
pub type PAM_1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PAM_1>;
impl<'a, REG> PAM_1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Source data is transferred as right aligned, padded with 0s up to the destination data width"]
    #[inline(always)]
    pub fn right_aligned_zero_padded(self) -> &'a mut crate::W<REG> {
        self.variant(PAM_1::RightAlignedZeroPadded)
    }
    #[doc = "Source data is transferred as right aligned, sign extended up to the destination data width"]
    #[inline(always)]
    pub fn right_aligned_sign_extended(self) -> &'a mut crate::W<REG> {
        self.variant(PAM_1::RightAlignedSignExtended)
    }
    #[doc = "Source data are FIFO queued and packed at the destination data width, in little endian order, before a destination transfer"]
    #[inline(always)]
    pub fn fifo(self) -> &'a mut crate::W<REG> {
        self.variant(PAM_1::Fifo)
    }
}
#[doc = "PAM value when source data width is higher than destination data width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PAM_2 {
    #[doc = "0: Source data is transferred as right aligned, left-truncated down to the destination data width"]
    RightAlignedLeftTruncated = 0,
    #[doc = "1: Source data is transferred as left-aligned, right-truncated down to the destination data width"]
    LeftAlignedRightTruncated = 1,
    #[doc = "2: Source data are FIFO queued and unpacked at the destination data width, in little endian order"]
    Fifo = 2,
}
impl From<PAM_2> for u8 {
    #[inline(always)]
    fn from(variant: PAM_2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PAM_2 {
    type Ux = u8;
}
#[doc = "Field `PAM_2` reader - PAM value when source data width is higher than destination data width"]
pub type PAM_2_R = crate::FieldReader<PAM_2>;
impl PAM_2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PAM_2> {
        match self.bits {
            0 => Some(PAM_2::RightAlignedLeftTruncated),
            1 => Some(PAM_2::LeftAlignedRightTruncated),
            2 => Some(PAM_2::Fifo),
            _ => None,
        }
    }
    #[doc = "Source data is transferred as right aligned, left-truncated down to the destination data width"]
    #[inline(always)]
    pub fn is_right_aligned_left_truncated(&self) -> bool {
        *self == PAM_2::RightAlignedLeftTruncated
    }
    #[doc = "Source data is transferred as left-aligned, right-truncated down to the destination data width"]
    #[inline(always)]
    pub fn is_left_aligned_right_truncated(&self) -> bool {
        *self == PAM_2::LeftAlignedRightTruncated
    }
    #[doc = "Source data are FIFO queued and unpacked at the destination data width, in little endian order"]
    #[inline(always)]
    pub fn is_fifo(&self) -> bool {
        *self == PAM_2::Fifo
    }
}
#[doc = "Field `PAM_2` writer - PAM value when source data width is higher than destination data width"]
pub type PAM_2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PAM_2>;
impl<'a, REG> PAM_2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Source data is transferred as right aligned, left-truncated down to the destination data width"]
    #[inline(always)]
    pub fn right_aligned_left_truncated(self) -> &'a mut crate::W<REG> {
        self.variant(PAM_2::RightAlignedLeftTruncated)
    }
    #[doc = "Source data is transferred as left-aligned, right-truncated down to the destination data width"]
    #[inline(always)]
    pub fn left_aligned_right_truncated(self) -> &'a mut crate::W<REG> {
        self.variant(PAM_2::LeftAlignedRightTruncated)
    }
    #[doc = "Source data are FIFO queued and unpacked at the destination data width, in little endian order"]
    #[inline(always)]
    pub fn fifo(self) -> &'a mut crate::W<REG> {
        self.variant(PAM_2::Fifo)
    }
}
#[doc = "source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBX {
    #[doc = "0: No byte-based exchanged within word"]
    NotExchanged = 0,
    #[doc = "1: The two consecutive (post PAM) bytes are exchanged in each destination half-word"]
    Exchanged = 1,
}
impl From<SBX> for bool {
    #[inline(always)]
    fn from(variant: SBX) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBX` reader - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:"]
pub type SBX_R = crate::BitReader<SBX>;
impl SBX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SBX {
        match self.bits {
            false => SBX::NotExchanged,
            true => SBX::Exchanged,
        }
    }
    #[doc = "No byte-based exchanged within word"]
    #[inline(always)]
    pub fn is_not_exchanged(&self) -> bool {
        *self == SBX::NotExchanged
    }
    #[doc = "The two consecutive (post PAM) bytes are exchanged in each destination half-word"]
    #[inline(always)]
    pub fn is_exchanged(&self) -> bool {
        *self == SBX::Exchanged
    }
}
#[doc = "Field `SBX` writer - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:"]
pub type SBX_W<'a, REG> = crate::BitWriter<'a, REG, SBX>;
impl<'a, REG> SBX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No byte-based exchanged within word"]
    #[inline(always)]
    pub fn not_exchanged(self) -> &'a mut crate::W<REG> {
        self.variant(SBX::NotExchanged)
    }
    #[doc = "The two consecutive (post PAM) bytes are exchanged in each destination half-word"]
    #[inline(always)]
    pub fn exchanged(self) -> &'a mut crate::W<REG> {
        self.variant(SBX::Exchanged)
    }
}
#[doc = "source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAP {
    #[doc = "0: Port 0 (AHB) allocated"]
    Port0 = 0,
    #[doc = "1: Port 1 (AHB) allocated"]
    Port1 = 1,
}
impl From<SAP> for bool {
    #[inline(always)]
    fn from(variant: SAP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAP` reader - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
pub type SAP_R = crate::BitReader<SAP>;
impl SAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SAP {
        match self.bits {
            false => SAP::Port0,
            true => SAP::Port1,
        }
    }
    #[doc = "Port 0 (AHB) allocated"]
    #[inline(always)]
    pub fn is_port0(&self) -> bool {
        *self == SAP::Port0
    }
    #[doc = "Port 1 (AHB) allocated"]
    #[inline(always)]
    pub fn is_port1(&self) -> bool {
        *self == SAP::Port1
    }
}
#[doc = "Field `SAP` writer - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
pub type SAP_W<'a, REG> = crate::BitWriter<'a, REG, SAP>;
impl<'a, REG> SAP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port 0 (AHB) allocated"]
    #[inline(always)]
    pub fn port0(self) -> &'a mut crate::W<REG> {
        self.variant(SAP::Port0)
    }
    #[doc = "Port 1 (AHB) allocated"]
    #[inline(always)]
    pub fn port1(self) -> &'a mut crate::W<REG> {
        self.variant(SAP::Port1)
    }
}
#[doc = "Field `DDW_LOG2` reader - binary logarithm of the destination data width of a burst, in bytes Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. Note: A destination burst transfer must have an aligned address with its data width (start address GPDMA_CxDAR\\[2:0\\]
and address offset GPDMA_CxTR3.DAO\\[2:0\\], versus DDW_LOG2\\[1:0\\]). Otherwise a user setting error is reported and no transfer is issued."]
pub use SDW_LOG2_R as DDW_LOG2_R;
#[doc = "Field `DDW_LOG2` writer - binary logarithm of the destination data width of a burst, in bytes Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. Note: A destination burst transfer must have an aligned address with its data width (start address GPDMA_CxDAR\\[2:0\\]
and address offset GPDMA_CxTR3.DAO\\[2:0\\], versus DDW_LOG2\\[1:0\\]). Otherwise a user setting error is reported and no transfer is issued."]
pub use SDW_LOG2_W as DDW_LOG2_W;
#[doc = "Field `DINC` reader - destination incrementing burst The destination address, pointed by GPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
pub use SINC_R as DINC_R;
#[doc = "Field `DINC` writer - destination incrementing burst The destination address, pointed by GPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
pub use SINC_W as DINC_W;
#[doc = "Field `DBL_1` reader - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\\[5:0\\]
=0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
pub type DBL_1_R = crate::FieldReader;
#[doc = "Field `DBL_1` writer - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\\[5:0\\]
=0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
pub type DBL_1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
#[doc = "Field `DBX` reader - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:"]
pub use SBX_R as DBX_R;
#[doc = "Field `DBX` writer - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:"]
pub use SBX_W as DBX_W;
#[doc = "destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DHX {
    #[doc = "0: No halfword-based exchange within word"]
    NotExchanged = 0,
    #[doc = "1: The two consecutive (post PAM) half-words are exchanged in each destination word"]
    Exchanged = 1,
}
impl From<DHX> for bool {
    #[inline(always)]
    fn from(variant: DHX) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DHX` reader - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:"]
pub type DHX_R = crate::BitReader<DHX>;
impl DHX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DHX {
        match self.bits {
            false => DHX::NotExchanged,
            true => DHX::Exchanged,
        }
    }
    #[doc = "No halfword-based exchange within word"]
    #[inline(always)]
    pub fn is_not_exchanged(&self) -> bool {
        *self == DHX::NotExchanged
    }
    #[doc = "The two consecutive (post PAM) half-words are exchanged in each destination word"]
    #[inline(always)]
    pub fn is_exchanged(&self) -> bool {
        *self == DHX::Exchanged
    }
}
#[doc = "Field `DHX` writer - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:"]
pub type DHX_W<'a, REG> = crate::BitWriter<'a, REG, DHX>;
impl<'a, REG> DHX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No halfword-based exchange within word"]
    #[inline(always)]
    pub fn not_exchanged(self) -> &'a mut crate::W<REG> {
        self.variant(DHX::NotExchanged)
    }
    #[doc = "The two consecutive (post PAM) half-words are exchanged in each destination word"]
    #[inline(always)]
    pub fn exchanged(self) -> &'a mut crate::W<REG> {
        self.variant(DHX::Exchanged)
    }
}
#[doc = "Field `DAP` reader - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
pub use SAP_R as DAP_R;
#[doc = "Field `DAP` writer - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
pub use SAP_W as DAP_W;
impl R {
    #[doc = "Bits 0:1 - binary logarithm of the source data width of a burst in bytes Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and no transfer is issued. Note: A source burst transfer must have an aligned address with its data width (start address GPDMA_CxSAR\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and none transfer is issued."]
    #[inline(always)]
    pub fn sdw_log2(&self) -> SDW_LOG2_R {
        SDW_LOG2_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
    #[inline(always)]
    pub fn sinc(&self) -> SINC_R {
        SINC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:9 - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\\[5:0\\]
=0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
    #[inline(always)]
    pub fn sbl_1(&self) -> SBL_1_R {
        SBL_1_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 11:12 - padding/alignment mode If DDW_LOG2\\[1:0\\]
= SDW_LOG2\\[1:0\\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else, in the following enumerated values, the condition PAM_1 is when destination data width is higher that source data width, and the condition PAM_2 is when destination data width is higher than source data width. 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination Note: If the transfer from the source peripheral is configured with peripheral flow-control mode (SWREQ = 0 and PFREQ = 1 and DREQ = 0), and if the destination data width > the source data width, packing is not supported."]
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 11:12 - PAM value when destination data width is higher than source data width"]
    #[inline(always)]
    pub fn pam_1(&self) -> PAM_1_R {
        PAM_1_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 11:12 - PAM value when source data width is higher than destination data width"]
    #[inline(always)]
    pub fn pam_2(&self) -> PAM_2_R {
        PAM_2_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:"]
    #[inline(always)]
    pub fn sbx(&self) -> SBX_R {
        SBX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
    #[inline(always)]
    pub fn sap(&self) -> SAP_R {
        SAP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - binary logarithm of the destination data width of a burst, in bytes Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. Note: A destination burst transfer must have an aligned address with its data width (start address GPDMA_CxDAR\\[2:0\\]
and address offset GPDMA_CxTR3.DAO\\[2:0\\], versus DDW_LOG2\\[1:0\\]). Otherwise a user setting error is reported and no transfer is issued."]
    #[inline(always)]
    pub fn ddw_log2(&self) -> DDW_LOG2_R {
        DDW_LOG2_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - destination incrementing burst The destination address, pointed by GPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
    #[inline(always)]
    pub fn dinc(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:25 - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\\[5:0\\]
=0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
    #[inline(always)]
    pub fn dbl_1(&self) -> DBL_1_R {
        DBL_1_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bit 26 - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:"]
    #[inline(always)]
    pub fn dbx(&self) -> DBX_R {
        DBX_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:"]
    #[inline(always)]
    pub fn dhx(&self) -> DHX_R {
        DHX_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
    #[inline(always)]
    pub fn dap(&self) -> DAP_R {
        DAP_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - binary logarithm of the source data width of a burst in bytes Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and no transfer is issued. Note: A source burst transfer must have an aligned address with its data width (start address GPDMA_CxSAR\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and none transfer is issued."]
    #[inline(always)]
    #[must_use]
    pub fn sdw_log2(&mut self) -> SDW_LOG2_W<C1TR1rs> {
        SDW_LOG2_W::new(self, 0)
    }
    #[doc = "Bit 3 - source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
    #[inline(always)]
    #[must_use]
    pub fn sinc(&mut self) -> SINC_W<C1TR1rs> {
        SINC_W::new(self, 3)
    }
    #[doc = "Bits 4:9 - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\\[5:0\\]
=0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
    #[inline(always)]
    #[must_use]
    pub fn sbl_1(&mut self) -> SBL_1_W<C1TR1rs> {
        SBL_1_W::new(self, 4)
    }
    #[doc = "Bits 11:12 - padding/alignment mode If DDW_LOG2\\[1:0\\]
= SDW_LOG2\\[1:0\\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else, in the following enumerated values, the condition PAM_1 is when destination data width is higher that source data width, and the condition PAM_2 is when destination data width is higher than source data width. 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination Note: If the transfer from the source peripheral is configured with peripheral flow-control mode (SWREQ = 0 and PFREQ = 1 and DREQ = 0), and if the destination data width > the source data width, packing is not supported."]
    #[inline(always)]
    #[must_use]
    pub fn pam(&mut self) -> PAM_W<C1TR1rs> {
        PAM_W::new(self, 11)
    }
    #[doc = "Bits 11:12 - PAM value when destination data width is higher than source data width"]
    #[inline(always)]
    #[must_use]
    pub fn pam_1(&mut self) -> PAM_1_W<C1TR1rs> {
        PAM_1_W::new(self, 11)
    }
    #[doc = "Bits 11:12 - PAM value when source data width is higher than destination data width"]
    #[inline(always)]
    #[must_use]
    pub fn pam_2(&mut self) -> PAM_2_W<C1TR1rs> {
        PAM_2_W::new(self, 11)
    }
    #[doc = "Bit 13 - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:"]
    #[inline(always)]
    #[must_use]
    pub fn sbx(&mut self) -> SBX_W<C1TR1rs> {
        SBX_W::new(self, 13)
    }
    #[doc = "Bit 14 - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
    #[inline(always)]
    #[must_use]
    pub fn sap(&mut self) -> SAP_W<C1TR1rs> {
        SAP_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - binary logarithm of the destination data width of a burst, in bytes Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. Note: A destination burst transfer must have an aligned address with its data width (start address GPDMA_CxDAR\\[2:0\\]
and address offset GPDMA_CxTR3.DAO\\[2:0\\], versus DDW_LOG2\\[1:0\\]). Otherwise a user setting error is reported and no transfer is issued."]
    #[inline(always)]
    #[must_use]
    pub fn ddw_log2(&mut self) -> DDW_LOG2_W<C1TR1rs> {
        DDW_LOG2_W::new(self, 16)
    }
    #[doc = "Bit 19 - destination incrementing burst The destination address, pointed by GPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
    #[inline(always)]
    #[must_use]
    pub fn dinc(&mut self) -> DINC_W<C1TR1rs> {
        DINC_W::new(self, 19)
    }
    #[doc = "Bits 20:25 - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\\[5:0\\]
=0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
    #[inline(always)]
    #[must_use]
    pub fn dbl_1(&mut self) -> DBL_1_W<C1TR1rs> {
        DBL_1_W::new(self, 20)
    }
    #[doc = "Bit 26 - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:"]
    #[inline(always)]
    #[must_use]
    pub fn dbx(&mut self) -> DBX_W<C1TR1rs> {
        DBX_W::new(self, 26)
    }
    #[doc = "Bit 27 - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:"]
    #[inline(always)]
    #[must_use]
    pub fn dhx(&mut self) -> DHX_W<C1TR1rs> {
        DHX_W::new(self, 27)
    }
    #[doc = "Bit 30 - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
    #[inline(always)]
    #[must_use]
    pub fn dap(&mut self) -> DAP_W<C1TR1rs> {
        DAP_W::new(self, 30)
    }
}
#[doc = "GPDMA channel 1 transfer register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1tr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1tr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1TR1rs;
impl crate::RegisterSpec for C1TR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1tr1::R`](R) reader structure"]
impl crate::Readable for C1TR1rs {}
#[doc = "`write(|w| ..)` method takes [`c1tr1::W`](W) writer structure"]
impl crate::Writable for C1TR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1TR1 to value 0"]
impl crate::Resettable for C1TR1rs {
    const RESET_VALUE: u32 = 0;
}
