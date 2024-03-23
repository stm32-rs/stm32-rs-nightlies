#[doc = "Register `LPGPIO_IDR` reader"]
pub type R = crate::R<LPGPIO_IDRrs>;
#[doc = "Field `ID0` reader - ID0"]
pub type ID0_R = crate::BitReader;
#[doc = "Field `ID1` reader - ID1"]
pub type ID1_R = crate::BitReader;
#[doc = "Field `ID2` reader - ID2"]
pub type ID2_R = crate::BitReader;
#[doc = "Field `ID3` reader - ID3"]
pub type ID3_R = crate::BitReader;
#[doc = "Field `ID4` reader - ID4"]
pub type ID4_R = crate::BitReader;
#[doc = "Field `ID5` reader - ID5"]
pub type ID5_R = crate::BitReader;
#[doc = "Field `ID6` reader - ID6"]
pub type ID6_R = crate::BitReader;
#[doc = "Field `ID7` reader - ID7"]
pub type ID7_R = crate::BitReader;
#[doc = "Field `ID8` reader - ID8"]
pub type ID8_R = crate::BitReader;
#[doc = "Field `ID9` reader - ID9"]
pub type ID9_R = crate::BitReader;
#[doc = "Field `ID10` reader - ID10"]
pub type ID10_R = crate::BitReader;
#[doc = "Field `ID11` reader - ID11"]
pub type ID11_R = crate::BitReader;
#[doc = "Field `ID12` reader - ID12"]
pub type ID12_R = crate::BitReader;
#[doc = "Field `ID13` reader - ID13"]
pub type ID13_R = crate::BitReader;
#[doc = "Field `ID14` reader - ID14"]
pub type ID14_R = crate::BitReader;
#[doc = "Field `ID15` reader - ID15"]
pub type ID15_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ID0"]
    #[inline(always)]
    pub fn id0(&self) -> ID0_R {
        ID0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ID1"]
    #[inline(always)]
    pub fn id1(&self) -> ID1_R {
        ID1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ID2"]
    #[inline(always)]
    pub fn id2(&self) -> ID2_R {
        ID2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ID3"]
    #[inline(always)]
    pub fn id3(&self) -> ID3_R {
        ID3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ID4"]
    #[inline(always)]
    pub fn id4(&self) -> ID4_R {
        ID4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ID5"]
    #[inline(always)]
    pub fn id5(&self) -> ID5_R {
        ID5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ID6"]
    #[inline(always)]
    pub fn id6(&self) -> ID6_R {
        ID6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ID7"]
    #[inline(always)]
    pub fn id7(&self) -> ID7_R {
        ID7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ID8"]
    #[inline(always)]
    pub fn id8(&self) -> ID8_R {
        ID8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ID9"]
    #[inline(always)]
    pub fn id9(&self) -> ID9_R {
        ID9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ID10"]
    #[inline(always)]
    pub fn id10(&self) -> ID10_R {
        ID10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ID11"]
    #[inline(always)]
    pub fn id11(&self) -> ID11_R {
        ID11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ID12"]
    #[inline(always)]
    pub fn id12(&self) -> ID12_R {
        ID12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ID13"]
    #[inline(always)]
    pub fn id13(&self) -> ID13_R {
        ID13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ID14"]
    #[inline(always)]
    pub fn id14(&self) -> ID14_R {
        ID14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ID15"]
    #[inline(always)]
    pub fn id15(&self) -> ID15_R {
        ID15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "LPGPIO port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpgpio_idr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPGPIO_IDRrs;
impl crate::RegisterSpec for LPGPIO_IDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpgpio_idr::R`](R) reader structure"]
impl crate::Readable for LPGPIO_IDRrs {}
#[doc = "`reset()` method sets LPGPIO_IDR to value 0"]
impl crate::Resettable for LPGPIO_IDRrs {
    const RESET_VALUE: u32 = 0;
}
