#[doc = "Register `SFR` reader"]
pub type R = crate::R<SFRrs>;
#[doc = "Register `SFR` writer"]
pub type W = crate::W<SFRrs>;
#[doc = "Field `SFSA` reader - Secure flash start address"]
pub type SFSA_R = crate::FieldReader;
#[doc = "Field `SFSA` writer - Secure flash start address"]
pub type SFSA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FSD` reader - Flash security disable"]
pub type FSD_R = crate::BitReader;
#[doc = "Field `FSD` writer - Flash security disable"]
pub type FSD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDS` reader - Disable Cortex M0 debug access"]
pub type DDS_R = crate::BitReader;
#[doc = "Field `DDS` writer - Disable Cortex M0 debug access"]
pub type DDS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Secure flash start address"]
    #[inline(always)]
    pub fn sfsa(&self) -> SFSA_R {
        SFSA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Flash security disable"]
    #[inline(always)]
    pub fn fsd(&self) -> FSD_R {
        FSD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Disable Cortex M0 debug access"]
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Secure flash start address"]
    #[inline(always)]
    #[must_use]
    pub fn sfsa(&mut self) -> SFSA_W<SFRrs> {
        SFSA_W::new(self, 0)
    }
    #[doc = "Bit 8 - Flash security disable"]
    #[inline(always)]
    #[must_use]
    pub fn fsd(&mut self) -> FSD_W<SFRrs> {
        FSD_W::new(self, 8)
    }
    #[doc = "Bit 12 - Disable Cortex M0 debug access"]
    #[inline(always)]
    #[must_use]
    pub fn dds(&mut self) -> DDS_W<SFRrs> {
        DDS_W::new(self, 12)
    }
}
#[doc = "Secure flash start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SFRrs;
impl crate::RegisterSpec for SFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr::R`](R) reader structure"]
impl crate::Readable for SFRrs {}
#[doc = "`write(|w| ..)` method takes [`sfr::W`](W) writer structure"]
impl crate::Writable for SFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SFR to value 0xffff_ee00"]
impl crate::Resettable for SFRrs {
    const RESET_VALUE: u32 = 0xffff_ee00;
}
