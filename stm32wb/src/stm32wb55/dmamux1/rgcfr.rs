#[doc = "Register `RGCFR` reader"]
pub type R = crate::R<RGCFRrs>;
#[doc = "Field `CSOF0` reader - Generator Clear Overrun Flag 0"]
pub type CSOF0_R = crate::BitReader;
#[doc = "Field `CSOF1` reader - Generator Clear Overrun Flag 1"]
pub type CSOF1_R = crate::BitReader;
#[doc = "Field `CSOF2` reader - Generator Clear Overrun Flag 2"]
pub type CSOF2_R = crate::BitReader;
#[doc = "Field `CSOF3` reader - Generator Clear Overrun Flag 3"]
pub type CSOF3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Generator Clear Overrun Flag 0"]
    #[inline(always)]
    pub fn csof0(&self) -> CSOF0_R {
        CSOF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Generator Clear Overrun Flag 1"]
    #[inline(always)]
    pub fn csof1(&self) -> CSOF1_R {
        CSOF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Generator Clear Overrun Flag 2"]
    #[inline(always)]
    pub fn csof2(&self) -> CSOF2_R {
        CSOF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Generator Clear Overrun Flag 3"]
    #[inline(always)]
    pub fn csof3(&self) -> CSOF3_R {
        CSOF3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "DMA Request Generator Clear Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgcfr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RGCFRrs;
impl crate::RegisterSpec for RGCFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgcfr::R`](R) reader structure"]
impl crate::Readable for RGCFRrs {}
#[doc = "`reset()` method sets RGCFR to value 0"]
impl crate::Resettable for RGCFRrs {
    const RESET_VALUE: u32 = 0;
}
