#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Field `HSYNC` reader - HSYNC"]
pub type HSYNC_R = crate::BitReader;
#[doc = "Field `VSYNC` reader - VSYNC"]
pub type VSYNC_R = crate::BitReader;
#[doc = "Field `FNE` reader - FIFO not empty"]
pub type FNE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - HSYNC"]
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VSYNC"]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO not empty"]
    #[inline(always)]
    pub fn fne(&self) -> FNE_R {
        FNE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
