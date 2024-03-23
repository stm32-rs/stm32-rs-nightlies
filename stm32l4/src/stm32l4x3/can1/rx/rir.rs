#[doc = "Register `RIR` reader"]
pub type R = crate::R<RIRrs>;
#[doc = "RTR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTR {
    #[doc = "0: Data frame"]
    Data = 0,
    #[doc = "1: Remote frame"]
    Remote = 1,
}
impl From<RTR> for bool {
    #[inline(always)]
    fn from(variant: RTR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTR` reader - RTR"]
pub type RTR_R = crate::BitReader<RTR>;
impl RTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTR {
        match self.bits {
            false => RTR::Data,
            true => RTR::Remote,
        }
    }
    #[doc = "Data frame"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == RTR::Data
    }
    #[doc = "Remote frame"]
    #[inline(always)]
    pub fn is_remote(&self) -> bool {
        *self == RTR::Remote
    }
}
#[doc = "IDE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDE {
    #[doc = "0: Standard identifier"]
    Standard = 0,
    #[doc = "1: Extended identifier"]
    Extended = 1,
}
impl From<IDE> for bool {
    #[inline(always)]
    fn from(variant: IDE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDE` reader - IDE"]
pub type IDE_R = crate::BitReader<IDE>;
impl IDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IDE {
        match self.bits {
            false => IDE::Standard,
            true => IDE::Extended,
        }
    }
    #[doc = "Standard identifier"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == IDE::Standard
    }
    #[doc = "Extended identifier"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == IDE::Extended
    }
}
#[doc = "Field `EXID` reader - EXID"]
pub type EXID_R = crate::FieldReader<u32>;
#[doc = "Field `STID` reader - STID"]
pub type STID_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 1 - RTR"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - EXID"]
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - STID"]
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
#[doc = "receive FIFO mailbox identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rir::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RIRrs;
impl crate::RegisterSpec for RIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rir::R`](R) reader structure"]
impl crate::Readable for RIRrs {}
#[doc = "`reset()` method sets RIR to value 0"]
impl crate::Resettable for RIRrs {
    const RESET_VALUE: u32 = 0;
}
