#[doc = "Register `DMAMUX_RG7CR` reader"]
pub type R = crate::R<DMAMUX_RG7CRrs>;
#[doc = "Register `DMAMUX_RG7CR` writer"]
pub type W = crate::W<DMAMUX_RG7CRrs>;
#[doc = "Field `SIG_ID` reader - SIG_ID"]
pub type SIG_ID_R = crate::FieldReader;
#[doc = "Field `SIG_ID` writer - SIG_ID"]
pub type SIG_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OIE` reader - OIE"]
pub type OIE_R = crate::BitReader;
#[doc = "Field `OIE` writer - OIE"]
pub type OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GE` reader - GE"]
pub type GE_R = crate::BitReader;
#[doc = "Field `GE` writer - GE"]
pub type GE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPOL` reader - GPOL"]
pub type GPOL_R = crate::FieldReader;
#[doc = "Field `GPOL` writer - GPOL"]
pub type GPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GNBREQ` reader - GNBREQ"]
pub type GNBREQ_R = crate::FieldReader;
#[doc = "Field `GNBREQ` writer - GNBREQ"]
pub type GNBREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:2 - SIG_ID"]
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - OIE"]
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - GE"]
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - GPOL"]
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - GNBREQ"]
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SIG_ID"]
    #[inline(always)]
    #[must_use]
    pub fn sig_id(&mut self) -> SIG_ID_W<DMAMUX_RG7CRrs> {
        SIG_ID_W::new(self, 0)
    }
    #[doc = "Bit 8 - OIE"]
    #[inline(always)]
    #[must_use]
    pub fn oie(&mut self) -> OIE_W<DMAMUX_RG7CRrs> {
        OIE_W::new(self, 8)
    }
    #[doc = "Bit 16 - GE"]
    #[inline(always)]
    #[must_use]
    pub fn ge(&mut self) -> GE_W<DMAMUX_RG7CRrs> {
        GE_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - GPOL"]
    #[inline(always)]
    #[must_use]
    pub fn gpol(&mut self) -> GPOL_W<DMAMUX_RG7CRrs> {
        GPOL_W::new(self, 17)
    }
    #[doc = "Bits 19:23 - GNBREQ"]
    #[inline(always)]
    #[must_use]
    pub fn gnbreq(&mut self) -> GNBREQ_W<DMAMUX_RG7CRrs> {
        GNBREQ_W::new(self, 19)
    }
}
#[doc = "DMAMUX request generator channel 7 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_rg7cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rg7cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAMUX_RG7CRrs;
impl crate::RegisterSpec for DMAMUX_RG7CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamux_rg7cr::R`](R) reader structure"]
impl crate::Readable for DMAMUX_RG7CRrs {}
#[doc = "`write(|w| ..)` method takes [`dmamux_rg7cr::W`](W) writer structure"]
impl crate::Writable for DMAMUX_RG7CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAMUX_RG7CR to value 0"]
impl crate::Resettable for DMAMUX_RG7CRrs {
    const RESET_VALUE: u32 = 0;
}
