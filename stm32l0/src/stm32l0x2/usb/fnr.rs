#[doc = "Register `FNR` reader"]
pub type R = crate::R<FNRrs>;
#[doc = "Field `FN` reader - FN"]
pub type FN_R = crate::FieldReader<u16>;
#[doc = "Field `LSOF` reader - LSOF"]
pub type LSOF_R = crate::FieldReader;
#[doc = "LCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK {
    #[doc = "1: the frame timer remains in this state until an USB reset or USB suspend event occurs"]
    Locked = 1,
}
impl From<LCK> for bool {
    #[inline(always)]
    fn from(variant: LCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK` reader - LCK"]
pub type LCK_R = crate::BitReader<LCK>;
impl LCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LCK> {
        match self.bits {
            true => Some(LCK::Locked),
            _ => None,
        }
    }
    #[doc = "the frame timer remains in this state until an USB reset or USB suspend event occurs"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LCK::Locked
    }
}
#[doc = "RXDM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDM {
    #[doc = "1: received data minus upstream port data line"]
    Received = 1,
}
impl From<RXDM> for bool {
    #[inline(always)]
    fn from(variant: RXDM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDM` reader - RXDM"]
pub type RXDM_R = crate::BitReader<RXDM>;
impl RXDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RXDM> {
        match self.bits {
            true => Some(RXDM::Received),
            _ => None,
        }
    }
    #[doc = "received data minus upstream port data line"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == RXDM::Received
    }
}
#[doc = "RXDP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDP {
    #[doc = "1: received data plus upstream port data line"]
    Received = 1,
}
impl From<RXDP> for bool {
    #[inline(always)]
    fn from(variant: RXDP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDP` reader - RXDP"]
pub type RXDP_R = crate::BitReader<RXDP>;
impl RXDP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RXDP> {
        match self.bits {
            true => Some(RXDP::Received),
            _ => None,
        }
    }
    #[doc = "received data plus upstream port data line"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == RXDP::Received
    }
}
impl R {
    #[doc = "Bits 0:10 - FN"]
    #[inline(always)]
    pub fn fn_(&self) -> FN_R {
        FN_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:12 - LSOF"]
    #[inline(always)]
    pub fn lsof(&self) -> LSOF_R {
        LSOF_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - LCK"]
    #[inline(always)]
    pub fn lck(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RXDM"]
    #[inline(always)]
    pub fn rxdm(&self) -> RXDM_R {
        RXDM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RXDP"]
    #[inline(always)]
    pub fn rxdp(&self) -> RXDP_R {
        RXDP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "frame number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fnr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FNRrs;
impl crate::RegisterSpec for FNRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fnr::R`](R) reader structure"]
impl crate::Readable for FNRrs {}
#[doc = "`reset()` method sets FNR to value 0"]
impl crate::Resettable for FNRrs {
    const RESET_VALUE: u32 = 0;
}
