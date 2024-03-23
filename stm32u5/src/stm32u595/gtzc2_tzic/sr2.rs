#[doc = "Register `SR2` reader"]
pub type R = crate::R<SR2rs>;
#[doc = "Field `SYSCFGF` reader - illegal access flag for SYSCFG"]
pub type SYSCFGF_R = crate::BitReader;
#[doc = "Field `RTCF` reader - illegal access flag for RTC"]
pub type RTCF_R = crate::BitReader;
#[doc = "Field `TAMPF` reader - illegal access flag for TAMP"]
pub type TAMPF_R = crate::BitReader;
#[doc = "Field `PWRF` reader - illegal access flag for PWRUSART1F"]
pub type PWRF_R = crate::BitReader;
#[doc = "Field `RCCF` reader - illegal access flag for RCC"]
pub type RCCF_R = crate::BitReader;
#[doc = "Field `LPDMA1F` reader - illegal access flag for LPDMA"]
pub type LPDMA1F_R = crate::BitReader;
#[doc = "Field `EXTIF` reader - illegal access flag for EXTI"]
pub type EXTIF_R = crate::BitReader;
#[doc = "Field `TZSC2F` reader - illegal access flag for GTZC2 TZSC registers"]
pub type TZSC2F_R = crate::BitReader;
#[doc = "Field `TZIC2F` reader - illegal access flag for GTZC2 TZIC registers"]
pub type TZIC2F_R = crate::BitReader;
#[doc = "Field `SRAM4F` reader - illegal access flag for SRAM4"]
pub type SRAM4F_R = crate::BitReader;
#[doc = "Field `MPCBB4_REGF` reader - illegal access flag for MPCBB4 registers"]
pub type MPCBB4_REGF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - illegal access flag for SYSCFG"]
    #[inline(always)]
    pub fn syscfgf(&self) -> SYSCFGF_R {
        SYSCFGF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - illegal access flag for RTC"]
    #[inline(always)]
    pub fn rtcf(&self) -> RTCF_R {
        RTCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - illegal access flag for TAMP"]
    #[inline(always)]
    pub fn tampf(&self) -> TAMPF_R {
        TAMPF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - illegal access flag for PWRUSART1F"]
    #[inline(always)]
    pub fn pwrf(&self) -> PWRF_R {
        PWRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - illegal access flag for RCC"]
    #[inline(always)]
    pub fn rccf(&self) -> RCCF_R {
        RCCF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - illegal access flag for LPDMA"]
    #[inline(always)]
    pub fn lpdma1f(&self) -> LPDMA1F_R {
        LPDMA1F_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - illegal access flag for EXTI"]
    #[inline(always)]
    pub fn extif(&self) -> EXTIF_R {
        EXTIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 14 - illegal access flag for GTZC2 TZSC registers"]
    #[inline(always)]
    pub fn tzsc2f(&self) -> TZSC2F_R {
        TZSC2F_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - illegal access flag for GTZC2 TZIC registers"]
    #[inline(always)]
    pub fn tzic2f(&self) -> TZIC2F_R {
        TZIC2F_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - illegal access flag for SRAM4"]
    #[inline(always)]
    pub fn sram4f(&self) -> SRAM4F_R {
        SRAM4F_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - illegal access flag for MPCBB4 registers"]
    #[inline(always)]
    pub fn mpcbb4_regf(&self) -> MPCBB4_REGF_R {
        MPCBB4_REGF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "TZIC status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR2rs;
impl crate::RegisterSpec for SR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr2::R`](R) reader structure"]
impl crate::Readable for SR2rs {}
#[doc = "`reset()` method sets SR2 to value 0"]
impl crate::Resettable for SR2rs {
    const RESET_VALUE: u32 = 0;
}
