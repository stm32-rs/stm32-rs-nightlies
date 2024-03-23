#[doc = "Register `DMAMUX_RGSR` reader"]
pub type R = crate::R<DMAMUX_RGSRrs>;
#[doc = "Field `OF0` reader - OF0"]
pub type OF0_R = crate::BitReader;
#[doc = "Field `OF1` reader - OF1"]
pub type OF1_R = crate::BitReader;
#[doc = "Field `OF2` reader - OF2"]
pub type OF2_R = crate::BitReader;
#[doc = "Field `OF3` reader - OF3"]
pub type OF3_R = crate::BitReader;
#[doc = "Field `OF4` reader - OF4"]
pub type OF4_R = crate::BitReader;
#[doc = "Field `OF5` reader - OF5"]
pub type OF5_R = crate::BitReader;
#[doc = "Field `OF6` reader - OF6"]
pub type OF6_R = crate::BitReader;
#[doc = "Field `OF7` reader - OF7"]
pub type OF7_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - OF0"]
    #[inline(always)]
    pub fn of0(&self) -> OF0_R {
        OF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OF1"]
    #[inline(always)]
    pub fn of1(&self) -> OF1_R {
        OF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OF2"]
    #[inline(always)]
    pub fn of2(&self) -> OF2_R {
        OF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OF3"]
    #[inline(always)]
    pub fn of3(&self) -> OF3_R {
        OF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OF4"]
    #[inline(always)]
    pub fn of4(&self) -> OF4_R {
        OF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OF5"]
    #[inline(always)]
    pub fn of5(&self) -> OF5_R {
        OF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OF6"]
    #[inline(always)]
    pub fn of6(&self) -> OF6_R {
        OF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - OF7"]
    #[inline(always)]
    pub fn of7(&self) -> OF7_R {
        OF7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "DMAMUX request generator interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_rgsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAMUX_RGSRrs;
impl crate::RegisterSpec for DMAMUX_RGSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamux_rgsr::R`](R) reader structure"]
impl crate::Readable for DMAMUX_RGSRrs {}
#[doc = "`reset()` method sets DMAMUX_RGSR to value 0"]
impl crate::Resettable for DMAMUX_RGSRrs {
    const RESET_VALUE: u32 = 0;
}
