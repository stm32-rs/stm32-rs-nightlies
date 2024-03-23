#[doc = "Register `VREFBUF_CSR` reader"]
pub type R = crate::R<VREFBUF_CSRrs>;
#[doc = "Register `VREFBUF_CSR` writer"]
pub type W = crate::W<VREFBUF_CSRrs>;
#[doc = "Field `ENVR` reader - ENVR"]
pub type ENVR_R = crate::BitReader;
#[doc = "Field `ENVR` writer - ENVR"]
pub type ENVR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIZ` reader - HIZ"]
pub type HIZ_R = crate::BitReader;
#[doc = "Field `HIZ` writer - HIZ"]
pub type HIZ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VRR` reader - VRR"]
pub type VRR_R = crate::BitReader;
#[doc = "Field `VRS` reader - VRS"]
pub type VRS_R = crate::FieldReader;
#[doc = "Field `VRS` writer - VRS"]
pub type VRS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - ENVR"]
    #[inline(always)]
    pub fn envr(&self) -> ENVR_R {
        ENVR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HIZ"]
    #[inline(always)]
    pub fn hiz(&self) -> HIZ_R {
        HIZ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - VRR"]
    #[inline(always)]
    pub fn vrr(&self) -> VRR_R {
        VRR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - VRS"]
    #[inline(always)]
    pub fn vrs(&self) -> VRS_R {
        VRS_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ENVR"]
    #[inline(always)]
    #[must_use]
    pub fn envr(&mut self) -> ENVR_W<VREFBUF_CSRrs> {
        ENVR_W::new(self, 0)
    }
    #[doc = "Bit 1 - HIZ"]
    #[inline(always)]
    #[must_use]
    pub fn hiz(&mut self) -> HIZ_W<VREFBUF_CSRrs> {
        HIZ_W::new(self, 1)
    }
    #[doc = "Bits 4:6 - VRS"]
    #[inline(always)]
    #[must_use]
    pub fn vrs(&mut self) -> VRS_W<VREFBUF_CSRrs> {
        VRS_W::new(self, 4)
    }
}
#[doc = "VREFBUF control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vrefbuf_csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vrefbuf_csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VREFBUF_CSRrs;
impl crate::RegisterSpec for VREFBUF_CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vrefbuf_csr::R`](R) reader structure"]
impl crate::Readable for VREFBUF_CSRrs {}
#[doc = "`write(|w| ..)` method takes [`vrefbuf_csr::W`](W) writer structure"]
impl crate::Writable for VREFBUF_CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VREFBUF_CSR to value 0x02"]
impl crate::Resettable for VREFBUF_CSRrs {
    const RESET_VALUE: u32 = 0x02;
}
