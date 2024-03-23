#[doc = "Register `RCC_MP_AXIMLPENSETR` reader"]
pub type R = crate::R<RCC_MP_AXIMLPENSETRrs>;
#[doc = "Register `RCC_MP_AXIMLPENSETR` writer"]
pub type W = crate::W<RCC_MP_AXIMLPENSETRrs>;
#[doc = "Field `SYSRAMLPEN` reader - SYSRAMLPEN"]
pub type SYSRAMLPEN_R = crate::BitReader;
#[doc = "Field `SYSRAMLPEN` writer - SYSRAMLPEN"]
pub type SYSRAMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYSRAMLPEN"]
    #[inline(always)]
    pub fn sysramlpen(&self) -> SYSRAMLPEN_R {
        SYSRAMLPEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSRAMLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn sysramlpen(&mut self) -> SYSRAMLPEN_W<RCC_MP_AXIMLPENSETRrs> {
        SYSRAMLPEN_W::new(self, 0)
    }
}
#[doc = "This register is used by the MPU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_aximlpensetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_aximlpensetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MP_AXIMLPENSETRrs;
impl crate::RegisterSpec for RCC_MP_AXIMLPENSETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mp_aximlpensetr::R`](R) reader structure"]
impl crate::Readable for RCC_MP_AXIMLPENSETRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mp_aximlpensetr::W`](W) writer structure"]
impl crate::Writable for RCC_MP_AXIMLPENSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MP_AXIMLPENSETR to value 0x01"]
impl crate::Resettable for RCC_MP_AXIMLPENSETRrs {
    const RESET_VALUE: u32 = 0x01;
}
