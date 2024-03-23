#[doc = "Register `GPIO_IDR` reader"]
pub type R = crate::R<GPIO_IDRrs>;
#[doc = "Field `ID0` reader - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type ID0_R = crate::BitReader;
#[doc = "Field `ID1` reader - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type ID1_R = crate::BitReader;
#[doc = "Field `ID2` reader - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type ID2_R = crate::BitReader;
#[doc = "Field `ID3` reader - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type ID3_R = crate::BitReader;
#[doc = "Field `ID4` reader - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type ID4_R = crate::BitReader;
#[doc = "Field `ID5` reader - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type ID5_R = crate::BitReader;
#[doc = "Field `ID6` reader - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type ID6_R = crate::BitReader;
#[doc = "Field `ID7` reader - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type ID7_R = crate::BitReader;
#[doc = "Field `ID8` reader - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type ID8_R = crate::BitReader;
#[doc = "Field `ID9` reader - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type ID9_R = crate::BitReader;
#[doc = "Field `ID10` reader - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type ID10_R = crate::BitReader;
#[doc = "Field `ID11` reader - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type ID11_R = crate::BitReader;
#[doc = "Field `ID12` reader - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type ID12_R = crate::BitReader;
#[doc = "Field `ID13` reader - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type ID13_R = crate::BitReader;
#[doc = "Field `ID14` reader - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type ID14_R = crate::BitReader;
#[doc = "Field `ID15` reader - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
pub type ID15_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn id0(&self) -> ID0_R {
        ID0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn id1(&self) -> ID1_R {
        ID1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn id2(&self) -> ID2_R {
        ID2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn id3(&self) -> ID3_R {
        ID3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn id4(&self) -> ID4_R {
        ID4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn id5(&self) -> ID5_R {
        ID5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn id6(&self) -> ID6_R {
        ID6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn id7(&self) -> ID7_R {
        ID7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn id8(&self) -> ID8_R {
        ID8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn id9(&self) -> ID9_R {
        ID9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn id10(&self) -> ID10_R {
        ID10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn id11(&self) -> ID11_R {
        ID11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn id12(&self) -> ID12_R {
        ID12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn id13(&self) -> ID13_R {
        ID13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn id14(&self) -> ID14_R {
        ID14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port x input data I/O pin y These bits are read-only. They contain the input value of the corresponding I/O port. Note: This bit is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn id15(&self) -> ID15_R {
        ID15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "GPIO port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_idr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_IDRrs;
impl crate::RegisterSpec for GPIO_IDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_idr::R`](R) reader structure"]
impl crate::Readable for GPIO_IDRrs {}
#[doc = "`reset()` method sets GPIO_IDR to value 0"]
impl crate::Resettable for GPIO_IDRrs {
    const RESET_VALUE: u32 = 0;
}
