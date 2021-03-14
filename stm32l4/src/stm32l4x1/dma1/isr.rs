#[doc = "Reader of register ISR"]
pub type R = crate::R<u32, super::ISR>;
#[doc = "Channel x transfer error flag (x = 1 ..7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF7_A {
    #[doc = "0: No transfer error"]
    NOERROR = 0,
    #[doc = "1: A transfer error has occured"]
    ERROR = 1,
}
impl From<TEIF7_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEIF7`"]
pub type TEIF7_R = crate::R<bool, TEIF7_A>;
impl TEIF7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF7_A {
        match self.bits {
            false => TEIF7_A::NOERROR,
            true => TEIF7_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TEIF7_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TEIF7_A::ERROR
    }
}
#[doc = "Channel x half transfer flag (x = 1 ..7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTIF7_A {
    #[doc = "0: No half transfer event"]
    NOTHALF = 0,
    #[doc = "1: A half transfer event has occured"]
    HALF = 1,
}
impl From<HTIF7_A> for bool {
    #[inline(always)]
    fn from(variant: HTIF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HTIF7`"]
pub type HTIF7_R = crate::R<bool, HTIF7_A>;
impl HTIF7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTIF7_A {
        match self.bits {
            false => HTIF7_A::NOTHALF,
            true => HTIF7_A::HALF,
        }
    }
    #[doc = "Checks if the value of the field is `NOTHALF`"]
    #[inline(always)]
    pub fn is_not_half(&self) -> bool {
        *self == HTIF7_A::NOTHALF
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == HTIF7_A::HALF
    }
}
#[doc = "Channel x transfer complete flag (x = 1 ..7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF7_A {
    #[doc = "0: No transfer complete event"]
    NOTCOMPLETE = 0,
    #[doc = "1: A transfer complete event has occured"]
    COMPLETE = 1,
}
impl From<TCIF7_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCIF7`"]
pub type TCIF7_R = crate::R<bool, TCIF7_A>;
impl TCIF7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF7_A {
        match self.bits {
            false => TCIF7_A::NOTCOMPLETE,
            true => TCIF7_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TCIF7_A::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCIF7_A::COMPLETE
    }
}
#[doc = "Channel x global interrupt flag (x = 1 ..7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF7_A {
    #[doc = "0: No transfer error, half event, complete event"]
    NOEVENT = 0,
    #[doc = "1: A transfer error, half event or complete event has occured"]
    EVENT = 1,
}
impl From<GIF7_A> for bool {
    #[inline(always)]
    fn from(variant: GIF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GIF7`"]
