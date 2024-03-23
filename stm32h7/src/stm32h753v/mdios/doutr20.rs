#[doc = "Register `DOUTR20` reader"]
pub type R = crate::R<DOUTR20rs>;
#[doc = "Register `DOUTR20` writer"]
pub type W = crate::W<DOUTR20rs>;
#[doc = "Field `DOUT20` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT20_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT20` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT20_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout20(&self) -> DOUT20_R {
        DOUT20_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout20(&mut self) -> DOUT20_W<DOUTR20rs> {
        DOUT20_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR20rs;
impl crate::RegisterSpec for DOUTR20rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr20::R`](R) reader structure"]
impl crate::Readable for DOUTR20rs {}
#[doc = "`write(|w| ..)` method takes [`doutr20::W`](W) writer structure"]
impl crate::Writable for DOUTR20rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR20 to value 0"]
impl crate::Resettable for DOUTR20rs {
    const RESET_VALUE: u32 = 0;
}
