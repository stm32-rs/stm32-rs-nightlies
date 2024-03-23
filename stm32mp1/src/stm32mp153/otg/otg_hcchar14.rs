#[doc = "Register `OTG_HCCHAR14` reader"]
pub type R = crate::R<OTG_HCCHAR14rs>;
#[doc = "Register `OTG_HCCHAR14` writer"]
pub type W = crate::W<OTG_HCCHAR14rs>;
#[doc = "Field `MPSIZ` reader - MPSIZ"]
pub type MPSIZ_R = crate::FieldReader<u16>;
#[doc = "Field `MPSIZ` writer - MPSIZ"]
pub type MPSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `EPNUM` reader - EPNUM"]
pub type EPNUM_R = crate::FieldReader;
#[doc = "Field `EPNUM` writer - EPNUM"]
pub type EPNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EPDIR` reader - EPDIR"]
pub type EPDIR_R = crate::BitReader;
#[doc = "Field `EPDIR` writer - EPDIR"]
pub type EPDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSDEV` reader - LSDEV"]
pub type LSDEV_R = crate::BitReader;
#[doc = "Field `LSDEV` writer - LSDEV"]
pub type LSDEV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTYP` reader - EPTYP"]
pub type EPTYP_R = crate::FieldReader;
#[doc = "Field `EPTYP` writer - EPTYP"]
pub type EPTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCNT` reader - MCNT"]
pub type MCNT_R = crate::FieldReader;
#[doc = "Field `MCNT` writer - MCNT"]
pub type MCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DAD` reader - DAD"]
pub type DAD_R = crate::FieldReader;
#[doc = "Field `DAD` writer - DAD"]
pub type DAD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CHDIS` reader - CHDIS"]
pub type CHDIS_R = crate::BitReader;
#[doc = "Field `CHDIS` writer - CHDIS"]
pub type CHDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHENA` reader - CHENA"]
pub type CHENA_R = crate::BitReader;
#[doc = "Field `CHENA` writer - CHENA"]
pub type CHENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - MPSIZ"]
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - EPNUM"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - EPDIR"]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - LSDEV"]
    #[inline(always)]
    pub fn lsdev(&self) -> LSDEV_R {
        LSDEV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - EPTYP"]
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - MCNT"]
    #[inline(always)]
    pub fn mcnt(&self) -> MCNT_R {
        MCNT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:28 - DAD"]
    #[inline(always)]
    pub fn dad(&self) -> DAD_R {
        DAD_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 30 - CHDIS"]
    #[inline(always)]
    pub fn chdis(&self) -> CHDIS_R {
        CHDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CHENA"]
    #[inline(always)]
    pub fn chena(&self) -> CHENA_R {
        CHENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - MPSIZ"]
    #[inline(always)]
    #[must_use]
    pub fn mpsiz(&mut self) -> MPSIZ_W<OTG_HCCHAR14rs> {
        MPSIZ_W::new(self, 0)
    }
    #[doc = "Bits 11:14 - EPNUM"]
    #[inline(always)]
    #[must_use]
    pub fn epnum(&mut self) -> EPNUM_W<OTG_HCCHAR14rs> {
        EPNUM_W::new(self, 11)
    }
    #[doc = "Bit 15 - EPDIR"]
    #[inline(always)]
    #[must_use]
    pub fn epdir(&mut self) -> EPDIR_W<OTG_HCCHAR14rs> {
        EPDIR_W::new(self, 15)
    }
    #[doc = "Bit 17 - LSDEV"]
    #[inline(always)]
    #[must_use]
    pub fn lsdev(&mut self) -> LSDEV_W<OTG_HCCHAR14rs> {
        LSDEV_W::new(self, 17)
    }
    #[doc = "Bits 18:19 - EPTYP"]
    #[inline(always)]
    #[must_use]
    pub fn eptyp(&mut self) -> EPTYP_W<OTG_HCCHAR14rs> {
        EPTYP_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - MCNT"]
    #[inline(always)]
    #[must_use]
    pub fn mcnt(&mut self) -> MCNT_W<OTG_HCCHAR14rs> {
        MCNT_W::new(self, 20)
    }
    #[doc = "Bits 22:28 - DAD"]
    #[inline(always)]
    #[must_use]
    pub fn dad(&mut self) -> DAD_W<OTG_HCCHAR14rs> {
        DAD_W::new(self, 22)
    }
    #[doc = "Bit 30 - CHDIS"]
    #[inline(always)]
    #[must_use]
    pub fn chdis(&mut self) -> CHDIS_W<OTG_HCCHAR14rs> {
        CHDIS_W::new(self, 30)
    }
    #[doc = "Bit 31 - CHENA"]
    #[inline(always)]
    #[must_use]
    pub fn chena(&mut self) -> CHENA_W<OTG_HCCHAR14rs> {
        CHENA_W::new(self, 31)
    }
}
#[doc = "OTG host channel 14 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcchar14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcchar14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HCCHAR14rs;
impl crate::RegisterSpec for OTG_HCCHAR14rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hcchar14::R`](R) reader structure"]
impl crate::Readable for OTG_HCCHAR14rs {}
#[doc = "`write(|w| ..)` method takes [`otg_hcchar14::W`](W) writer structure"]
impl crate::Writable for OTG_HCCHAR14rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_HCCHAR14 to value 0"]
impl crate::Resettable for OTG_HCCHAR14rs {
    const RESET_VALUE: u32 = 0;
}
