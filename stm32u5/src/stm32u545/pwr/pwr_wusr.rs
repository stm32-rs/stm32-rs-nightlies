#[doc = "Register `PWR_WUSR` reader"]
pub type R = crate::R<PWR_WUSRrs>;
#[doc = "Field `WUF1` reader - Wakeup flag 1 This bit is set when a wakeup event is detected on WKUP1 pin. This bit is cleared by writing 1 in the CWUF1 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN1 = 0."]
pub type WUF1_R = crate::BitReader;
#[doc = "Field `WUF2` reader - Wakeup flag 2 This bit is set when a wakeup event is detected on WKUP2 pin. This bit is cleared by writing 1 in the CWUF2 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN2 = 0."]
pub type WUF2_R = crate::BitReader;
#[doc = "Field `WUF3` reader - Wakeup flag 3 This bit is set when a wakeup event is detected on WKUP3 pin. This bit is cleared by writing 1 in the CWUF3 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN3 = 0."]
pub type WUF3_R = crate::BitReader;
#[doc = "Field `WUF4` reader - Wakeup flag 4 This bit is set when a wakeup event is detected on WKUP4 pin. This bit is cleared by writing 1 in the CWUF4 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN4 = 0."]
pub type WUF4_R = crate::BitReader;
#[doc = "Field `WUF5` reader - Wakeup flag 5 This bit is set when a wakeup event is detected on WKUP5 pin. This bit is cleared by writing 1 in the CWUF5 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN5 = 0."]
pub type WUF5_R = crate::BitReader;
#[doc = "Field `WUF6` reader - Wakeup flag 6 This bit is set when a wakeup event is detected on WKUP6 pin. This bit is cleared by writing 1 in the CWUF6 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN6 = 0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
pub type WUF6_R = crate::BitReader;
#[doc = "Field `WUF7` reader - Wakeup flag 7 This bit is set when a wakeup event is detected on WKUP7 pin. This bit is cleared by writing 1 in the CWUF7 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN7 = 0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
pub type WUF7_R = crate::BitReader;
#[doc = "Field `WUF8` reader - Wakeup flag 8 This bit is set when a wakeup event is detected on WKUP8 pin. This bit is cleared by writing 1 in the CWUF8 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN8 = 0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
pub type WUF8_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Wakeup flag 1 This bit is set when a wakeup event is detected on WKUP1 pin. This bit is cleared by writing 1 in the CWUF1 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN1 = 0."]
    #[inline(always)]
    pub fn wuf1(&self) -> WUF1_R {
        WUF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup flag 2 This bit is set when a wakeup event is detected on WKUP2 pin. This bit is cleared by writing 1 in the CWUF2 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN2 = 0."]
    #[inline(always)]
    pub fn wuf2(&self) -> WUF2_R {
        WUF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup flag 3 This bit is set when a wakeup event is detected on WKUP3 pin. This bit is cleared by writing 1 in the CWUF3 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN3 = 0."]
    #[inline(always)]
    pub fn wuf3(&self) -> WUF3_R {
        WUF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup flag 4 This bit is set when a wakeup event is detected on WKUP4 pin. This bit is cleared by writing 1 in the CWUF4 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN4 = 0."]
    #[inline(always)]
    pub fn wuf4(&self) -> WUF4_R {
        WUF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup flag 5 This bit is set when a wakeup event is detected on WKUP5 pin. This bit is cleared by writing 1 in the CWUF5 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN5 = 0."]
    #[inline(always)]
    pub fn wuf5(&self) -> WUF5_R {
        WUF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wakeup flag 6 This bit is set when a wakeup event is detected on WKUP6 pin. This bit is cleared by writing 1 in the CWUF6 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN6 = 0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
    #[inline(always)]
    pub fn wuf6(&self) -> WUF6_R {
        WUF6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wakeup flag 7 This bit is set when a wakeup event is detected on WKUP7 pin. This bit is cleared by writing 1 in the CWUF7 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN7 = 0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
    #[inline(always)]
    pub fn wuf7(&self) -> WUF7_R {
        WUF7_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wakeup flag 8 This bit is set when a wakeup event is detected on WKUP8 pin. This bit is cleared by writing 1 in the CWUF8 bit of PWR_WUSCR when WUSEL ≠ 11, or by hardware when WUPEN8 = 0. If WUSEL = 11, this bit is cleared by hardware when all internal wakeup source are cleared."]
    #[inline(always)]
    pub fn wuf8(&self) -> WUF8_R {
        WUF8_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "PWR wakeup status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_wusr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_WUSRrs;
impl crate::RegisterSpec for PWR_WUSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_wusr::R`](R) reader structure"]
impl crate::Readable for PWR_WUSRrs {}
#[doc = "`reset()` method sets PWR_WUSR to value 0"]
impl crate::Resettable for PWR_WUSRrs {
    const RESET_VALUE: u32 = 0;
}
