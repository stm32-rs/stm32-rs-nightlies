#[doc = "Reader of register ISR"]
pub type R = crate::R<u32, super::ISR>;
#[doc = "Channel 1 Global interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF1_A {
    #[doc = "0: No transfer error, half event, complete event"]
    NOEVENT = 0,
    #[doc = "1: A transfer error, half event or complete event has occured"]
    EVENT = 1,
}
impl From<GIF1_A> for bool {
    #[inline(always)]
    fn from(variant: GIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GIF1`"]
pub type GIF1_R = crate::R<bool, GIF1_A>;
impl GIF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIF1_A {
        match self.bits {
            false => GIF1_A::NOEVENT,
            true => GIF1_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == GIF1_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == GIF1_A::EVENT
    }
}
#[doc = "Channel 1 Transfer Complete flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF1_A {
    #[doc = "0: No transfer complete event"]
    NOTCOMPLETE = 0,
    #[doc = "1: A transfer complete event has occured"]
    COMPLETE = 1,
}
impl From<TCIF1_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCIF1`"]
pub type TCIF1_R = crate::R<bool, TCIF1_A>;
impl TCIF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF1_A {
        match self.bits {
            false => TCIF1_A::NOTCOMPLETE,
            true => TCIF1_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TCIF1_A::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCIF1_A::COMPLETE
    }
}
#[doc = "Channel 1 Half Transfer Complete flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTIF1_A {
    #[doc = "0: No half transfer event"]
    NOTHALF = 0,
    #[doc = "1: A half transfer event has occured"]
    HALF = 1,
}
impl From<HTIF1_A> for bool {
    #[inline(always)]
    fn from(variant: HTIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HTIF1`"]
pub type HTIF1_R = crate::R<bool, HTIF1_A>;
impl HTIF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTIF1_A {
        match self.bits {
            false => HTIF1_A::NOTHALF,
            true => HTIF1_A::HALF,
        }
    }
    #[doc = "Checks if the value of the field is `NOTHALF`"]
    #[inline(always)]
    pub fn is_not_half(&self) -> bool {
        *self == HTIF1_A::NOTHALF
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == HTIF1_A::HALF
    }
}
#[doc = "Channel 1 Transfer Error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF1_A {
    #[doc = "0: No transfer error"]
    NOERROR = 0,
    #[doc = "1: A transfer error has occured"]
    ERROR = 1,
}
impl From<TEIF1_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEIF1`"]
pub type TEIF1_R = crate::R<bool, TEIF1_A>;
impl TEIF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF1_A {
        match self.bits {
            false => TEIF1_A::NOERROR,
            true => TEIF1_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TEIF1_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TEIF1_A::ERROR
    }
}
#[doc = "Channel 2 Global interrupt flag"]
pub type GIF2_A = GIF1_A;
#[doc = "Reader of field `GIF2`"]
pub type GIF2_R = crate::R<bool, GIF1_A>;
#[doc = "Channel 2 Transfer Complete flag"]
pub type TCIF2_A = TCIF1_A;
#[doc = "Reader of field `TCIF2`"]
pub type TCIF2_R = crate::R<bool, TCIF1_A>;
#[doc = "Channel 2 Half Transfer Complete flag"]
pub type HTIF2_A = HTIF1_A;
#[doc = "Reader of field `HTIF2`"]
pub type HTIF2_R = crate::R<bool, HTIF1_A>;
#[doc = "Channel 2 Transfer Error flag"]
pub type TEIF2_A = TEIF1_A;
#[doc = "Reader of field `TEIF2`"]
pub type TEIF2_R = crate::R<bool, TEIF1_A>;
#[doc = "Channel 3 Global interrupt flag"]
pub type GIF3_A = GIF1_A;
#[doc = "Reader of field `GIF3`"]
pub type GIF3_R = crate::R<bool, GIF1_A>;
#[doc = "Channel 3 Transfer Complete flag"]
pub type TCIF3_A = TCIF1_A;
#[doc = "Reader of field `TCIF3`"]
pub type TCIF3_R = crate::R<bool, TCIF1_A>;
#[doc = "Channel 3 Half Transfer Complete flag"]
pub type HTIF3_A = HTIF1_A;
#[doc = "Reader of field `HTIF3`"]
pub type HTIF3_R = crate::R<bool, HTIF1_A>;
#[doc = "Channel 3 Transfer Error flag"]
pub type TEIF3_A = TEIF1_A;
#[doc = "Reader of field `TEIF3`"]
pub type TEIF3_R = crate::R<bool, TEIF1_A>;
#[doc = "Channel 4 Global interrupt flag"]
pub type GIF4_A = GIF1_A;
#[doc = "Reader of field `GIF4`"]
pub type GIF4_R = crate::R<bool, GIF1_A>;
#[doc = "Channel 4 Transfer Complete flag"]
pub type TCIF4_A = TCIF1_A;
#[doc = "Reader of field `TCIF4`"]
pub type TCIF4_R = crate::R<bool, TCIF1_A>;
#[doc = "Channel 4 Half Transfer Complete flag"]
pub type HTIF4_A = HTIF1_A;
#[doc = "Reader of field `HTIF4`"]
pub type HTIF4_R = crate::R<bool, HTIF1_A>;
#[doc = "Channel 4 Transfer Error flag"]
pub type TEIF4_A = TEIF1_A;
#[doc = "Reader of field `TEIF4`"]
pub type TEIF4_R = crate::R<bool, TEIF1_A>;
#[doc = "Channel 5 Global interrupt flag"]
pub type GIF5_A = GIF1_A;
#[doc = "Reader of field `GIF5`"]
pub type GIF5_R = crate::R<bool, GIF1_A>;
#[doc = "Channel 5 Transfer Complete flag"]
pub type TCIF5_A = TCIF1_A;
#[doc = "Reader of field `TCIF5`"]
pub type TCIF5_R = crate::R<bool, TCIF1_A>;
#[doc = "Channel 5 Half Transfer Complete flag"]
pub type HTIF5_A = HTIF1_A;
#[doc = "Reader of field `HTIF5`"]
pub type HTIF5_R = crate::R<bool, HTIF1_A>;
#[doc = "Channel 5 Transfer Error flag"]
pub type TEIF5_A = TEIF1_A;
#[doc = "Reader of field `TEIF5`"]
pub type TEIF5_R = crate::R<bool, TEIF1_A>;
#[doc = "Channel 6 Global interrupt flag"]
pub type GIF6_A = GIF1_A;
#[doc = "Reader of field `GIF6`"]
pub type GIF6_R = crate::R<bool, GIF1_A>;
#[doc = "Channel 6 Transfer Complete flag"]
pub type TCIF6_A = TCIF1_A;
#[doc = "Reader of field `TCIF6`"]
pub type TCIF6_R = crate::R<bool, TCIF1_A>;
#[doc = "Channel 6 Half Transfer Complete flag"]
pub type HTIF6_A = HTIF1_A;
#[doc = "Reader of field `HTIF6`"]
pub type HTIF6_R = crate::R<bool, HTIF1_A>;
#[doc = "Channel 6 Transfer Error flag"]
pub type TEIF6_A = TEIF1_A;
#[doc = "Reader of field `TEIF6`"]
pub type TEIF6_R = crate::R<bool, TEIF1_A>;
#[doc = "Channel 7 Global interrupt flag"]
pub type GIF7_A = GIF1_A;
#[doc = "Reader of field `GIF7`"]
pub type GIF7_R = crate::R<bool, GIF1_A>;
#[doc = "Channel 7 Transfer Complete flag"]
pub type TCIF7_A = TCIF1_A;
#[doc = "Reader of field `TCIF7`"]
pub type TCIF7_R = crate::R<bool, TCIF1_A>;
#[doc = "Channel 7 Half Transfer Complete flag"]
pub type HTIF7_A = HTIF1_A;
#[doc = "Reader of field `HTIF7`"]
pub type HTIF7_R = crate::R<bool, HTIF1_A>;
#[doc = "Channel 7 Transfer Error flag"]
pub type TEIF7_A = TEIF1_A;
#[doc = "Reader of field `TEIF7`"]
pub type TEIF7_R = crate::R<bool, TEIF1_A>;
impl R {
    #[doc = "Bit 0 - Channel 1 Global interrupt flag"]
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 1 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 1 Transfer Error flag"]
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 2 Global interrupt flag"]
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 2 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 2 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 2 Transfer Error flag"]
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 3 Global interrupt flag"]
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel 3 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel 3 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Transfer Error flag"]
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Global interrupt flag"]
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel 4 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel 4 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif4(&self) -> HTIF4_R {
        HTIF4_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Channel 4 Transfer Error flag"]
    #[inline(always)]
    pub fn teif4(&self) -> TEIF4_R {
        TEIF4_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel 5 Global interrupt flag"]
    #[inline(always)]
    pub fn gif5(&self) -> GIF5_R {
        GIF5_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel 5 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel 5 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif5(&self) -> HTIF5_R {
        HTIF5_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel 5 Transfer Error flag"]
    #[inline(always)]
    pub fn teif5(&self) -> TEIF5_R {
        TEIF5_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Channel 6 Global interrupt flag"]
    #[inline(always)]
    pub fn gif6(&self) -> GIF6_R {
        GIF6_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Channel 6 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF6_R {
        TCIF6_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Channel 6 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif6(&self) -> HTIF6_R {
        HTIF6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Channel 6 Transfer Error flag"]
    #[inline(always)]
    pub fn teif6(&self) -> TEIF6_R {
        TEIF6_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Channel 7 Global interrupt flag"]
    #[inline(always)]
    pub fn gif7(&self) -> GIF7_R {
        GIF7_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Channel 7 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif7(&self) -> TCIF7_R {
        TCIF7_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Channel 7 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif7(&self) -> HTIF7_R {
        HTIF7_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Channel 7 Transfer Error flag"]
    #[inline(always)]
    pub fn teif7(&self) -> TEIF7_R {
        TEIF7_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
