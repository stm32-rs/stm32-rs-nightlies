#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSRrs>;
#[doc = "Field `WUF` reader - Wakeup flag"]
pub type WUF_R = crate::BitReader;
#[doc = "Field `SBF` reader - Standby flag"]
pub type SBF_R = crate::BitReader;
#[doc = "Field `PVDO` reader - PVD output"]
pub type PVDO_R = crate::BitReader;
#[doc = "Field `BRR` reader - Backup regulator ready"]
pub type BRR_R = crate::BitReader;
#[doc = "Field `EWUP` reader - Enable WKUP pin"]
pub type EWUP_R = crate::BitReader;
#[doc = "Field `EWUP` writer - Enable WKUP pin"]
pub type EWUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRE` reader - Backup regulator enable"]
pub type BRE_R = crate::BitReader;
#[doc = "Field `BRE` writer - Backup regulator enable"]
pub type BRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VOSRDY` reader - Regulator voltage scaling output selection ready bit"]
pub type VOSRDY_R = crate::BitReader;
#[doc = "Field `VOSRDY` writer - Regulator voltage scaling output selection ready bit"]
pub type VOSRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wakeup flag"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Standby flag"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PVD output"]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Backup regulator ready"]
    #[inline(always)]
    pub fn brr(&self) -> BRR_R {
        BRR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable WKUP pin"]
    #[inline(always)]
    pub fn ewup(&self) -> EWUP_R {
        EWUP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Backup regulator enable"]
    #[inline(always)]
    pub fn bre(&self) -> BRE_R {
        BRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Regulator voltage scaling output selection ready bit"]
    #[inline(always)]
    pub fn vosrdy(&self) -> VOSRDY_R {
        VOSRDY_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Enable WKUP pin"]
    #[inline(always)]
    #[must_use]
    pub fn ewup(&mut self) -> EWUP_W<CSRrs> {
        EWUP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Backup regulator enable"]
    #[inline(always)]
    #[must_use]
    pub fn bre(&mut self) -> BRE_W<CSRrs> {
        BRE_W::new(self, 9)
    }
    #[doc = "Bit 14 - Regulator voltage scaling output selection ready bit"]
    #[inline(always)]
    #[must_use]
    pub fn vosrdy(&mut self) -> VOSRDY_W<CSRrs> {
        VOSRDY_W::new(self, 14)
    }
}
#[doc = "power control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSRrs {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0;
}
