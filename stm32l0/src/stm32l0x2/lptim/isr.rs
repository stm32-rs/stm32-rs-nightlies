#[doc = "Reader of register ISR"]
pub type R = crate::R<u32, super::ISR>;
#[doc = "Counter direction change up to down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOWN_A {
    #[doc = "1: Counter direction change up to down"]
    SET = 1,
}
impl From<DOWN_A> for bool {
    #[inline(always)]
    fn from(variant: DOWN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DOWN`"]
pub type DOWN_R = crate::R<bool, DOWN_A>;
impl DOWN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DOWN_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(DOWN_A::SET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == DOWN_A::SET
    }
}
#[doc = "Counter direction change down to up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UP_A {
    #[doc = "1: Counter direction change down to up"]
    SET = 1,
}
impl From<UP_A> for bool {
    #[inline(always)]
    fn from(variant: UP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UP`"]
pub type UP_R = crate::R<bool, UP_A>;
impl UP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, UP_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(UP_A::SET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == UP_A::SET
    }
}
#[doc = "Autoreload register update OK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARROK_A {
    #[doc = "1: Autoreload register update OK"]
    SET = 1,
}
impl From<ARROK_A> for bool {
    #[inline(always)]
    fn from(variant: ARROK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ARROK`"]
pub type ARROK_R = crate::R<bool, ARROK_A>;
impl ARROK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ARROK_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(ARROK_A::SET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ARROK_A::SET
    }
}
#[doc = "Compare register update OK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPOK_A {
    #[doc = "1: Compare register update OK"]
    SET = 1,
}
impl From<CMPOK_A> for bool {
    #[inline(always)]
    fn from(variant: CMPOK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMPOK`"]
pub type CMPOK_R = crate::R<bool, CMPOK_A>;
impl CMPOK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CMPOK_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CMPOK_A::SET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CMPOK_A::SET
    }
}
#[doc = "External trigger edge event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTTRIG_A {
    #[doc = "1: External trigger edge event"]
    SET = 1,
}
impl From<EXTTRIG_A> for bool {
    #[inline(always)]
    fn from(variant: EXTTRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTTRIG`"]
pub type EXTTRIG_R = crate::R<bool, EXTTRIG_A>;
impl EXTTRIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, EXTTRIG_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(EXTTRIG_A::SET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == EXTTRIG_A::SET
    }
}
#[doc = "Autoreload match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARRM_A {
    #[doc = "1: Autoreload match"]
    SET = 1,
}
impl From<ARRM_A> for bool {
    #[inline(always)]
    fn from(variant: ARRM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ARRM`"]
pub type ARRM_R = crate::R<bool, ARRM_A>;
impl ARRM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ARRM_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(ARRM_A::SET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ARRM_A::SET
    }
}
#[doc = "Compare match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPM_A {
    #[doc = "1: Compare match"]
    SET = 1,
}
impl From<CMPM_A> for bool {
    #[inline(always)]
    fn from(variant: CMPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMPM`"]
pub type CMPM_R = crate::R<bool, CMPM_A>;
impl CMPM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CMPM_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CMPM_A::SET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CMPM_A::SET
    }
}
impl R {
    #[doc = "Bit 6 - Counter direction change up to down"]
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Counter direction change down to up"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Autoreload register update OK"]
    #[inline(always)]
    pub fn arrok(&self) -> ARROK_R {
        ARROK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Compare register update OK"]
    #[inline(always)]
    pub fn cmpok(&self) -> CMPOK_R {
        CMPOK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External trigger edge event"]
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Autoreload match"]
    #[inline(always)]
    pub fn arrm(&self) -> ARRM_R {
        ARRM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Compare match"]
    #[inline(always)]
    pub fn cmpm(&self) -> CMPM_R {
        CMPM_R::new((self.bits & 0x01) != 0)
    }
}
