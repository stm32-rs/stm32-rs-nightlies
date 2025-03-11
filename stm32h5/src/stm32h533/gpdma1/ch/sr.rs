///Register `SR` reader
pub type R = crate::R<SRrs>;
/**idle flag

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLEFR {
    ///0: Event not triggered
    NoTrigger = 0,
    ///1: Event triggered
    Trigger = 1,
}
impl From<IDLEFR> for bool {
    #[inline(always)]
    fn from(variant: IDLEFR) -> Self {
        variant as u8 != 0
    }
}
///Field `IDLEF` reader - idle flag
pub type IDLEF_R = crate::BitReader<IDLEFR>;
impl IDLEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDLEFR {
        match self.bits {
            false => IDLEFR::NoTrigger,
            true => IDLEFR::Trigger,
        }
    }
    ///Event not triggered
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == IDLEFR::NoTrigger
    }
    ///Event triggered
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == IDLEFR::Trigger
    }
}
///Field `TCF` reader - transfer complete flag
pub use IDLEF_R as TCF_R;
///Field `HTF` reader - half transfer flag
pub use IDLEF_R as HTF_R;
///Field `DTEF` reader - data transfer error flag
pub use IDLEF_R as DTEF_R;
///Field `ULEF` reader - update link transfer error flag
pub use IDLEF_R as ULEF_R;
///Field `USEF` reader - user setting error flag
pub use IDLEF_R as USEF_R;
///Field `SUSPF` reader - completed suspension flag
pub use IDLEF_R as SUSPF_R;
///Field `TOF` reader - trigger overrun flag
pub use IDLEF_R as TOF_R;
///Field `FIFOL` reader - monitored FIFO level
pub type FIFOL_R = crate::FieldReader;
impl R {
    ///Bit 0 - idle flag
    #[inline(always)]
    pub fn idlef(&self) -> IDLEF_R {
        IDLEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - transfer complete flag
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - half transfer flag
    #[inline(always)]
    pub fn htf(&self) -> HTF_R {
        HTF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - data transfer error flag
    #[inline(always)]
    pub fn dtef(&self) -> DTEF_R {
        DTEF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - update link transfer error flag
    #[inline(always)]
    pub fn ulef(&self) -> ULEF_R {
        ULEF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - user setting error flag
    #[inline(always)]
    pub fn usef(&self) -> USEF_R {
        USEF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - completed suspension flag
    #[inline(always)]
    pub fn suspf(&self) -> SUSPF_R {
        SUSPF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - trigger overrun flag
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:23 - monitored FIFO level
    #[inline(always)]
    pub fn fifol(&self) -> FIFOL_R {
        FIFOL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("idlef", &self.idlef())
            .field("tcf", &self.tcf())
            .field("htf", &self.htf())
            .field("dtef", &self.dtef())
            .field("ulef", &self.ulef())
            .field("usef", &self.usef())
            .field("suspf", &self.suspf())
            .field("tof", &self.tof())
            .field("fifol", &self.fifol())
            .finish()
    }
}
/**GPDMA channel 0 status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0x01
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x01;
}
