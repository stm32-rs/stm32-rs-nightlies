#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<ISRrs>;
#[doc = "Field `EOAF` reader - End of acquisition flag"]
pub type EOAF_R = crate::BitReader;
#[doc = "Field `EOAF` writer - End of acquisition flag"]
pub type EOAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCEF` reader - Max count error flag"]
pub type MCEF_R = crate::BitReader;
#[doc = "Field `MCEF` writer - Max count error flag"]
pub type MCEF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - End of acquisition flag"]
    #[inline(always)]
    pub fn eoaf(&self) -> EOAF_R {
        EOAF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Max count error flag"]
    #[inline(always)]
    pub fn mcef(&self) -> MCEF_R {
        MCEF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of acquisition flag"]
    #[inline(always)]
    #[must_use]
    pub fn eoaf(&mut self) -> EOAF_W<ISRrs> {
        EOAF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Max count error flag"]
    #[inline(always)]
    #[must_use]
    pub fn mcef(&mut self) -> MCEF_W<ISRrs> {
        MCEF_W::new(self, 1)
    }
}
#[doc = "interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISRrs {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0;
}
