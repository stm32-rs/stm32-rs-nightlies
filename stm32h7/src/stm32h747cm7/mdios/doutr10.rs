#[doc = "Register `DOUTR10` reader"]
pub type R = crate::R<DOUTR10rs>;
#[doc = "Register `DOUTR10` writer"]
pub type W = crate::W<DOUTR10rs>;
#[doc = "Field `DOUT10` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT10_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT10` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT10_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout10(&self) -> DOUT10_R {
        DOUT10_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout10(&mut self) -> DOUT10_W<DOUTR10rs> {
        DOUT10_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR10rs;
impl crate::RegisterSpec for DOUTR10rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr10::R`](R) reader structure"]
impl crate::Readable for DOUTR10rs {}
#[doc = "`write(|w| ..)` method takes [`doutr10::W`](W) writer structure"]
impl crate::Writable for DOUTR10rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR10 to value 0"]
impl crate::Resettable for DOUTR10rs {
    const RESET_VALUE: u32 = 0;
}
