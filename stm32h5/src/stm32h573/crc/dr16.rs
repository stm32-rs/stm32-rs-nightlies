#[doc = "Register `DR16` reader"]
pub type R = crate::R<DR16rs>;
#[doc = "Register `DR16` writer"]
pub type W = crate::W<DR16rs>;
#[doc = "Field `DR16` reader - Data register bits"]
pub type DR16_R = crate::FieldReader<u16>;
#[doc = "Field `DR16` writer - Data register bits"]
pub type DR16_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data register bits"]
    #[inline(always)]
    pub fn dr16(&self) -> DR16_R {
        DR16_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data register bits"]
    #[inline(always)]
    #[must_use]
    pub fn dr16(&mut self) -> DR16_W<DR16rs> {
        DR16_W::new(self, 0)
    }
}
#[doc = "Data register - half-word sized\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR16rs;
impl crate::RegisterSpec for DR16rs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dr16::R`](R) reader structure"]
impl crate::Readable for DR16rs {}
#[doc = "`write(|w| ..)` method takes [`dr16::W`](W) writer structure"]
impl crate::Writable for DR16rs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DR16 to value 0xffff"]
impl crate::Resettable for DR16rs {
    const RESET_VALUE: u16 = 0xffff;
}
