#[doc = "Reader of register CDSR"]
pub type R = crate::R<u32, super::CDSR>;
#[doc = "Horizontal Synchronization display Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSYNCS_A {
    #[doc = "0: Currently not in HSYNC phase"]
    NOTACTIVE = 0,
    #[doc = "1: Currently in HSYNC phase"]
    ACTIVE = 1,
}
impl From<HSYNCS_A> for bool {
    #[inline(always)]
    fn from(variant: HSYNCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSYNCS`"]
pub type HSYNCS_R = crate::R<bool, HSYNCS_A>;
impl HSYNCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSYNCS_A {
        match self.bits {
            false => HSYNCS_A::NOTACTIVE,
            true => HSYNCS_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == HSYNCS_A::NOTACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == HSYNCS_A::ACTIVE
    }
}
#[doc = "Vertical Synchronization display Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VSYNCS_A {
    #[doc = "0: Currently not in VSYNC phase"]
    NOTACTIVE = 0,
    #[doc = "1: Currently in VSYNC phase"]
    ACTIVE = 1,
}
impl From<VSYNCS_A> for bool {
    #[inline(always)]
    fn from(variant: VSYNCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VSYNCS`"]
pub type VSYNCS_R = crate::R<bool, VSYNCS_A>;
impl VSYNCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VSYNCS_A {
        match self.bits {
            false => VSYNCS_A::NOTACTIVE,
            true => VSYNCS_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == VSYNCS_A::NOTACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == VSYNCS_A::ACTIVE
    }
}
#[doc = "Horizontal Data Enable display Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDES_A {
    #[doc = "0: Currently not in horizontal Data Enable phase"]
    NOTACTIVE = 0,
    #[doc = "1: Currently in horizontal Data Enable phase"]
    ACTIVE = 1,
}
impl From<HDES_A> for bool {
    #[inline(always)]
    fn from(variant: HDES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HDES`"]
pub type HDES_R = crate::R<bool, HDES_A>;
impl HDES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDES_A {
        match self.bits {
            false => HDES_A::NOTACTIVE,
            true => HDES_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == HDES_A::NOTACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == HDES_A::ACTIVE
    }
}
#[doc = "Vertical Data Enable display Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDES_A {
    #[doc = "0: Currently not in vertical Data Enable phase"]
    NOTACTIVE = 0,
    #[doc = "1: Currently in vertical Data Enable phase"]
    ACTIVE = 1,
}
impl From<VDES_A> for bool {
    #[inline(always)]
    fn from(variant: VDES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VDES`"]
pub type VDES_R = crate::R<bool, VDES_A>;
impl VDES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDES_A {
        match self.bits {
            false => VDES_A::NOTACTIVE,
            true => VDES_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == VDES_A::NOTACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == VDES_A::ACTIVE
    }
}
impl R {
    #[doc = "Bit 3 - Horizontal Synchronization display Status"]
    #[inline(always)]
    pub fn hsyncs(&self) -> HSYNCS_R {
        HSYNCS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Vertical Synchronization display Status"]
    #[inline(always)]
    pub fn vsyncs(&self) -> VSYNCS_R {
        VSYNCS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Horizontal Data Enable display Status"]
    #[inline(always)]
    pub fn hdes(&self) -> HDES_R {
        HDES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Vertical Data Enable display Status"]
    #[inline(always)]
    pub fn vdes(&self) -> VDES_R {
        VDES_R::new((self.bits & 0x01) != 0)
    }
}
