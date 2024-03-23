#[doc = "Register `RGSR` reader"]
pub type R = crate::R<RGSRrs>;
#[doc = "Field `OF0` reader - Generator Overrun Flag 0"]
pub type OF0_R = crate::BitReader;
#[doc = "Field `OF1` reader - Generator Overrun Flag 1"]
pub type OF1_R = crate::BitReader;
#[doc = "Field `OF2` reader - Generator Overrun Flag 2"]
pub type OF2_R = crate::BitReader;
#[doc = "Field `OF3` reader - Generator Overrun Flag 3"]
pub type OF3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Generator Overrun Flag 0"]
    #[inline(always)]
    pub fn of0(&self) -> OF0_R {
        OF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Generator Overrun Flag 1"]
    #[inline(always)]
    pub fn of1(&self) -> OF1_R {
        OF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Generator Overrun Flag 2"]
    #[inline(always)]
    pub fn of2(&self) -> OF2_R {
        OF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Generator Overrun Flag 3"]
    #[inline(always)]
    pub fn of3(&self) -> OF3_R {
        OF3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "DMA Request Generator Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RGSRrs;
impl crate::RegisterSpec for RGSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgsr::R`](R) reader structure"]
impl crate::Readable for RGSRrs {}
#[doc = "`reset()` method sets RGSR to value 0"]
impl crate::Resettable for RGSRrs {
    const RESET_VALUE: u32 = 0;
}
