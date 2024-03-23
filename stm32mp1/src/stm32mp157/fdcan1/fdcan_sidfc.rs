#[doc = "Register `FDCAN_SIDFC` reader"]
pub type R = crate::R<FDCAN_SIDFCrs>;
#[doc = "Register `FDCAN_SIDFC` writer"]
pub type W = crate::W<FDCAN_SIDFCrs>;
#[doc = "Field `FLSSA` reader - FLSSA"]
pub type FLSSA_R = crate::FieldReader<u16>;
#[doc = "Field `FLSSA` writer - FLSSA"]
pub type FLSSA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `LSS` reader - LSS"]
pub type LSS_R = crate::FieldReader;
#[doc = "Field `LSS` writer - LSS"]
pub type LSS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 2:15 - FLSSA"]
    #[inline(always)]
    pub fn flssa(&self) -> FLSSA_R {
        FLSSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:23 - LSS"]
    #[inline(always)]
    pub fn lss(&self) -> LSS_R {
        LSS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - FLSSA"]
    #[inline(always)]
    #[must_use]
    pub fn flssa(&mut self) -> FLSSA_W<FDCAN_SIDFCrs> {
        FLSSA_W::new(self, 2)
    }
    #[doc = "Bits 16:23 - LSS"]
    #[inline(always)]
    #[must_use]
    pub fn lss(&mut self) -> LSS_W<FDCAN_SIDFCrs> {
        LSS_W::new(self, 16)
    }
}
#[doc = "Settings for 11-bit standard message ID filtering.The standard ID filter configuration register controls the filter path for standard messages as described in Figure708.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_sidfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_sidfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_SIDFCrs;
impl crate::RegisterSpec for FDCAN_SIDFCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_sidfc::R`](R) reader structure"]
impl crate::Readable for FDCAN_SIDFCrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_sidfc::W`](W) writer structure"]
impl crate::Writable for FDCAN_SIDFCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_SIDFC to value 0"]
impl crate::Resettable for FDCAN_SIDFCrs {
    const RESET_VALUE: u32 = 0;
}
