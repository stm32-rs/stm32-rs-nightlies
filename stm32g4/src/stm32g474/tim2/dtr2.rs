#[doc = "Register `DTR2` reader"]
pub type R = crate::R<DTR2rs>;
#[doc = "Register `DTR2` writer"]
pub type W = crate::W<DTR2rs>;
#[doc = "Field `DTGF` reader - Dead-time falling edge generator setup"]
pub type DTGF_R = crate::FieldReader;
#[doc = "Field `DTGF` writer - Dead-time falling edge generator setup"]
pub type DTGF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DTAE` reader - Deadtime Asymmetric Enable"]
pub type DTAE_R = crate::BitReader;
#[doc = "Field `DTAE` writer - Deadtime Asymmetric Enable"]
pub type DTAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTPE` reader - Deadtime Preload Enable"]
pub type DTPE_R = crate::BitReader;
#[doc = "Field `DTPE` writer - Deadtime Preload Enable"]
pub type DTPE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Dead-time falling edge generator setup"]
    #[inline(always)]
    pub fn dtgf(&self) -> DTGF_R {
        DTGF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Deadtime Asymmetric Enable"]
    #[inline(always)]
    pub fn dtae(&self) -> DTAE_R {
        DTAE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Deadtime Preload Enable"]
    #[inline(always)]
    pub fn dtpe(&self) -> DTPE_R {
        DTPE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead-time falling edge generator setup"]
    #[inline(always)]
    #[must_use]
    pub fn dtgf(&mut self) -> DTGF_W<DTR2rs> {
        DTGF_W::new(self, 0)
    }
    #[doc = "Bit 16 - Deadtime Asymmetric Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtae(&mut self) -> DTAE_W<DTR2rs> {
        DTAE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Deadtime Preload Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtpe(&mut self) -> DTPE_W<DTR2rs> {
        DTPE_W::new(self, 17)
    }
}
#[doc = "timer Deadtime Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTR2rs;
impl crate::RegisterSpec for DTR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtr2::R`](R) reader structure"]
impl crate::Readable for DTR2rs {}
#[doc = "`write(|w| ..)` method takes [`dtr2::W`](W) writer structure"]
impl crate::Writable for DTR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTR2 to value 0"]
impl crate::Resettable for DTR2rs {
    const RESET_VALUE: u32 = 0;
}
