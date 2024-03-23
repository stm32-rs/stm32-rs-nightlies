#[doc = "Register `ETZPC_TZMA1_SIZE` reader"]
pub type R = crate::R<ETZPC_TZMA1_SIZErs>;
#[doc = "Register `ETZPC_TZMA1_SIZE` writer"]
pub type W = crate::W<ETZPC_TZMA1_SIZErs>;
#[doc = "Field `R0SIZE` reader - R0SIZE"]
pub type R0SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `R0SIZE` writer - R0SIZE"]
pub type R0SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `LOCK` reader - LOCK"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - LOCK"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - R0SIZE"]
    #[inline(always)]
    pub fn r0size(&self) -> R0SIZE_R {
        R0SIZE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - R0SIZE"]
    #[inline(always)]
    #[must_use]
    pub fn r0size(&mut self) -> R0SIZE_W<ETZPC_TZMA1_SIZErs> {
        R0SIZE_W::new(self, 0)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<ETZPC_TZMA1_SIZErs> {
        LOCK_W::new(self, 31)
    }
}
#[doc = "ETZPC RAM secure size definition\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etzpc_tzma1_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etzpc_tzma1_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETZPC_TZMA1_SIZErs;
impl crate::RegisterSpec for ETZPC_TZMA1_SIZErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etzpc_tzma1_size::R`](R) reader structure"]
impl crate::Readable for ETZPC_TZMA1_SIZErs {}
#[doc = "`write(|w| ..)` method takes [`etzpc_tzma1_size::W`](W) writer structure"]
impl crate::Writable for ETZPC_TZMA1_SIZErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETZPC_TZMA1_SIZE to value 0x03ff"]
impl crate::Resettable for ETZPC_TZMA1_SIZErs {
    const RESET_VALUE: u32 = 0x03ff;
}
