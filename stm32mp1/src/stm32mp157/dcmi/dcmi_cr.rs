#[doc = "Register `DCMI_CR` reader"]
pub type R = crate::R<DCMI_CRrs>;
#[doc = "Register `DCMI_CR` writer"]
pub type W = crate::W<DCMI_CRrs>;
#[doc = "Field `CAPTURE` reader - CAPTURE"]
pub type CAPTURE_R = crate::BitReader;
#[doc = "Field `CAPTURE` writer - CAPTURE"]
pub type CAPTURE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM` reader - CM"]
pub type CM_R = crate::BitReader;
#[doc = "Field `CM` writer - CM"]
pub type CM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CROP` reader - CROP"]
pub type CROP_R = crate::BitReader;
#[doc = "Field `CROP` writer - CROP"]
pub type CROP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JPEG` reader - JPEG"]
pub type JPEG_R = crate::BitReader;
#[doc = "Field `JPEG` writer - JPEG"]
pub type JPEG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESS` reader - ESS"]
pub type ESS_R = crate::BitReader;
#[doc = "Field `ESS` writer - ESS"]
pub type ESS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKPOL` reader - PCKPOL"]
pub type PCKPOL_R = crate::BitReader;
#[doc = "Field `PCKPOL` writer - PCKPOL"]
pub type PCKPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSPOL` reader - HSPOL"]
pub type HSPOL_R = crate::BitReader;
#[doc = "Field `HSPOL` writer - HSPOL"]
pub type HSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSPOL` reader - VSPOL"]
pub type VSPOL_R = crate::BitReader;
#[doc = "Field `VSPOL` writer - VSPOL"]
pub type VSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCRC` reader - FCRC"]
pub type FCRC_R = crate::FieldReader;
#[doc = "Field `FCRC` writer - FCRC"]
pub type FCRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EDM` reader - EDM"]
pub type EDM_R = crate::FieldReader;
#[doc = "Field `EDM` writer - EDM"]
pub type EDM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENABLE` reader - ENABLE"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - ENABLE"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSM` reader - BSM"]
pub type BSM_R = crate::FieldReader;
#[doc = "Field `BSM` writer - BSM"]
pub type BSM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OEBS` reader - OEBS"]
pub type OEBS_R = crate::BitReader;
#[doc = "Field `OEBS` writer - OEBS"]
pub type OEBS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSM` reader - LSM"]
pub type LSM_R = crate::BitReader;
#[doc = "Field `LSM` writer - LSM"]
pub type LSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OELS` reader - OELS"]
pub type OELS_R = crate::BitReader;
#[doc = "Field `OELS` writer - OELS"]
pub type OELS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CAPTURE"]
    #[inline(always)]
    pub fn capture(&self) -> CAPTURE_R {
        CAPTURE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CM"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CROP"]
    #[inline(always)]
    pub fn crop(&self) -> CROP_R {
        CROP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - JPEG"]
    #[inline(always)]
    pub fn jpeg(&self) -> JPEG_R {
        JPEG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ESS"]
    #[inline(always)]
    pub fn ess(&self) -> ESS_R {
        ESS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PCKPOL"]
    #[inline(always)]
    pub fn pckpol(&self) -> PCKPOL_R {
        PCKPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HSPOL"]
    #[inline(always)]
    pub fn hspol(&self) -> HSPOL_R {
        HSPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VSPOL"]
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - FCRC"]
    #[inline(always)]
    pub fn fcrc(&self) -> FCRC_R {
        FCRC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - EDM"]
    #[inline(always)]
    pub fn edm(&self) -> EDM_R {
        EDM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 14 - ENABLE"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - BSM"]
    #[inline(always)]
    pub fn bsm(&self) -> BSM_R {
        BSM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - OEBS"]
    #[inline(always)]
    pub fn oebs(&self) -> OEBS_R {
        OEBS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LSM"]
    #[inline(always)]
    pub fn lsm(&self) -> LSM_R {
        LSM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OELS"]
    #[inline(always)]
    pub fn oels(&self) -> OELS_R {
        OELS_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAPTURE"]
    #[inline(always)]
    #[must_use]
    pub fn capture(&mut self) -> CAPTURE_W<DCMI_CRrs> {
        CAPTURE_W::new(self, 0)
    }
    #[doc = "Bit 1 - CM"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<DCMI_CRrs> {
        CM_W::new(self, 1)
    }
    #[doc = "Bit 2 - CROP"]
    #[inline(always)]
    #[must_use]
    pub fn crop(&mut self) -> CROP_W<DCMI_CRrs> {
        CROP_W::new(self, 2)
    }
    #[doc = "Bit 3 - JPEG"]
    #[inline(always)]
    #[must_use]
    pub fn jpeg(&mut self) -> JPEG_W<DCMI_CRrs> {
        JPEG_W::new(self, 3)
    }
    #[doc = "Bit 4 - ESS"]
    #[inline(always)]
    #[must_use]
    pub fn ess(&mut self) -> ESS_W<DCMI_CRrs> {
        ESS_W::new(self, 4)
    }
    #[doc = "Bit 5 - PCKPOL"]
    #[inline(always)]
    #[must_use]
    pub fn pckpol(&mut self) -> PCKPOL_W<DCMI_CRrs> {
        PCKPOL_W::new(self, 5)
    }
    #[doc = "Bit 6 - HSPOL"]
    #[inline(always)]
    #[must_use]
    pub fn hspol(&mut self) -> HSPOL_W<DCMI_CRrs> {
        HSPOL_W::new(self, 6)
    }
    #[doc = "Bit 7 - VSPOL"]
    #[inline(always)]
    #[must_use]
    pub fn vspol(&mut self) -> VSPOL_W<DCMI_CRrs> {
        VSPOL_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - FCRC"]
    #[inline(always)]
    #[must_use]
    pub fn fcrc(&mut self) -> FCRC_W<DCMI_CRrs> {
        FCRC_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - EDM"]
    #[inline(always)]
    #[must_use]
    pub fn edm(&mut self) -> EDM_W<DCMI_CRrs> {
        EDM_W::new(self, 10)
    }
    #[doc = "Bit 14 - ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<DCMI_CRrs> {
        ENABLE_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - BSM"]
    #[inline(always)]
    #[must_use]
    pub fn bsm(&mut self) -> BSM_W<DCMI_CRrs> {
        BSM_W::new(self, 16)
    }
    #[doc = "Bit 18 - OEBS"]
    #[inline(always)]
    #[must_use]
    pub fn oebs(&mut self) -> OEBS_W<DCMI_CRrs> {
        OEBS_W::new(self, 18)
    }
    #[doc = "Bit 19 - LSM"]
    #[inline(always)]
    #[must_use]
    pub fn lsm(&mut self) -> LSM_W<DCMI_CRrs> {
        LSM_W::new(self, 19)
    }
    #[doc = "Bit 20 - OELS"]
    #[inline(always)]
    #[must_use]
    pub fn oels(&mut self) -> OELS_W<DCMI_CRrs> {
        OELS_W::new(self, 20)
    }
}
#[doc = "DCMI control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcmi_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcmi_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCMI_CRrs;
impl crate::RegisterSpec for DCMI_CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcmi_cr::R`](R) reader structure"]
impl crate::Readable for DCMI_CRrs {}
#[doc = "`write(|w| ..)` method takes [`dcmi_cr::W`](W) writer structure"]
impl crate::Writable for DCMI_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCMI_CR to value 0"]
impl crate::Resettable for DCMI_CRrs {
    const RESET_VALUE: u32 = 0;
}
