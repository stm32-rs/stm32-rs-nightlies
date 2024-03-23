#[doc = "Register `CCFG` reader"]
pub type R = crate::R<CCFGrs>;
#[doc = "Register `CCFG` writer"]
pub type W = crate::W<CCFGrs>;
#[doc = "Field `TQBT` reader - Time Quanta per Bit Time"]
pub type TQBT_R = crate::FieldReader;
#[doc = "Field `TQBT` writer - Time Quanta per Bit Time"]
pub type TQBT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BCC` reader - Bypass Clock Calibration"]
pub type BCC_R = crate::BitReader;
#[doc = "Field `BCC` writer - Bypass Clock Calibration"]
pub type BCC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFL` reader - Calibration Field Length"]
pub type CFL_R = crate::BitReader;
#[doc = "Field `CFL` writer - Calibration Field Length"]
pub type CFL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCPM` reader - Oscillator Clock Periods Minimum"]
pub type OCPM_R = crate::FieldReader;
#[doc = "Field `OCPM` writer - Oscillator Clock Periods Minimum"]
pub type OCPM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CDIV` reader - Clock Divider"]
pub type CDIV_R = crate::FieldReader;
#[doc = "Field `CDIV` writer - Clock Divider"]
pub type CDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SWR` reader - Software Reset"]
pub type SWR_R = crate::BitReader;
#[doc = "Field `SWR` writer - Software Reset"]
pub type SWR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Time Quanta per Bit Time"]
    #[inline(always)]
    pub fn tqbt(&self) -> TQBT_R {
        TQBT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Bypass Clock Calibration"]
    #[inline(always)]
    pub fn bcc(&self) -> BCC_R {
        BCC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Calibration Field Length"]
    #[inline(always)]
    pub fn cfl(&self) -> CFL_R {
        CFL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Oscillator Clock Periods Minimum"]
    #[inline(always)]
    pub fn ocpm(&self) -> OCPM_R {
        OCPM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Clock Divider"]
    #[inline(always)]
    pub fn cdiv(&self) -> CDIV_R {
        CDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Software Reset"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Time Quanta per Bit Time"]
    #[inline(always)]
    #[must_use]
    pub fn tqbt(&mut self) -> TQBT_W<CCFGrs> {
        TQBT_W::new(self, 0)
    }
    #[doc = "Bit 6 - Bypass Clock Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn bcc(&mut self) -> BCC_W<CCFGrs> {
        BCC_W::new(self, 6)
    }
    #[doc = "Bit 7 - Calibration Field Length"]
    #[inline(always)]
    #[must_use]
    pub fn cfl(&mut self) -> CFL_W<CCFGrs> {
        CFL_W::new(self, 7)
    }
    #[doc = "Bits 8:15 - Oscillator Clock Periods Minimum"]
    #[inline(always)]
    #[must_use]
    pub fn ocpm(&mut self) -> OCPM_W<CCFGrs> {
        OCPM_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn cdiv(&mut self) -> CDIV_W<CCFGrs> {
        CDIV_W::new(self, 16)
    }
    #[doc = "Bit 31 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swr(&mut self) -> SWR_W<CCFGrs> {
        SWR_W::new(self, 31)
    }
}
#[doc = "Calibration Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCFGrs;
impl crate::RegisterSpec for CCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccfg::R`](R) reader structure"]
impl crate::Readable for CCFGrs {}
#[doc = "`write(|w| ..)` method takes [`ccfg::W`](W) writer structure"]
impl crate::Writable for CCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCFG to value 0"]
impl crate::Resettable for CCFGrs {
    const RESET_VALUE: u32 = 0;
}
