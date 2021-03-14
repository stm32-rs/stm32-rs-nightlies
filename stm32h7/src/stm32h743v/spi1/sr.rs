#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `CTSIZE`"]
pub type CTSIZE_R = crate::R<u16, u16>;
#[doc = "RxFIFO Word Not Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXWNE_A {
    #[doc = "0: Less than 32-bit data frame received"]
    LESSTHAN32 = 0,
    #[doc = "1: At least 32-bit data frame received"]
    ATLEAST32 = 1,
}
impl From<RXWNE_A> for bool {
    #[inline(always)]
    fn from(variant: RXWNE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXWNE`"]
pub type RXWNE_R = crate::R<bool, RXWNE_A>;
impl RXWNE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXWNE_A {
        match self.bits {
            false => RXWNE_A::LESSTHAN32,
            true => RXWNE_A::ATLEAST32,
        }
    }
    #[doc = "Checks if the value of the field is `LESSTHAN32`"]
    #[inline(always)]
    pub fn is_less_than32(&self) -> bool {
        *self == RXWNE_A::LESSTHAN32
    }
    #[doc = "Checks if the value of the field is `ATLEAST32`"]
    #[inline(always)]
    pub fn is_at_least32(&self) -> bool {
        *self == RXWNE_A::ATLEAST32
    }
}
#[doc = "RxFIFO Packing LeVeL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXPLVL_A {
    #[doc = "0: Zero frames beyond packing ratio available"]
    ZEROFRAMES = 0,
    #[doc = "1: One frame beyond packing ratio available"]
    ONEFRAME = 1,
    #[doc = "2: Two frame beyond packing ratio available"]
    TWOFRAMES = 2,
    #[doc = "3: Three frame beyond packing ratio available"]
    THREEFRAMES = 3,
}
impl From<RXPLVL_A> for u8 {
    #[inline(always)]
    fn from(variant: RXPLVL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RXPLVL`"]
pub type RXPLVL_R = crate::R<u8, RXPLVL_A>;
impl RXPLVL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPLVL_A {
        match self.bits {
            0 => RXPLVL_A::ZEROFRAMES,
            1 => RXPLVL_A::ONEFRAME,
            2 => RXPLVL_A::TWOFRAMES,
            3 => RXPLVL_A::THREEFRAMES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ZEROFRAMES`"]
    #[inline(always)]
    pub fn is_zero_frames(&self) -> bool {
        *self == RXPLVL_A::ZEROFRAMES
    }
    #[doc = "Checks if the value of the field is `ONEFRAME`"]
    #[inline(always)]
    pub fn is_one_frame(&self) -> bool {
        *self == RXPLVL_A::ONEFRAME
    }
    #[doc = "Checks if the value of the field is `TWOFRAMES`"]
    #[inline(always)]
    pub fn is_two_frames(&self) -> bool {
        *self == RXPLVL_A::TWOFRAMES
    }
    #[doc = "Checks if the value of the field is `THREEFRAMES`"]
    #[inline(always)]
    pub fn is_three_frames(&self) -> bool {
        *self == RXPLVL_A::THREEFRAMES
    }
}
#[doc = "TxFIFO transmission complete\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXC_A {
    #[doc = "0: Transmission ongoing"]
    ONGOING = 0,
    #[doc = "1: Transmission completed"]
    COMPLETED = 1,
}
impl From<TXC_A> for bool {
    #[inline(always)]
    fn from(variant: TXC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXC`"]
pub type TXC_R = crate::R<bool, TXC_A>;
impl TXC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXC_A {
        match self.bits {
            false => TXC_A::ONGOING,
            true => TXC_A::COMPLETED,
        }
    }
    #[doc = "Checks if the value of the field is `ONGOING`"]
    #[inline(always)]
    pub fn is_ongoing(&self) -> bool {
        *self == TXC_A::ONGOING
    }
    #[doc = "Checks if the value of the field is `COMPLETED`"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == TXC_A::COMPLETED
    }
}
#[doc = "SUSPend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSP_A {
    #[doc = "0: Master not suspended"]
    NOTSUSPENDED = 0,
    #[doc = "1: Master suspended"]
    SUSPENDED = 1,
}
impl From<SUSP_A> for bool {
    #[inline(always)]
    fn from(variant: SUSP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SUSP`"]
pub type SUSP_R = crate::R<bool, SUSP_A>;
impl SUSP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSP_A {
        match self.bits {
            false => SUSP_A::NOTSUSPENDED,
            true => SUSP_A::SUSPENDED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSUSPENDED`"]
    #[inline(always)]
    pub fn is_not_suspended(&self) -> bool {
        *self == SUSP_A::NOTSUSPENDED
    }
    #[doc = "Checks if the value of the field is `SUSPENDED`"]
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == SUSP_A::SUSPENDED
    }
}
#[doc = "Additional number of SPI data to be transacted was reload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSERF_A {
    #[doc = "0: Additional number of SPI data to be transacted not yet loaded"]
    NOTLOADED = 0,
    #[doc = "1: Additional number of SPI data to be transacted was reloaded"]
    LOADED = 1,
}
impl From<TSERF_A> for bool {
    #[inline(always)]
    fn from(variant: TSERF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TSERF`"]
pub type TSERF_R = crate::R<bool, TSERF_A>;
impl TSERF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSERF_A {
        match self.bits {
            false => TSERF_A::NOTLOADED,
            true => TSERF_A::LOADED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLOADED`"]
    #[inline(always)]
    pub fn is_not_loaded(&self) -> bool {
        *self == TSERF_A::NOTLOADED
    }
    #[doc = "Checks if the value of the field is `LOADED`"]
    #[inline(always)]
    pub fn is_loaded(&self) -> bool {
        *self == TSERF_A::LOADED
    }
}
#[doc = "Mode Fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODF_A {
    #[doc = "0: No mode fault detected"]
    NOFAULT = 0,
    #[doc = "1: Mode fault detected"]
    FAULT = 1,
}
impl From<MODF_A> for bool {
    #[inline(always)]
    fn from(variant: MODF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODF`"]
pub type MODF_R = crate::R<bool, MODF_A>;
impl MODF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODF_A {
        match self.bits {
            false => MODF_A::NOFAULT,
            true => MODF_A::FAULT,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAULT`"]
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == MODF_A::NOFAULT
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == MODF_A::FAULT
    }
}
#[doc = "TI frame format error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIFRE_A {
    #[doc = "0: TI frame format error detected"]
    NOERROR = 0,
    #[doc = "1: TI frame format error detected"]
    ERROR = 1,
}
impl From<TIFRE_A> for bool {
    #[inline(always)]
    fn from(variant: TIFRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIFRE`"]
pub type TIFRE_R = crate::R<bool, TIFRE_A>;
impl TIFRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIFRE_A {
        match self.bits {
            false => TIFRE_A::NOERROR,
            true => TIFRE_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TIFRE_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TIFRE_A::ERROR
    }
}
#[doc = "CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCE_A {
    #[doc = "0: No CRC error detected"]
    NOERROR = 0,
    #[doc = "1: CRC error detected"]
    ERROR = 1,
}
impl From<CRCE_A> for bool {
    #[inline(always)]
    fn from(variant: CRCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRCE`"]
pub type CRCE_R = crate::R<bool, CRCE_A>;
impl CRCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCE_A {
        match self.bits {
            false => CRCE_A::NOERROR,
            true => CRCE_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == CRCE_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == CRCE_A::ERROR
    }
}
#[doc = "Overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_A {
    #[doc = "0: No overrun occurred"]
    NOOVERRUN = 0,
    #[doc = "1: Overrun occurred"]
    OVERRUN = 1,
}
impl From<OVR_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVR`"]
pub type OVR_R = crate::R<bool, OVR_A>;
impl OVR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVR_A {
        match self.bits {
            false => OVR_A::NOOVERRUN,
            true => OVR_A::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOOVERRUN`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR_A::NOOVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR_A::OVERRUN
    }
}
#[doc = "Underrun at slave transmission mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDR_A {
    #[doc = "0: No underrun occurred"]
    NOUNDERRUN = 0,
    #[doc = "1: Underrun occurred"]
    UNDERRUN = 1,
}
impl From<UDR_A> for bool {
    #[inline(always)]
    fn from(variant: UDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UDR`"]
pub type UDR_R = crate::R<bool, UDR_A>;
impl UDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UDR_A {
        match self.bits {
            false => UDR_A::NOUNDERRUN,
            true => UDR_A::UNDERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOUNDERRUN`"]
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == UDR_A::NOUNDERRUN
    }
    #[doc = "Checks if the value of the field is `UNDERRUN`"]
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == UDR_A::UNDERRUN
    }
}
#[doc = "Transmission Transfer Filled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXTF_A {
    #[doc = "0: Transmission buffer incomplete"]
    NOTCOMPLETED = 0,
    #[doc = "1: Transmission buffer filled with at least one transfer"]
    COMPLETED = 1,
}
impl From<TXTF_A> for bool {
    #[inline(always)]
    fn from(variant: TXTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXTF`"]
pub type TXTF_R = crate::R<bool, TXTF_A>;
impl TXTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXTF_A {
        match self.bits {
            false => TXTF_A::NOTCOMPLETED,
            true => TXTF_A::COMPLETED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETED`"]
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == TXTF_A::NOTCOMPLETED
    }
    #[doc = "Checks if the value of the field is `COMPLETED`"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == TXTF_A::COMPLETED
    }
}
#[doc = "End Of Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOT_A {
    #[doc = "0: Transfer ongoing or not started"]
    NOTCOMPLETED = 0,
    #[doc = "1: Transfer complete"]
    COMPLETED = 1,
}
impl From<EOT_A> for bool {
    #[inline(always)]
    fn from(variant: EOT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EOT`"]
pub type EOT_R = crate::R<bool, EOT_A>;
impl EOT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOT_A {
        match self.bits {
            false => EOT_A::NOTCOMPLETED,
            true => EOT_A::COMPLETED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETED`"]
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == EOT_A::NOTCOMPLETED
    }
    #[doc = "Checks if the value of the field is `COMPLETED`"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == EOT_A::COMPLETED
    }
}
#[doc = "Duplex Packet\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DXP_A {
    #[doc = "0: Duplex packet unavailable: no space for transmission and/or no data received"]
    UNAVAILABLE = 0,
    #[doc = "1: Duplex packet available: space for transmission and data received"]
    AVAILABLE = 1,
}
impl From<DXP_A> for bool {
    #[inline(always)]
    fn from(variant: DXP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DXP`"]
pub type DXP_R = crate::R<bool, DXP_A>;
impl DXP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DXP_A {
        match self.bits {
            false => DXP_A::UNAVAILABLE,
            true => DXP_A::AVAILABLE,
        }
    }
    #[doc = "Checks if the value of the field is `UNAVAILABLE`"]
    #[inline(always)]
    pub fn is_unavailable(&self) -> bool {
        *self == DXP_A::UNAVAILABLE
    }
    #[doc = "Checks if the value of the field is `AVAILABLE`"]
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == DXP_A::AVAILABLE
    }
}
#[doc = "Tx-Packet space available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXP_A {
    #[doc = "0: Tx buffer full"]
    FULL = 0,
    #[doc = "1: Tx buffer not full"]
    NOTFULL = 1,
}
impl From<TXP_A> for bool {
    #[inline(always)]
    fn from(variant: TXP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXP`"]
pub type TXP_R = crate::R<bool, TXP_A>;
impl TXP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXP_A {
        match self.bits {
            false => TXP_A::FULL,
            true => TXP_A::NOTFULL,
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == TXP_A::FULL
    }
    #[doc = "Checks if the value of the field is `NOTFULL`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == TXP_A::NOTFULL
    }
}
#[doc = "Rx-Packet available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXP_A {
    #[doc = "0: Rx buffer empty"]
    EMPTY = 0,
    #[doc = "1: Rx buffer not empty"]
    NOTEMPTY = 1,
}
impl From<RXP_A> for bool {
    #[inline(always)]
    fn from(variant: RXP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXP`"]
pub type RXP_R = crate::R<bool, RXP_A>;
impl RXP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXP_A {
        match self.bits {
            false => RXP_A::EMPTY,
            true => RXP_A::NOTEMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXP_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `NOTEMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXP_A::NOTEMPTY
    }
}
impl R {
    #[doc = "Bits 16:31 - Number of data frames remaining in current TSIZE session"]
    #[inline(always)]
    pub fn ctsize(&self) -> CTSIZE_R {
        CTSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 15 - RxFIFO Word Not Empty"]
    #[inline(always)]
    pub fn rxwne(&self) -> RXWNE_R {
        RXWNE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - RxFIFO Packing LeVeL"]
    #[inline(always)]
    pub fn rxplvl(&self) -> RXPLVL_R {
        RXPLVL_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12 - TxFIFO transmission complete"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SUSPend"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Additional number of SPI data to be transacted was reload"]
    #[inline(always)]
    pub fn tserf(&self) -> TSERF_R {
        TSERF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Mode Fault"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TI frame format error"]
    #[inline(always)]
    pub fn tifre(&self) -> TIFRE_R {
        TIFRE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CRC Error"]
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Overrun"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Underrun at slave transmission mode"]
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmission Transfer Filled"]
    #[inline(always)]
    pub fn txtf(&self) -> TXTF_R {
        TXTF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End Of Transfer"]
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Duplex Packet"]
    #[inline(always)]
    pub fn dxp(&self) -> DXP_R {
        DXP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Tx-Packet space available"]
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Rx-Packet available"]
    #[inline(always)]
    pub fn rxp(&self) -> RXP_R {
        RXP_R::new((self.bits & 0x01) != 0)
    }
}
