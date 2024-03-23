#[doc = "Register `PSR` reader"]
pub type R = crate::R<PSRrs>;
#[doc = "Field `PD` reader - PHY Direction"]
pub type PD_R = crate::BitReader;
#[doc = "Field `PSSC` reader - PHY Stop State Clock lane"]
pub type PSSC_R = crate::BitReader;
#[doc = "Field `UANC` reader - ULPS Active Not Clock lane"]
pub type UANC_R = crate::BitReader;
#[doc = "Field `PSS0` reader - PHY Stop State lane 0"]
pub type PSS0_R = crate::BitReader;
#[doc = "Field `UAN0` reader - ULPS Active Not lane 1"]
pub type UAN0_R = crate::BitReader;
#[doc = "Field `RUE0` reader - RX ULPS Escape lane 0"]
pub type RUE0_R = crate::BitReader;
#[doc = "Field `PSS1` reader - PHY Stop State lane 1"]
pub type PSS1_R = crate::BitReader;
#[doc = "Field `UAN1` reader - ULPS Active Not lane 1"]
pub type UAN1_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - PHY Direction"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PHY Stop State Clock lane"]
    #[inline(always)]
    pub fn pssc(&self) -> PSSC_R {
        PSSC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ULPS Active Not Clock lane"]
    #[inline(always)]
    pub fn uanc(&self) -> UANC_R {
        UANC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PHY Stop State lane 0"]
    #[inline(always)]
    pub fn pss0(&self) -> PSS0_R {
        PSS0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ULPS Active Not lane 1"]
    #[inline(always)]
    pub fn uan0(&self) -> UAN0_R {
        UAN0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX ULPS Escape lane 0"]
    #[inline(always)]
    pub fn rue0(&self) -> RUE0_R {
        RUE0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PHY Stop State lane 1"]
    #[inline(always)]
    pub fn pss1(&self) -> PSS1_R {
        PSS1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ULPS Active Not lane 1"]
    #[inline(always)]
    pub fn uan1(&self) -> UAN1_R {
        UAN1_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "DSI Host PHY Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSRrs;
impl crate::RegisterSpec for PSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr::R`](R) reader structure"]
impl crate::Readable for PSRrs {}
#[doc = "`reset()` method sets PSR to value 0x1528"]
impl crate::Resettable for PSRrs {
    const RESET_VALUE: u32 = 0x1528;
}
