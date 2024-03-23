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
#[doc = "Field `VREFINTRDYF` reader - Internal voltage reference (VREFINT) ready flag"]
pub type VREFINTRDYF_R = crate::BitReader;
#[doc = "Field `VOSF` reader - Voltage Scaling select flag"]
pub type VOSF_R = crate::BitReader;
#[doc = "Field `REGLPF` reader - Regulator LP flag"]
pub type REGLPF_R = crate::BitReader;
#[doc = "Field `EWUP1` reader - Enable WKUP pin 1"]
pub type EWUP1_R = crate::BitReader;
#[doc = "Field `EWUP1` writer - Enable WKUP pin 1"]
pub type EWUP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP2` reader - Enable WKUP pin 2"]
pub type EWUP2_R = crate::BitReader;
#[doc = "Field `EWUP2` writer - Enable WKUP pin 2"]
pub type EWUP2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP3` reader - Enable WKUP pin 3"]
pub type EWUP3_R = crate::BitReader;
#[doc = "Field `EWUP3` writer - Enable WKUP pin 3"]
pub type EWUP3_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 3 - Internal voltage reference (VREFINT) ready flag"]
    #[inline(always)]
    pub fn vrefintrdyf(&self) -> VREFINTRDYF_R {
        VREFINTRDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Voltage Scaling select flag"]
    #[inline(always)]
    pub fn vosf(&self) -> VOSF_R {
        VOSF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Regulator LP flag"]
    #[inline(always)]
    pub fn reglpf(&self) -> REGLPF_R {
        REGLPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable WKUP pin 1"]
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable WKUP pin 2"]
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable WKUP pin 3"]
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Enable WKUP pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn ewup1(&mut self) -> EWUP1_W<CSRrs> {
        EWUP1_W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable WKUP pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn ewup2(&mut self) -> EWUP2_W<CSRrs> {
        EWUP2_W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable WKUP pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn ewup3(&mut self) -> EWUP3_W<CSRrs> {
        EWUP3_W::new(self, 10)
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
#[doc = "`reset()` method sets CSR to value 0x08"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0x08;
}
