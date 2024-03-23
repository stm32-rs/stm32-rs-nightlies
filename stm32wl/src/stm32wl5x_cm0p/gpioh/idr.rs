#[doc = "Register `IDR` reader"]
pub type R = crate::R<IDRrs>;
#[doc = "Port input data (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDR3 {
    #[doc = "0: Input is logic low"]
    Low = 0,
    #[doc = "1: Input is logic high"]
    High = 1,
}
impl From<IDR3> for bool {
    #[inline(always)]
    fn from(variant: IDR3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDR3` reader - Port input data (y = 0..15)"]
pub type IDR3_R = crate::BitReader<IDR3>;
impl IDR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IDR3 {
        match self.bits {
            false => IDR3::Low,
            true => IDR3::High,
        }
    }
    #[doc = "Input is logic low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDR3::Low
    }
    #[doc = "Input is logic high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDR3::High
    }
}
impl R {
    #[doc = "Bit 3 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr3(&self) -> IDR3_R {
        IDR3_R::new(((self.bits >> 3) & 1) != 0)
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
