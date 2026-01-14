///Register `RIR` reader
pub type R = crate::R<RIRrs>;
/**RTR

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTR {
    ///0: Data frame
    Data = 0,
    ///1: Remote frame
    Remote = 1,
}
impl From<RTR> for bool {
    #[inline(always)]
    fn from(variant: RTR) -> Self {
        variant as u8 != 0
    }
}
///Field `RTR` reader - RTR
pub type RTR_R = crate::BitReader<RTR>;
impl RTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTR {
        match self.bits {
            false => RTR::Data,
            true => RTR::Remote,
        }
    }
    ///Data frame
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == RTR::Data
    }
    ///Remote frame
    #[inline(always)]
    pub fn is_remote(&self) -> bool {
        *self == RTR::Remote
    }
}
/**IDE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDE {
    ///0: Standard identifier
    Standard = 0,
    ///1: Extended identifier
    Extended = 1,
}
impl From<IDE> for bool {
    #[inline(always)]
    fn from(variant: IDE) -> Self {
        variant as u8 != 0
    }
}
///Field `IDE` reader - IDE
pub type IDE_R = crate::BitReader<IDE>;
impl IDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDE {
        match self.bits {
            false => IDE::Standard,
            true => IDE::Extended,
        }
    }
    ///Standard identifier
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == IDE::Standard
    }
    ///Extended identifier
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == IDE::Extended
    }
}
///Field `EXID` reader - EXID
pub type EXID_R = crate::FieldReader<u32>;
///Field `STID` reader - STID
pub type STID_R = crate::FieldReader<u16>;
impl R {
    ///Bit 1 - RTR
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IDE
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:20 - EXID
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    ///Bits 21:31 - STID
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIR")
            .field("stid", &self.stid())
            .field("exid", &self.exid())
            .field("ide", &self.ide())
            .field("rtr", &self.rtr())
            .finish()
    }
}
/**receive FIFO mailbox identifier register

You can [`read`](crate::Reg::read) this register and get [`rir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RIRrs;
impl crate::RegisterSpec for RIRrs {
    type Ux = u32;
}
///`read()` method returns [`rir::R`](R) reader structure
impl crate::Readable for RIRrs {}
///`reset()` method sets RIR to value 0
impl crate::Resettable for RIRrs {}
