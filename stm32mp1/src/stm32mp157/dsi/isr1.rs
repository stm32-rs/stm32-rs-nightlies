#[doc = "Register `ISR1` reader"]
pub type R = crate::R<ISR1rs>;
#[doc = "Field `TOHSTX` reader - TOHSTX"]
pub type TOHSTX_R = crate::BitReader;
#[doc = "Field `TOLPRX` reader - TOLPRX"]
pub type TOLPRX_R = crate::BitReader;
#[doc = "Field `ECCSE` reader - ECCSE"]
pub type ECCSE_R = crate::BitReader;
#[doc = "Field `ECCME` reader - ECCME"]
pub type ECCME_R = crate::BitReader;
#[doc = "Field `CRCE` reader - CRCE"]
pub type CRCE_R = crate::BitReader;
#[doc = "Field `PSE` reader - PSE"]
pub type PSE_R = crate::BitReader;
#[doc = "Field `EOTPE` reader - EOTPE"]
pub type EOTPE_R = crate::BitReader;
#[doc = "Field `LPWRE` reader - LPWRE"]
pub type LPWRE_R = crate::BitReader;
#[doc = "Field `GCWRE` reader - GCWRE"]
pub type GCWRE_R = crate::BitReader;
#[doc = "Field `GPWRE` reader - GPWRE"]
pub type GPWRE_R = crate::BitReader;
#[doc = "Field `GPTXE` reader - GPTXE"]
pub type GPTXE_R = crate::BitReader;
#[doc = "Field `GPRDE` reader - GPRDE"]
pub type GPRDE_R = crate::BitReader;
#[doc = "Field `GPRXE` reader - GPRXE"]
pub type GPRXE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TOHSTX"]
    #[inline(always)]
    pub fn tohstx(&self) -> TOHSTX_R {
        TOHSTX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TOLPRX"]
    #[inline(always)]
    pub fn tolprx(&self) -> TOLPRX_R {
        TOLPRX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ECCSE"]
    #[inline(always)]
    pub fn eccse(&self) -> ECCSE_R {
        ECCSE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECCME"]
    #[inline(always)]
    pub fn eccme(&self) -> ECCME_R {
        ECCME_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRCE"]
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PSE"]
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EOTPE"]
    #[inline(always)]
    pub fn eotpe(&self) -> EOTPE_R {
        EOTPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LPWRE"]
    #[inline(always)]
    pub fn lpwre(&self) -> LPWRE_R {
        LPWRE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GCWRE"]
    #[inline(always)]
    pub fn gcwre(&self) -> GCWRE_R {
        GCWRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPWRE"]
    #[inline(always)]
    pub fn gpwre(&self) -> GPWRE_R {
        GPWRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTXE"]
    #[inline(always)]
    pub fn gptxe(&self) -> GPTXE_R {
        GPTXE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPRDE"]
    #[inline(always)]
    pub fn gprde(&self) -> GPRDE_R {
        GPRDE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPRXE"]
    #[inline(always)]
    pub fn gprxe(&self) -> GPRXE_R {
        GPRXE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "DSI Host interrupt and status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR1rs;
impl crate::RegisterSpec for ISR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr1::R`](R) reader structure"]
impl crate::Readable for ISR1rs {}
#[doc = "`reset()` method sets ISR1 to value 0"]
impl crate::Resettable for ISR1rs {
    const RESET_VALUE: u32 = 0;
}
