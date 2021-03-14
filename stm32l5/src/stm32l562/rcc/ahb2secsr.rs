#[doc = "Reader of register AHB2SECSR"]
pub type R = crate::R<u32, super::AHB2SECSR>;
#[doc = "Reader of field `SDMMC1SECF`"]
pub type SDMMC1SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `OTFDEC1SECF`"]
pub type OTFDEC1SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRAM2SECF`"]
pub type SRAM2SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIOHSECF`"]
pub type GPIOHSECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIOGSECF`"]
pub type GPIOGSECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIOFSECF`"]
pub type GPIOFSECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIOESECF`"]
pub type GPIOESECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIODSECF`"]
pub type GPIODSECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIOCSECF`"]
pub type GPIOCSECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIOBSECF`"]
pub type GPIOBSECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIOASECF`"]
pub type GPIOASECF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 22 - SDMMC1SECF"]
    #[inline(always)]
    pub fn sdmmc1secf(&self) -> SDMMC1SECF_R {
        SDMMC1SECF_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - OTFDEC1SECF"]
    #[inline(always)]
    pub fn otfdec1secf(&self) -> OTFDEC1SECF_R {
        OTFDEC1SECF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SRAM2SECF"]
    #[inline(always)]
    pub fn sram2secf(&self) -> SRAM2SECF_R {
        SRAM2SECF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIOHSECF"]
    #[inline(always)]
    pub fn gpiohsecf(&self) -> GPIOHSECF_R {
        GPIOHSECF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIOGSECF"]
    #[inline(always)]
    pub fn gpiogsecf(&self) -> GPIOGSECF_R {
        GPIOGSECF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIOFSECF"]
    #[inline(always)]
    pub fn gpiofsecf(&self) -> GPIOFSECF_R {
        GPIOFSECF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIOESECF"]
    #[inline(always)]
    pub fn gpioesecf(&self) -> GPIOESECF_R {
        GPIOESECF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIODSECF"]
    #[inline(always)]
    pub fn gpiodsecf(&self) -> GPIODSECF_R {
        GPIODSECF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIOCSECF"]
    #[inline(always)]
    pub fn gpiocsecf(&self) -> GPIOCSECF_R {
        GPIOCSECF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIOBSECF"]
    #[inline(always)]
    pub fn gpiobsecf(&self) -> GPIOBSECF_R {
        GPIOBSECF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - GPIOASECF"]
    #[inline(always)]
    pub fn gpioasecf(&self) -> GPIOASECF_R {
        GPIOASECF_R::new((self.bits & 0x01) != 0)
    }
}
