#[doc = "Register `DOUTR15` reader"]
pub type R = crate::R<DOUTR15rs>;
#[doc = "Register `DOUTR15` writer"]
pub type W = crate::W<DOUTR15rs>;
#[doc = "Field `DOUT15` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT15_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT15` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT15_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout15(&self) -> DOUT15_R {
        DOUT15_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout15(&mut self) -> DOUT15_W<DOUTR15rs> {
        DOUT15_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR15rs;
impl crate::RegisterSpec for DOUTR15rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr15::R`](R) reader structure"]
impl crate::Readable for DOUTR15rs {}
#[doc = "`write(|w| ..)` method takes [`doutr15::W`](W) writer structure"]
impl crate::Writable for DOUTR15rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR15 to value 0"]
impl crate::Resettable for DOUTR15rs {
    const RESET_VALUE: u32 = 0;
}
