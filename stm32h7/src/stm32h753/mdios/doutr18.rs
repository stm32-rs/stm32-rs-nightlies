#[doc = "Register `DOUTR18` reader"]
pub type R = crate::R<DOUTR18rs>;
#[doc = "Register `DOUTR18` writer"]
pub type W = crate::W<DOUTR18rs>;
#[doc = "Field `DOUT18` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT18_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT18` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT18_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout18(&self) -> DOUT18_R {
        DOUT18_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout18(&mut self) -> DOUT18_W<DOUTR18rs> {
        DOUT18_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr18::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr18::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR18rs;
impl crate::RegisterSpec for DOUTR18rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr18::R`](R) reader structure"]
impl crate::Readable for DOUTR18rs {}
#[doc = "`write(|w| ..)` method takes [`doutr18::W`](W) writer structure"]
impl crate::Writable for DOUTR18rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR18 to value 0"]
impl crate::Resettable for DOUTR18rs {
    const RESET_VALUE: u32 = 0;
}
