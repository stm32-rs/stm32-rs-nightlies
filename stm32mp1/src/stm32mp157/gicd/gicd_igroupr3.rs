#[doc = "Register `GICD_IGROUPR3` reader"]
pub type R = crate::R<GICD_IGROUPR3rs>;
#[doc = "Register `GICD_IGROUPR3` writer"]
pub type W = crate::W<GICD_IGROUPR3rs>;
#[doc = "Field `IGROUPR3` reader - IGROUPR3"]
pub type IGROUPR3_R = crate::FieldReader<u32>;
#[doc = "Field `IGROUPR3` writer - IGROUPR3"]
pub type IGROUPR3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IGROUPR3"]
    #[inline(always)]
    pub fn igroupr3(&self) -> IGROUPR3_R {
        IGROUPR3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IGROUPR3"]
    #[inline(always)]
    #[must_use]
    pub fn igroupr3(&mut self) -> IGROUPR3_W<GICD_IGROUPR3rs> {
        IGROUPR3_W::new(self, 0)
    }
}
#[doc = "For interrupts ID = x*32 to ID = x*32+31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_igroupr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_igroupr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IGROUPR3rs;
impl crate::RegisterSpec for GICD_IGROUPR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_igroupr3::R`](R) reader structure"]
impl crate::Readable for GICD_IGROUPR3rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_igroupr3::W`](W) writer structure"]
impl crate::Writable for GICD_IGROUPR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IGROUPR3 to value 0"]
impl crate::Resettable for GICD_IGROUPR3rs {
    const RESET_VALUE: u32 = 0;
}
