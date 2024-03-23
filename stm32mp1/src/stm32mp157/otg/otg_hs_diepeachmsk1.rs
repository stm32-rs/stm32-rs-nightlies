#[doc = "Register `OTG_HS_DIEPEACHMSK1` reader"]
pub type R = crate::R<OTG_HS_DIEPEACHMSK1rs>;
#[doc = "Register `OTG_HS_DIEPEACHMSK1` writer"]
pub type W = crate::W<OTG_HS_DIEPEACHMSK1rs>;
#[doc = "Field `XFRCM` reader - XFRCM"]
pub type XFRCM_R = crate::BitReader;
#[doc = "Field `XFRCM` writer - XFRCM"]
pub type XFRCM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDM` reader - EPDM"]
pub type EPDM_R = crate::BitReader;
#[doc = "Field `EPDM` writer - EPDM"]
pub type EPDM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERRM` reader - AHBERRM"]
pub type AHBERRM_R = crate::BitReader;
#[doc = "Field `AHBERRM` writer - AHBERRM"]
pub type AHBERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOM` reader - TOM"]
pub type TOM_R = crate::BitReader;
#[doc = "Field `TOM` writer - TOM"]
pub type TOM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITTXFEMSK` reader - ITTXFEMSK"]
pub type ITTXFEMSK_R = crate::BitReader;
#[doc = "Field `ITTXFEMSK` writer - ITTXFEMSK"]
pub type ITTXFEMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNEM` reader - INEPNEM"]
pub type INEPNEM_R = crate::BitReader;
#[doc = "Field `INEPNEM` writer - INEPNEM"]
pub type INEPNEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFURM` reader - TXFURM"]
pub type TXFURM_R = crate::BitReader;
#[doc = "Field `TXFURM` writer - TXFURM"]
pub type TXFURM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAM` reader - BNAM"]
pub type BNAM_R = crate::BitReader;
#[doc = "Field `BNAM` writer - BNAM"]
pub type BNAM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKM` reader - NAKM"]
pub type NAKM_R = crate::BitReader;
#[doc = "Field `NAKM` writer - NAKM"]
pub type NAKM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - XFRCM"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EPDM"]
    #[inline(always)]
    pub fn epdm(&self) -> EPDM_R {
        EPDM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHBERRM"]
    #[inline(always)]
    pub fn ahberrm(&self) -> AHBERRM_R {
        AHBERRM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TOM"]
    #[inline(always)]
    pub fn tom(&self) -> TOM_R {
        TOM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ITTXFEMSK"]
    #[inline(always)]
    pub fn ittxfemsk(&self) -> ITTXFEMSK_R {
        ITTXFEMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - INEPNEM"]
    #[inline(always)]
    pub fn inepnem(&self) -> INEPNEM_R {
        INEPNEM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - TXFURM"]
    #[inline(always)]
    pub fn txfurm(&self) -> TXFURM_R {
        TXFURM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNAM"]
    #[inline(always)]
    pub fn bnam(&self) -> BNAM_R {
        BNAM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - NAKM"]
    #[inline(always)]
    pub fn nakm(&self) -> NAKM_R {
        NAKM_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XFRCM"]
    #[inline(always)]
    #[must_use]
    pub fn xfrcm(&mut self) -> XFRCM_W<OTG_HS_DIEPEACHMSK1rs> {
        XFRCM_W::new(self, 0)
    }
    #[doc = "Bit 1 - EPDM"]
    #[inline(always)]
    #[must_use]
    pub fn epdm(&mut self) -> EPDM_W<OTG_HS_DIEPEACHMSK1rs> {
        EPDM_W::new(self, 1)
    }
    #[doc = "Bit 2 - AHBERRM"]
    #[inline(always)]
    #[must_use]
    pub fn ahberrm(&mut self) -> AHBERRM_W<OTG_HS_DIEPEACHMSK1rs> {
        AHBERRM_W::new(self, 2)
    }
    #[doc = "Bit 3 - TOM"]
    #[inline(always)]
    #[must_use]
    pub fn tom(&mut self) -> TOM_W<OTG_HS_DIEPEACHMSK1rs> {
        TOM_W::new(self, 3)
    }
    #[doc = "Bit 4 - ITTXFEMSK"]
    #[inline(always)]
    #[must_use]
    pub fn ittxfemsk(&mut self) -> ITTXFEMSK_W<OTG_HS_DIEPEACHMSK1rs> {
        ITTXFEMSK_W::new(self, 4)
    }
    #[doc = "Bit 6 - INEPNEM"]
    #[inline(always)]
    #[must_use]
    pub fn inepnem(&mut self) -> INEPNEM_W<OTG_HS_DIEPEACHMSK1rs> {
        INEPNEM_W::new(self, 6)
    }
    #[doc = "Bit 8 - TXFURM"]
    #[inline(always)]
    #[must_use]
    pub fn txfurm(&mut self) -> TXFURM_W<OTG_HS_DIEPEACHMSK1rs> {
        TXFURM_W::new(self, 8)
    }
    #[doc = "Bit 9 - BNAM"]
    #[inline(always)]
    #[must_use]
    pub fn bnam(&mut self) -> BNAM_W<OTG_HS_DIEPEACHMSK1rs> {
        BNAM_W::new(self, 9)
    }
    #[doc = "Bit 13 - NAKM"]
    #[inline(always)]
    #[must_use]
    pub fn nakm(&mut self) -> NAKM_W<OTG_HS_DIEPEACHMSK1rs> {
        NAKM_W::new(self, 13)
    }
}
#[doc = "This register works with the OTG_DIEPINT1 register to generate a dedicated interrupt OTG_HS_EP1_IN for endpoint #1. The IN endpoint interrupt for a specific status in the OTG_DOEPINT1 register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_diepeachmsk1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_diepeachmsk1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HS_DIEPEACHMSK1rs;
impl crate::RegisterSpec for OTG_HS_DIEPEACHMSK1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_diepeachmsk1::R`](R) reader structure"]
impl crate::Readable for OTG_HS_DIEPEACHMSK1rs {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_diepeachmsk1::W`](W) writer structure"]
impl crate::Writable for OTG_HS_DIEPEACHMSK1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_HS_DIEPEACHMSK1 to value 0"]
impl crate::Resettable for OTG_HS_DIEPEACHMSK1rs {
    const RESET_VALUE: u32 = 0;
}
