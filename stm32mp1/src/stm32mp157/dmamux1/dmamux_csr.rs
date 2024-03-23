#[doc = "Register `DMAMUX_CSR` reader"]
pub type R = crate::R<DMAMUX_CSRrs>;
#[doc = "Field `SOF0` reader - SOF0"]
pub type SOF0_R = crate::BitReader;
#[doc = "Field `SOF1` reader - SOF1"]
pub type SOF1_R = crate::BitReader;
#[doc = "Field `SOF2` reader - SOF2"]
pub type SOF2_R = crate::BitReader;
#[doc = "Field `SOF3` reader - SOF3"]
pub type SOF3_R = crate::BitReader;
#[doc = "Field `SOF4` reader - SOF4"]
pub type SOF4_R = crate::BitReader;
#[doc = "Field `SOF5` reader - SOF5"]
pub type SOF5_R = crate::BitReader;
#[doc = "Field `SOF6` reader - SOF6"]
pub type SOF6_R = crate::BitReader;
#[doc = "Field `SOF7` reader - SOF7"]
pub type SOF7_R = crate::BitReader;
#[doc = "Field `SOF8` reader - SOF8"]
pub type SOF8_R = crate::BitReader;
#[doc = "Field `SOF9` reader - SOF9"]
pub type SOF9_R = crate::BitReader;
#[doc = "Field `SOF10` reader - SOF10"]
pub type SOF10_R = crate::BitReader;
#[doc = "Field `SOF11` reader - SOF11"]
pub type SOF11_R = crate::BitReader;
#[doc = "Field `SOF12` reader - SOF12"]
pub type SOF12_R = crate::BitReader;
#[doc = "Field `SOF13` reader - SOF13"]
pub type SOF13_R = crate::BitReader;
#[doc = "Field `SOF14` reader - SOF14"]
pub type SOF14_R = crate::BitReader;
#[doc = "Field `SOF15` reader - SOF15"]
pub type SOF15_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SOF0"]
    #[inline(always)]
    pub fn sof0(&self) -> SOF0_R {
        SOF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SOF1"]
    #[inline(always)]
    pub fn sof1(&self) -> SOF1_R {
        SOF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SOF2"]
    #[inline(always)]
    pub fn sof2(&self) -> SOF2_R {
        SOF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SOF3"]
    #[inline(always)]
    pub fn sof3(&self) -> SOF3_R {
        SOF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SOF4"]
    #[inline(always)]
    pub fn sof4(&self) -> SOF4_R {
        SOF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SOF5"]
    #[inline(always)]
    pub fn sof5(&self) -> SOF5_R {
        SOF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SOF6"]
    #[inline(always)]
    pub fn sof6(&self) -> SOF6_R {
        SOF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SOF7"]
    #[inline(always)]
    pub fn sof7(&self) -> SOF7_R {
        SOF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SOF8"]
    #[inline(always)]
    pub fn sof8(&self) -> SOF8_R {
        SOF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SOF9"]
    #[inline(always)]
    pub fn sof9(&self) -> SOF9_R {
        SOF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SOF10"]
    #[inline(always)]
    pub fn sof10(&self) -> SOF10_R {
        SOF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SOF11"]
    #[inline(always)]
    pub fn sof11(&self) -> SOF11_R {
        SOF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SOF12"]
    #[inline(always)]
    pub fn sof12(&self) -> SOF12_R {
        SOF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SOF13"]
    #[inline(always)]
    pub fn sof13(&self) -> SOF13_R {
        SOF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SOF14"]
    #[inline(always)]
    pub fn sof14(&self) -> SOF14_R {
        SOF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SOF15"]
    #[inline(always)]
    pub fn sof15(&self) -> SOF15_R {
        SOF15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "DMAMUX request line multiplexer interrupt channel status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_csr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAMUX_CSRrs;
impl crate::RegisterSpec for DMAMUX_CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamux_csr::R`](R) reader structure"]
impl crate::Readable for DMAMUX_CSRrs {}
#[doc = "`reset()` method sets DMAMUX_CSR to value 0"]
impl crate::Resettable for DMAMUX_CSRrs {
    const RESET_VALUE: u32 = 0;
}
