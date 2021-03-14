#[doc = "Reader of register MACRxTxSR"]
pub type R = crate::R<u32, super::MACRXTXSR>;
#[doc = "Reader of field `TJT`"]
pub type TJT_R = crate::R<bool, bool>;
#[doc = "Reader of field `NCARR`"]
pub type NCARR_R = crate::R<bool, bool>;
#[doc = "Reader of field `LCARR`"]
pub type LCARR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXDEF`"]
pub type EXDEF_R = crate::R<bool, bool>;
#[doc = "Reader of field `LCOL`"]
pub type LCOL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXCOL`"]
pub type EXCOL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RWT`"]
pub type RWT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Transmit Jabber Timeout"]
    #[inline(always)]
    pub fn tjt(&self) -> TJT_R {
        TJT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - No Carrier"]
    #[inline(always)]
    pub fn ncarr(&self) -> NCARR_R {
        NCARR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Loss of Carrier"]
    #[inline(always)]
    pub fn lcarr(&self) -> LCARR_R {
        LCARR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Excessive Deferral"]
    #[inline(always)]
    pub fn exdef(&self) -> EXDEF_R {
        EXDEF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Late Collision"]
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Excessive Collisions"]
    #[inline(always)]
    pub fn excol(&self) -> EXCOL_R {
        EXCOL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive Watchdog Timeout"]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
