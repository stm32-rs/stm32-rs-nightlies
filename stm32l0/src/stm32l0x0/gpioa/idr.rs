#[doc = "Register `IDR` reader"]
pub type R = crate::R<IDRrs>;
#[doc = "Port input data pin %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ID0 {
    #[doc = "0: Input is logic low"]
    Low = 0,
    #[doc = "1: Input is logic high"]
    High = 1,
}
impl From<ID0> for bool {
    #[inline(always)]
    fn from(variant: ID0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ID(0-15)` reader - Port input data pin %s"]
pub type ID_R = crate::BitReader<ID0>;
impl ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ID0 {
        match self.bits {
            false => ID0::Low,
            true => ID0::High,
        }
    }
    #[doc = "Input is logic low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == ID0::Low
    }
    #[doc = "Input is logic high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == ID0::High
    }
}
impl R {
    #[doc = "Port input data pin (0-15)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `ID0` field"]
    #[inline(always)]
    pub fn id(&self, n: u8) -> ID_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        ID_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Port input data pin (0-15)"]
    #[inline(always)]
    pub fn id_iter(&self) -> impl Iterator<Item = ID_R> + '_ {
        (0..16).map(move |n| ID_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Port input data pin 0"]
    #[inline(always)]
    pub fn id0(&self) -> ID_R {
        ID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port input data pin 1"]
    #[inline(always)]
    pub fn id1(&self) -> ID_R {
        ID_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port input data pin 2"]
    #[inline(always)]
    pub fn id2(&self) -> ID_R {
        ID_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port input data pin 3"]
    #[inline(always)]
    pub fn id3(&self) -> ID_R {
        ID_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port input data pin 4"]
    #[inline(always)]
    pub fn id4(&self) -> ID_R {
        ID_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port input data pin 5"]
    #[inline(always)]
    pub fn id5(&self) -> ID_R {
        ID_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port input data pin 6"]
    #[inline(always)]
    pub fn id6(&self) -> ID_R {
        ID_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port input data pin 7"]
    #[inline(always)]
    pub fn id7(&self) -> ID_R {
        ID_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port input data pin 8"]
    #[inline(always)]
    pub fn id8(&self) -> ID_R {
        ID_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port input data pin 9"]
    #[inline(always)]
    pub fn id9(&self) -> ID_R {
        ID_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port input data pin 10"]
    #[inline(always)]
    pub fn id10(&self) -> ID_R {
        ID_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port input data pin 11"]
    #[inline(always)]
    pub fn id11(&self) -> ID_R {
        ID_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port input data pin 12"]
    #[inline(always)]
    pub fn id12(&self) -> ID_R {
        ID_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port input data pin 13"]
    #[inline(always)]
    pub fn id13(&self) -> ID_R {
        ID_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port input data pin 14"]
    #[inline(always)]
    pub fn id14(&self) -> ID_R {
        ID_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port input data pin 15"]
    #[inline(always)]
    pub fn id15(&self) -> ID_R {
        ID_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "GPIO port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDRrs;
impl crate::RegisterSpec for IDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idr::R`](R) reader structure"]
impl crate::Readable for IDRrs {}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDRrs {
    const RESET_VALUE: u32 = 0;
}
