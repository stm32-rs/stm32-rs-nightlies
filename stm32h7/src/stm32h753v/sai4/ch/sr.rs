#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Overrun / underrun. This bit is read only. The overrun and underrun conditions can occur only when the audio block is configured as a receiver and a transmitter, respectively. It can generate an interrupt if OVRUDRIE bit is set in SAI_xIM register. This flag is cleared when the software sets COVRUDR bit in SAI_xCLRFR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRUDRR {
    #[doc = "0: No overrun/underrun error"]
    NoError = 0,
    #[doc = "1: Overrun/underrun error detection"]
    Overrun = 1,
}
impl From<OVRUDRR> for bool {
    #[inline(always)]
    fn from(variant: OVRUDRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRUDR` reader - Overrun / underrun. This bit is read only. The overrun and underrun conditions can occur only when the audio block is configured as a receiver and a transmitter, respectively. It can generate an interrupt if OVRUDRIE bit is set in SAI_xIM register. This flag is cleared when the software sets COVRUDR bit in SAI_xCLRFR register."]
pub type OVRUDR_R = crate::BitReader<OVRUDRR>;
impl OVRUDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVRUDRR {
        match self.bits {
            false => OVRUDRR::NoError,
            true => OVRUDRR::Overrun,
        }
    }
    #[doc = "No overrun/underrun error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == OVRUDRR::NoError
    }
    #[doc = "Overrun/underrun error detection"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVRUDRR::Overrun
    }
}
#[doc = "Mute detection. This bit is read only. This flag is set if consecutive 0 values are received in each slot of a given audio frame and for a consecutive number of audio frames (set in the MUTECNT bit in the SAI_xCR2 register). It can generate an interrupt if MUTEDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets bit CMUTEDET in the SAI_xCLRFR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUTEDETR {
    #[doc = "0: No MUTE detection on the SD input line"]
    NoMute = 0,
    #[doc = "1: MUTE value detected on the SD input line (0 value) for a specified number of consecutive audio frame"]
    Mute = 1,
}
impl From<MUTEDETR> for bool {
    #[inline(always)]
    fn from(variant: MUTEDETR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUTEDET` reader - Mute detection. This bit is read only. This flag is set if consecutive 0 values are received in each slot of a given audio frame and for a consecutive number of audio frames (set in the MUTECNT bit in the SAI_xCR2 register). It can generate an interrupt if MUTEDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets bit CMUTEDET in the SAI_xCLRFR register."]
pub type MUTEDET_R = crate::BitReader<MUTEDETR>;
impl MUTEDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MUTEDETR {
        match self.bits {
            false => MUTEDETR::NoMute,
            true => MUTEDETR::Mute,
        }
    }
    #[doc = "No MUTE detection on the SD input line"]
    #[inline(always)]
    pub fn is_no_mute(&self) -> bool {
        *self == MUTEDETR::NoMute
    }
    #[doc = "MUTE value detected on the SD input line (0 value) for a specified number of consecutive audio frame"]
    #[inline(always)]
    pub fn is_mute(&self) -> bool {
        *self == MUTEDETR::Mute
    }
}
#[doc = "Wrong clock configuration flag. This bit is read only. This bit is used only when the audio block operates in master mode (MODE\\[1\\]
= 0) and NODIV = 0. It can generate an interrupt if WCKCFGIE bit is set in SAI_xIM register. This flag is cleared when the software sets CWCKCFG bit in SAI_xCLRFR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WCKCFGR {
    #[doc = "0: Clock configuration is correct"]
    Correct = 0,
    #[doc = "1: Clock configuration does not respect the rule concerning the frame length specification"]
    Wrong = 1,
}
impl From<WCKCFGR> for bool {
    #[inline(always)]
    fn from(variant: WCKCFGR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WCKCFG` reader - Wrong clock configuration flag. This bit is read only. This bit is used only when the audio block operates in master mode (MODE\\[1\\]
= 0) and NODIV = 0. It can generate an interrupt if WCKCFGIE bit is set in SAI_xIM register. This flag is cleared when the software sets CWCKCFG bit in SAI_xCLRFR register."]
pub type WCKCFG_R = crate::BitReader<WCKCFGR>;
impl WCKCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WCKCFGR {
        match self.bits {
            false => WCKCFGR::Correct,
            true => WCKCFGR::Wrong,
        }
    }
    #[doc = "Clock configuration is correct"]
    #[inline(always)]
    pub fn is_correct(&self) -> bool {
        *self == WCKCFGR::Correct
    }
    #[doc = "Clock configuration does not respect the rule concerning the frame length specification"]
    #[inline(always)]
    pub fn is_wrong(&self) -> bool {
        *self == WCKCFGR::Wrong
    }
}
#[doc = "FIFO request. This bit is read only. The request depends on the audio block configuration: If the block is configured in transmission mode, the FIFO request is related to a write request operation in the SAI_xDR. If the block configured in reception, the FIFO request related to a read request operation from the SAI_xDR. This flag can generate an interrupt if FREQIE bit is set in SAI_xIM register.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FREQR {
    #[doc = "0: No FIFO request"]
    NoRequest = 0,
    #[doc = "1: FIFO request to read or to write the SAI_xDR"]
    Request = 1,
}
impl From<FREQR> for bool {
    #[inline(always)]
    fn from(variant: FREQR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREQ` reader - FIFO request. This bit is read only. The request depends on the audio block configuration: If the block is configured in transmission mode, the FIFO request is related to a write request operation in the SAI_xDR. If the block configured in reception, the FIFO request related to a read request operation from the SAI_xDR. This flag can generate an interrupt if FREQIE bit is set in SAI_xIM register."]
pub type FREQ_R = crate::BitReader<FREQR>;
impl FREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FREQR {
        match self.bits {
            false => FREQR::NoRequest,
            true => FREQR::Request,
        }
    }
    #[doc = "No FIFO request"]
    #[inline(always)]
    pub fn is_no_request(&self) -> bool {
        *self == FREQR::NoRequest
    }
    #[doc = "FIFO request to read or to write the SAI_xDR"]
    #[inline(always)]
    pub fn is_request(&self) -> bool {
        *self == FREQR::Request
    }
}
#[doc = "Codec not ready. This bit is read only. This bit is used only when the AC97 audio protocol is selected in the SAI_xCR1 register and configured in receiver mode. It can generate an interrupt if CNRDYIE bit is set in SAI_xIM register. This flag is cleared when the software sets CCNRDY bit in SAI_xCLRFR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNRDYR {
    #[doc = "0: External AC’97 Codec is ready"]
    Ready = 0,
    #[doc = "1: External AC’97 Codec is not ready"]
    NotReady = 1,
}
impl From<CNRDYR> for bool {
    #[inline(always)]
    fn from(variant: CNRDYR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNRDY` reader - Codec not ready. This bit is read only. This bit is used only when the AC97 audio protocol is selected in the SAI_xCR1 register and configured in receiver mode. It can generate an interrupt if CNRDYIE bit is set in SAI_xIM register. This flag is cleared when the software sets CCNRDY bit in SAI_xCLRFR register."]
pub type CNRDY_R = crate::BitReader<CNRDYR>;
impl CNRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CNRDYR {
        match self.bits {
            false => CNRDYR::Ready,
            true => CNRDYR::NotReady,
        }
    }
    #[doc = "External AC’97 Codec is ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == CNRDYR::Ready
    }
    #[doc = "External AC’97 Codec is not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == CNRDYR::NotReady
    }
}
#[doc = "Anticipated frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC97or SPDIF mode. It can generate an interrupt if AFSDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets CAFSDET bit in SAI_xCLRFR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFSDETR {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Frame synchronization signal is detected earlier than expected"]
    EarlySync = 1,
}
impl From<AFSDETR> for bool {
    #[inline(always)]
    fn from(variant: AFSDETR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AFSDET` reader - Anticipated frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC97or SPDIF mode. It can generate an interrupt if AFSDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets CAFSDET bit in SAI_xCLRFR register."]
pub type AFSDET_R = crate::BitReader<AFSDETR>;
impl AFSDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFSDETR {
        match self.bits {
            false => AFSDETR::NoError,
            true => AFSDETR::EarlySync,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == AFSDETR::NoError
    }
    #[doc = "Frame synchronization signal is detected earlier than expected"]
    #[inline(always)]
    pub fn is_early_sync(&self) -> bool {
        *self == AFSDETR::EarlySync
    }
}
#[doc = "Late frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC97 or SPDIF mode. It can generate an interrupt if LFSDETIE bit is set in the SAI_xIM register. This flag is cleared when the software sets bit CLFSDET in SAI_xCLRFR register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFSDETR {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Frame synchronization signal is not present at the right time"]
    NoSync = 1,
}
impl From<LFSDETR> for bool {
    #[inline(always)]
    fn from(variant: LFSDETR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFSDET` reader - Late frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC97 or SPDIF mode. It can generate an interrupt if LFSDETIE bit is set in the SAI_xIM register. This flag is cleared when the software sets bit CLFSDET in SAI_xCLRFR register"]
pub type LFSDET_R = crate::BitReader<LFSDETR>;
impl LFSDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LFSDETR {
        match self.bits {
            false => LFSDETR::NoError,
            true => LFSDETR::NoSync,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == LFSDETR::NoError
    }
    #[doc = "Frame synchronization signal is not present at the right time"]
    #[inline(always)]
    pub fn is_no_sync(&self) -> bool {
        *self == LFSDETR::NoSync
    }
}
#[doc = "FIFO level threshold. This bit is read only. The FIFO level threshold flag is managed only by hardware and its setting depends on SAI block configuration (transmitter or receiver mode). If the SAI block is configured as transmitter: If SAI block is configured as receiver:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLVLR {
    #[doc = "0: FIFO empty"]
    Empty = 0,
    #[doc = "1: FIFO &lt;= 1⁄4 but not empty"]
    Quarter1 = 1,
    #[doc = "2: 1⁄4 &lt; FIFO &lt;= 1⁄2"]
    Quarter2 = 2,
    #[doc = "3: 1⁄2 &lt; FIFO &lt;= 3⁄4"]
    Quarter3 = 3,
    #[doc = "4: 3⁄4 &lt; FIFO but not full"]
    Quarter4 = 4,
    #[doc = "5: FIFO full"]
    Full = 5,
}
impl From<FLVLR> for u8 {
    #[inline(always)]
    fn from(variant: FLVLR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FLVLR {
    type Ux = u8;
}
#[doc = "Field `FLVL` reader - FIFO level threshold. This bit is read only. The FIFO level threshold flag is managed only by hardware and its setting depends on SAI block configuration (transmitter or receiver mode). If the SAI block is configured as transmitter: If SAI block is configured as receiver:"]
pub type FLVL_R = crate::FieldReader<FLVLR>;
impl FLVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FLVLR> {
        match self.bits {
            0 => Some(FLVLR::Empty),
            1 => Some(FLVLR::Quarter1),
            2 => Some(FLVLR::Quarter2),
            3 => Some(FLVLR::Quarter3),
            4 => Some(FLVLR::Quarter4),
            5 => Some(FLVLR::Full),
            _ => None,
        }
    }
    #[doc = "FIFO empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FLVLR::Empty
    }
    #[doc = "FIFO &lt;= 1⁄4 but not empty"]
    #[inline(always)]
    pub fn is_quarter1(&self) -> bool {
        *self == FLVLR::Quarter1
    }
    #[doc = "1⁄4 &lt; FIFO &lt;= 1⁄2"]
    #[inline(always)]
    pub fn is_quarter2(&self) -> bool {
        *self == FLVLR::Quarter2
    }
    #[doc = "1⁄2 &lt; FIFO &lt;= 3⁄4"]
    #[inline(always)]
    pub fn is_quarter3(&self) -> bool {
        *self == FLVLR::Quarter3
    }
    #[doc = "3⁄4 &lt; FIFO but not full"]
    #[inline(always)]
    pub fn is_quarter4(&self) -> bool {
        *self == FLVLR::Quarter4
    }
    #[doc = "FIFO full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FLVLR::Full
    }
}
impl R {
    #[doc = "Bit 0 - Overrun / underrun. This bit is read only. The overrun and underrun conditions can occur only when the audio block is configured as a receiver and a transmitter, respectively. It can generate an interrupt if OVRUDRIE bit is set in SAI_xIM register. This flag is cleared when the software sets COVRUDR bit in SAI_xCLRFR register."]
    #[inline(always)]
    pub fn ovrudr(&self) -> OVRUDR_R {
        OVRUDR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mute detection. This bit is read only. This flag is set if consecutive 0 values are received in each slot of a given audio frame and for a consecutive number of audio frames (set in the MUTECNT bit in the SAI_xCR2 register). It can generate an interrupt if MUTEDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets bit CMUTEDET in the SAI_xCLRFR register."]
    #[inline(always)]
    pub fn mutedet(&self) -> MUTEDET_R {
        MUTEDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wrong clock configuration flag. This bit is read only. This bit is used only when the audio block operates in master mode (MODE\\[1\\]
= 0) and NODIV = 0. It can generate an interrupt if WCKCFGIE bit is set in SAI_xIM register. This flag is cleared when the software sets CWCKCFG bit in SAI_xCLRFR register."]
    #[inline(always)]
    pub fn wckcfg(&self) -> WCKCFG_R {
        WCKCFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO request. This bit is read only. The request depends on the audio block configuration: If the block is configured in transmission mode, the FIFO request is related to a write request operation in the SAI_xDR. If the block configured in reception, the FIFO request related to a read request operation from the SAI_xDR. This flag can generate an interrupt if FREQIE bit is set in SAI_xIM register."]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Codec not ready. This bit is read only. This bit is used only when the AC97 audio protocol is selected in the SAI_xCR1 register and configured in receiver mode. It can generate an interrupt if CNRDYIE bit is set in SAI_xIM register. This flag is cleared when the software sets CCNRDY bit in SAI_xCLRFR register."]
    #[inline(always)]
    pub fn cnrdy(&self) -> CNRDY_R {
        CNRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Anticipated frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC97or SPDIF mode. It can generate an interrupt if AFSDETIE bit is set in SAI_xIM register. This flag is cleared when the software sets CAFSDET bit in SAI_xCLRFR register."]
    #[inline(always)]
    pub fn afsdet(&self) -> AFSDET_R {
        AFSDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Late frame synchronization detection. This bit is read only. This flag can be set only if the audio block is configured in slave mode. It is not used in AC97 or SPDIF mode. It can generate an interrupt if LFSDETIE bit is set in the SAI_xIM register. This flag is cleared when the software sets bit CLFSDET in SAI_xCLRFR register"]
    #[inline(always)]
    pub fn lfsdet(&self) -> LFSDET_R {
        LFSDET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:18 - FIFO level threshold. This bit is read only. The FIFO level threshold flag is managed only by hardware and its setting depends on SAI block configuration (transmitter or receiver mode). If the SAI block is configured as transmitter: If SAI block is configured as receiver:"]
    #[inline(always)]
    pub fn flvl(&self) -> FLVL_R {
        FLVL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`reset()` method sets SR to value 0x08"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x08;
}
