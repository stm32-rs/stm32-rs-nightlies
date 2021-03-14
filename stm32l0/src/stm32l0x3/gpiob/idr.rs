#[doc = "Reader of register IDR"]
pub type R = crate::R<u32, super::IDR>;
#[doc = "Port input data bit (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ID15_A {
    #[doc = "1: Input is logic high"]
    HIGH = 1,
    #[doc = "0: Input is logic low"]
    LOW = 0,
}
impl From<ID15_A> for bool {
    #[inline(always)]
    fn from(variant: ID15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ID15`"]
pub type ID15_R = crate::R<bool, ID15_A>;
impl ID15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ID15_A {
        match self.bits {
            true => ID15_A::HIGH,
            false => ID15_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == ID15_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == ID15_A::LOW
    }
}
#[doc = "Port input data bit (y = 0..15)"]
pub type ID14_A = ID15_A;
#[doc = "Reader of field `ID14`"]
pub type ID14_R = crate::R<bool, ID15_A>;
#[doc = "Port input data bit (y = 0..15)"]
pub type ID13_A = ID15_A;
#[doc = "Reader of field `ID13`"]
pub type ID13_R = crate::R<bool, ID15_A>;
#[doc = "Port input data bit (y = 0..15)"]
pub type ID12_A = ID15_A;
#[doc = "Reader of field `ID12`"]
pub type ID12_R = crate::R<bool, ID15_A>;
#[doc = "Port input data bit (y = 0..15)"]
pub type ID11_A = ID15_A;
#[doc = "Reader of field `ID11`"]
pub type ID11_R = crate::R<bool, ID15_A>;
#[doc = "Port input data bit (y = 0..15)"]
pub type ID10_A = ID15_A;
#[doc = "Reader of field `ID10`"]
pub type ID10_R = crate::R<bool, ID15_A>;
#[doc = "Port input data bit (y = 0..15)"]
pub type ID9_A = ID15_A;
#[doc = "Reader of field `ID9`"]
pub type ID9_R = crate::R<bool, ID15_A>;
#[doc = "Port input data bit (y = 0..15)"]
pub type ID8_A = ID15_A;
#[doc = "Reader of field `ID8`"]
pub type ID8_R = crate::R<bool, ID15_A>;
#[doc = "Port input data bit (y = 0..15)"]
pub type ID7_A = ID15_A;
#[doc = "Reader of field `ID7`"]
pub type ID7_R = crate::R<bool, ID15_A>;
#[doc = "Port input data bit (y = 0..15)"]
pub type ID6_A = ID15_A;
#[doc = "Reader of field `ID6`"]
pub type ID6_R = crate::R<bool, ID15_A>;
#[doc = "Port input data bit (y = 0..15)"]
pub type ID5_A = ID15_A;
#[doc = "Reader of field `ID5`"]
pub type ID5_R = crate::R<bool, ID15_A>;
#[doc = "Port input data bit (y = 0..15)"]
pub type ID4_A = ID15_A;
#[doc = "Reader of field `ID4`"]
pub type ID4_R = crate::R<bool, ID15_A>;
#[doc = "Port input data bit (y = 0..15)"]
pub type ID3_A = ID15_A;
#[doc = "Reader of field `ID3`"]
pub type ID3_R = crate::R<bool, ID15_A>;
#[doc = "Port input data bit (y = 0..15)"]
pub type ID2_A = ID15_A;
#[doc = "Reader of field `ID2`"]
pub type ID2_R = crate::R<bool, ID15_A>;
#[doc = "Port input data bit (y = 0..15)"]
pub type ID1_A = ID15_A;
#[doc = "Reader of field `ID1`"]
pub type ID1_R = crate::R<bool, ID15_A>;
#[doc = "Port input data bit (y = 0..15)"]
pub type ID0_A = ID15_A;
#[doc = "Reader of field `ID0`"]
pub type ID0_R = crate::R<bool, ID15_A>;
impl R {
    #[doc = "Bit 15 - Port input data bit (y = 0..15)"]
    #[inline(always)]
    pub fn id15(&self) -> ID15_R {
        ID15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port input data bit (y = 0..15)"]
    #[inline(always)]
    pub fn id14(&self) -> ID14_R {
        ID14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port input data bit (y = 0..15)"]
    #[inline(always)]
    pub fn id13(&self) -> ID13_R {
        ID13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port input data bit (y = 0..15)"]
    #[inline(always)]
    pub fn id12(&self) -> ID12_R {
        ID12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port input data bit (y = 0..15)"]
    #[inline(always)]
    pub fn id11(&self) -> ID11_R {
        ID11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port input data bit (y = 0..15)"]
    #[inline(always)]
    pub fn id10(&self) -> ID10_R {
        ID10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port input data bit (y = 0..15)"]
    #[inline(always)]
    pub fn id9(&self) -> ID9_R {
        ID9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port input data bit (y = 0..15)"]
    #[inline(always)]
    pub fn id8(&self) -> ID8_R {
        ID8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port input data bit (y = 0..15)"]
    #[inline(always)]
    pub fn id7(&self) -> ID7_R {
        ID7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port input data bit (y = 0..15)"]
    #[inline(always)]
    pub fn id6(&self) -> ID6_R {
        ID6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port input data bit (y = 0..15)"]
    #[inline(always)]
    pub fn id5(&self) -> ID5_R {
        ID5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port input data bit (y = 0..15)"]
    #[inline(always)]
    pub fn id4(&self) -> ID4_R {
        ID4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port input data bit (y = 0..15)"]
    #[inline(always)]
    pub fn id3(&self) -> ID3_R {
        ID3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port input data bit (y = 0..15)"]
    #[inline(always)]
    pub fn id2(&self) -> ID2_R {
        ID2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port input data bit (y = 0..15)"]
    #[inline(always)]
    pub fn id1(&self) -> ID1_R {
        ID1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Port input data bit (y = 0..15)"]
    #[inline(always)]
    pub fn id0(&self) -> ID0_R {
        ID0_R::new((self.bits & 0x01) != 0)
    }
}
