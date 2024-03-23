#[doc = "Register `FCCAN_CCU_CCFG` reader"]
pub type R = crate::R<FCCAN_CCU_CCFGrs>;
#[doc = "Register `FCCAN_CCU_CCFG` writer"]
pub type W = crate::W<FCCAN_CCU_CCFGrs>;
#[doc = "Field `TQBT` reader - TQBT"]
pub type TQBT_R = crate::FieldReader;
#[doc = "Field `TQBT` writer - TQBT"]
pub type TQBT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BCC` reader - BCC"]
pub type BCC_R = crate::BitReader;
#[doc = "Field `BCC` writer - BCC"]
pub type BCC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFL` reader - CFL"]
pub type CFL_R = crate::BitReader;
#[doc = "Field `CFL` writer - CFL"]
pub type CFL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCPM` reader - OCPM"]
pub type OCPM_R = crate::FieldReader;
#[doc = "Field `OCPM` writer - OCPM"]
pub type OCPM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CDIV` reader - CDIV"]
pub type CDIV_R = crate::FieldReader;
#[doc = "Field `CDIV` writer - CDIV"]
pub type CDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SWR` reader - SWR"]
pub type SWR_R = crate::BitReader;
#[doc = "Field `SWR` writer - SWR"]
pub type SWR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - TQBT"]
    #[inline(always)]
    pub fn tqbt(&self) -> TQBT_R {
        TQBT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - BCC"]
    #[inline(always)]
    pub fn bcc(&self) -> BCC_R {
        BCC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CFL"]
    #[inline(always)]
    pub fn cfl(&self) -> CFL_R {
        CFL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - OCPM"]
    #[inline(always)]
    pub fn ocpm(&self) -> OCPM_R {
        OCPM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - CDIV"]
    #[inline(always)]
    pub fn cdiv(&self) -> CDIV_R {
        CDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - SWR"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - TQBT"]
    #[inline(always)]
    #[must_use]
    pub fn tqbt(&mut self) -> TQBT_W<FCCAN_CCU_CCFGrs> {
        TQBT_W::new(self, 0)
    }
    #[doc = "Bit 6 - BCC"]
    #[inline(always)]
    #[must_use]
    pub fn bcc(&mut self) -> BCC_W<FCCAN_CCU_CCFGrs> {
        BCC_W::new(self, 6)
    }
    #[doc = "Bit 7 - CFL"]
    #[inline(always)]
    #[must_use]
    pub fn cfl(&mut self) -> CFL_W<FCCAN_CCU_CCFGrs> {
        CFL_W::new(self, 7)
    }
    #[doc = "Bits 8:15 - OCPM"]
    #[inline(always)]
    #[must_use]
    pub fn ocpm(&mut self) -> OCPM_W<FCCAN_CCU_CCFGrs> {
        OCPM_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - CDIV"]
    #[inline(always)]
    #[must_use]
    pub fn cdiv(&mut self) -> CDIV_W<FCCAN_CCU_CCFGrs> {
        CDIV_W::new(self, 16)
    }
    #[doc = "Bit 31 - SWR"]
    #[inline(always)]
    #[must_use]
    pub fn swr(&mut self) -> SWR_W<FCCAN_CCU_CCFGrs> {
        SWR_W::new(self, 31)
    }
}
#[doc = "Calibration configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fccan_ccu_ccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fccan_ccu_ccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCCAN_CCU_CCFGrs;
impl crate::RegisterSpec for FCCAN_CCU_CCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fccan_ccu_ccfg::R`](R) reader structure"]
impl crate::Readable for FCCAN_CCU_CCFGrs {}
#[doc = "`write(|w| ..)` method takes [`fccan_ccu_ccfg::W`](W) writer structure"]
impl crate::Writable for FCCAN_CCU_CCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCCAN_CCU_CCFG to value 0x04"]
impl crate::Resettable for FCCAN_CCU_CCFGrs {
    const RESET_VALUE: u32 = 0x04;
}