pub type GIF7_R = crate::R<bool, GIF7_A>;
impl GIF7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIF7_A {
        match self.bits {
            false => GIF7_A::NOEVENT,
            true => GIF7_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == GIF7_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == GIF7_A::EVENT
    }
}
#[doc = "Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF6_A = TEIF7_A;
#[doc = "Reader of field `TEIF6`"]
pub type TEIF6_R = crate::R<bool, TEIF7_A>;
#[doc = "Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF6_A = HTIF7_A;
#[doc = "Reader of field `HTIF6`"]
pub type HTIF6_R = crate::R<bool, HTIF7_A>;
#[doc = "Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF6_A = TCIF7_A;
#[doc = "Reader of field `TCIF6`"]
pub type TCIF6_R = crate::R<bool, TCIF7_A>;
#[doc = "Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF6_A = GIF7_A;
#[doc = "Reader of field `GIF6`"]
pub type GIF6_R = crate::R<bool, GIF7_A>;
#[doc = "Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF5_A = TEIF7_A;
#[doc = "Reader of field `TEIF5`"]
pub type TEIF5_R = crate::R<bool, TEIF7_A>;
#[doc = "Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF5_A = HTIF7_A;
#[doc = "Reader of field `HTIF5`"]
pub type HTIF5_R = crate::R<bool, HTIF7_A>;
#[doc = "Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF5_A = TCIF7_A;
#[doc = "Reader of field `TCIF5`"]
pub type TCIF5_R = crate::R<bool, TCIF7_A>;
#[doc = "Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF5_A = GIF7_A;
#[doc = "Reader of field `GIF5`"]
pub type GIF5_R = crate::R<bool, GIF7_A>;
#[doc = "Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF4_A = TEIF7_A;
#[doc = "Reader of field `TEIF4`"]
pub type TEIF4_R = crate::R<bool, TEIF7_A>;
#[doc = "Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF4_A = HTIF7_A;
#[doc = "Reader of field `HTIF4`"]
pub type HTIF4_R = crate::R<bool, HTIF7_A>;
#[doc = "Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF4_A = TCIF7_A;
#[doc = "Reader of field `TCIF4`"]
pub type TCIF4_R = crate::R<bool, TCIF7_A>;
#[doc = "Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF4_A = GIF7_A;
#[doc = "Reader of field `GIF4`"]
pub type GIF4_R = crate::R<bool, GIF7_A>;
#[doc = "Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF3_A = TEIF7_A;
#[doc = "Reader of field `TEIF3`"]
pub type TEIF3_R = crate::R<bool, TEIF7_A>;
#[doc = "Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF3_A = HTIF7_A;
#[doc = "Reader of field `HTIF3`"]
pub type HTIF3_R = crate::R<bool, HTIF7_A>;
#[doc = "Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF3_A = TCIF7_A;
#[doc = "Reader of field `TCIF3`"]
pub type TCIF3_R = crate::R<bool, TCIF7_A>;
#[doc = "Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF3_A = GIF7_A;
#[doc = "Reader of field `GIF3`"]
pub type GIF3_R = crate::R<bool, GIF7_A>;
#[doc = "Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF2_A = TEIF7_A;
#[doc = "Reader of field `TEIF2`"]
pub type TEIF2_R = crate::R<bool, TEIF7_A>;
#[doc = "Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF2_A = HTIF7_A;
#[doc = "Reader of field `HTIF2`"]
pub type HTIF2_R = crate::R<bool, HTIF7_A>;
#[doc = "Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF2_A = TCIF7_A;
#[doc = "Reader of field `TCIF2`"]
pub type TCIF2_R = crate::R<bool, TCIF7_A>;
#[doc = "Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF2_A = GIF7_A;
#[doc = "Reader of field `GIF2`"]
pub type GIF2_R = crate::R<bool, GIF7_A>;
#[doc = "Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF1_A = TEIF7_A;
#[doc = "Reader of field `TEIF1`"]
pub type TEIF1_R = crate::R<bool, TEIF7_A>;
#[doc = "Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF1_A = HTIF7_A;
#[doc = "Reader of field `HTIF1`"]
pub type HTIF1_R = crate::R<bool, HTIF7_A>;
#[doc = "Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF1_A = TCIF7_A;
#[doc = "Reader of field `TCIF1`"]
pub type TCIF1_R = crate::R<bool, TCIF7_A>;
#[doc = "Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF1_A = GIF7_A;
#[doc = "Reader of field `GIF1`"]
pub type GIF1_R = crate::R<bool, GIF7_A>;
impl R {
    #[doc = "Bit 27 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn teif7(&self) -> TEIF7_R {
        TEIF7_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn htif7(&self) -> HTIF7_R {
        HTIF7_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn tcif7(&self) -> TCIF7_R {
        TCIF7_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn gif7(&self) -> GIF7_R {
        GIF7_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn teif6(&self) -> TEIF6_R {
        TEIF6_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn htif6(&self) -> HTIF6_R {
        HTIF6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF6_R {
        TCIF6_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn gif6(&self) -> GIF6_R {
        GIF6_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn teif5(&self) -> TEIF5_R {
        TEIF5_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn htif5(&self) -> HTIF5_R {
        HTIF5_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn gif5(&self) -> GIF5_R {
        GIF5_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn teif4(&self) -> TEIF4_R {
        TEIF4_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn htif4(&self) -> HTIF4_R {
        HTIF4_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new((self.bits & 0x01) != 0)
    }
}
