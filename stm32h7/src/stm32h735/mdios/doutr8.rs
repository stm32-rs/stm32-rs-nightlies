#[doc = "Register `DOUTR8` reader"]
pub type R = crate::R<DOUTR8rs>;
#[doc = "Register `DOUTR8` writer"]
pub type W = crate::W<DOUTR8rs>;
#[doc = "Field `DOUT8` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT8_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT8` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT8_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout8(&self) -> DOUT8_R {
        DOUT8_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout8(&mut self) -> DOUT8_W<DOUTR8rs> {
        DOUT8_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR8rs;
impl crate::RegisterSpec for DOUTR8rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr8::R`](R) reader structure"]
impl crate::Readable for DOUTR8rs {}
#[doc = "`write(|w| ..)` method takes [`doutr8::W`](W) writer structure"]
impl crate::Writable for DOUTR8rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR8 to value 0"]
impl crate::Resettable for DOUTR8rs {
    const RESET_VALUE: u32 = 0;
}
