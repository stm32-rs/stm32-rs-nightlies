#[doc = "Register `SR1` reader"]
pub type R = crate::R<SR1rs>;
#[doc = "Field `WUF1` reader - Wakeup flag 1"]
pub type WUF1_R = crate::BitReader;
#[doc = "Field `WUF2` reader - Wakeup flag 2"]
pub type WUF2_R = crate::BitReader;
#[doc = "Field `WUF3` reader - Wakeup flag 3"]
pub type WUF3_R = crate::BitReader;
#[doc = "Field `WUF4` reader - Wakeup flag 4"]
pub type WUF4_R = crate::BitReader;
#[doc = "Field `WUF5` reader - Wakeup flag 5"]
pub type WUF5_R = crate::BitReader;
#[doc = "Field `WUF6` reader - Wakeup flag 6"]
pub type WUF6_R = crate::BitReader;
#[doc = "Field `SBF` reader - Standby flag"]
pub type SBF_R = crate::BitReader;
#[doc = "Field `WUFI` reader - Wakeup flag internal"]
pub type WUFI_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Wakeup flag 1"]
    #[inline(always)]
    pub fn wuf1(&self) -> WUF1_R {
        WUF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup flag 2"]
    #[inline(always)]
    pub fn wuf2(&self) -> WUF2_R {
        WUF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup flag 3"]
    #[inline(always)]
    pub fn wuf3(&self) -> WUF3_R {
        WUF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup flag 4"]
    #[inline(always)]
    pub fn wuf4(&self) -> WUF4_R {
        WUF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup flag 5"]
    #[inline(always)]
    pub fn wuf5(&self) -> WUF5_R {
        WUF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wakeup flag 6"]
    #[inline(always)]
    pub fn wuf6(&self) -> WUF6_R {
        WUF6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Standby flag"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - Wakeup flag internal"]
    #[inline(always)]
    pub fn wufi(&self) -> WUFI_R {
        WUFI_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Power status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR1rs;
impl crate::RegisterSpec for SR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr1::R`](R) reader structure"]
impl crate::Readable for SR1rs {}
#[doc = "`reset()` method sets SR1 to value 0"]
impl crate::Resettable for SR1rs {
    const RESET_VALUE: u32 = 0;
}
