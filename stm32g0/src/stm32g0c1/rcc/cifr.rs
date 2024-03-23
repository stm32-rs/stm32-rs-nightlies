#[doc = "Register `CIFR` reader"]
pub type R = crate::R<CIFRrs>;
#[doc = "Field `LSIRDYF` reader - LSI ready interrupt flag"]
pub type LSIRDYF_R = crate::BitReader;
#[doc = "Field `LSERDYF` reader - LSE ready interrupt flag"]
pub type LSERDYF_R = crate::BitReader;
#[doc = "Field `HSI48RDYF` reader - HSI48RDYF"]
pub type HSI48RDYF_R = crate::BitReader;
#[doc = "Field `HSIRDYF` reader - HSI ready interrupt flag"]
pub type HSIRDYF_R = crate::BitReader;
#[doc = "Field `HSERDYF` reader - HSE ready interrupt flag"]
pub type HSERDYF_R = crate::BitReader;
#[doc = "Field `PLLSYSRDYF` reader - PLL ready interrupt flag"]
pub type PLLSYSRDYF_R = crate::BitReader;
#[doc = "Field `CSSF` reader - Clock security system interrupt flag"]
pub type CSSF_R = crate::BitReader;
#[doc = "Field `LSECSSF` reader - LSE Clock security system interrupt flag"]
pub type LSECSSF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - LSI ready interrupt flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt flag"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI48RDYF"]
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI ready interrupt flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL ready interrupt flag"]
    #[inline(always)]
    pub fn pllsysrdyf(&self) -> PLLSYSRDYF_R {
        PLLSYSRDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Clock security system interrupt flag"]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE Clock security system interrupt flag"]
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Clock interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cifr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIFRrs;
impl crate::RegisterSpec for CIFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cifr::R`](R) reader structure"]
impl crate::Readable for CIFRrs {}
#[doc = "`reset()` method sets CIFR to value 0"]
impl crate::Resettable for CIFRrs {
    const RESET_VALUE: u32 = 0;
}
