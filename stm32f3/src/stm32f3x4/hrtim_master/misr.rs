#[doc = "Reader of register MISR"]
pub type R = crate::R<u32, super::MISR>;
#[doc = "Master Update Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUPD_A {
    #[doc = "0: No master update interrupt occurred"]
    NOEVENT = 0,
    #[doc = "1: Master update interrupt occurred"]
    EVENT = 1,
}
impl From<MUPD_A> for bool {
    #[inline(always)]
    fn from(variant: MUPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MUPD`"]
pub type MUPD_R = crate::R<bool, MUPD_A>;
impl MUPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUPD_A {
        match self.bits {
            false => MUPD_A::NOEVENT,
            true => MUPD_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == MUPD_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == MUPD_A::EVENT
    }
}
#[doc = "Sync Input Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_A {
    #[doc = "0: No sync input interrupt occurred"]
    NOEVENT = 0,
    #[doc = "1: Sync input interrupt occurred"]
    EVENT = 1,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNC`"]
pub type SYNC_R = crate::R<bool, SYNC_A>;
impl SYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_A {
        match self.bits {
            false => SYNC_A::NOEVENT,
            true => SYNC_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == SYNC_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == SYNC_A::EVENT
    }
}
#[doc = "Master Repetition Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MREP_A {
    #[doc = "0: No master repetition interrupt occurred"]
    NOEVENT = 0,
    #[doc = "1: Master repetition interrupt occurred"]
    EVENT = 1,
}
impl From<MREP_A> for bool {
    #[inline(always)]
    fn from(variant: MREP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MREP`"]
pub type MREP_R = crate::R<bool, MREP_A>;
impl MREP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MREP_A {
        match self.bits {
            false => MREP_A::NOEVENT,
            true => MREP_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == MREP_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == MREP_A::EVENT
    }
}
#[doc = "Master Compare 4 Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCMP4_A {
    #[doc = "0: No master compare interrupt occurred"]
    NOEVENT = 0,
    #[doc = "1: Master compare interrupt occurred"]
    EVENT = 1,
}
impl From<MCMP4_A> for bool {
    #[inline(always)]
    fn from(variant: MCMP4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCMP4`"]
pub type MCMP4_R = crate::R<bool, MCMP4_A>;
impl MCMP4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCMP4_A {
        match self.bits {
            false => MCMP4_A::NOEVENT,
            true => MCMP4_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == MCMP4_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == MCMP4_A::EVENT
    }
}
#[doc = "Master Compare 3 Interrupt Flag"]
pub type MCMP3_A = MCMP4_A;
#[doc = "Reader of field `MCMP3`"]
pub type MCMP3_R = crate::R<bool, MCMP4_A>;
#[doc = "Master Compare 2 Interrupt Flag"]
pub type MCMP2_A = MCMP4_A;
#[doc = "Reader of field `MCMP2`"]
pub type MCMP2_R = crate::R<bool, MCMP4_A>;
#[doc = "Master Compare 1 Interrupt Flag"]
pub type MCMP1_A = MCMP4_A;
#[doc = "Reader of field `MCMP1`"]
pub type MCMP1_R = crate::R<bool, MCMP4_A>;
impl R {
    #[doc = "Bit 6 - Master Update Interrupt Flag"]
    #[inline(always)]
    pub fn mupd(&self) -> MUPD_R {
        MUPD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Sync Input Interrupt Flag"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Master Repetition Interrupt Flag"]
    #[inline(always)]
    pub fn mrep(&self) -> MREP_R {
        MREP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Master Compare 4 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp4(&self) -> MCMP4_R {
        MCMP4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Master Compare 3 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp3(&self) -> MCMP3_R {
        MCMP3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Master Compare 2 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp2(&self) -> MCMP2_R {
        MCMP2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Master Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp1(&self) -> MCMP1_R {
        MCMP1_R::new((self.bits & 0x01) != 0)
    }
}
