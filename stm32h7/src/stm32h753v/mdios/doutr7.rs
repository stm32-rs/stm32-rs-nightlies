#[doc = "Register `DOUTR7` reader"]
pub type R = crate::R<DOUTR7rs>;
#[doc = "Register `DOUTR7` writer"]
pub type W = crate::W<DOUTR7rs>;
#[doc = "Field `DOUT7` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT7_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT7` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT7_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout7(&self) -> DOUT7_R {
        DOUT7_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout7(&mut self) -> DOUT7_W<DOUTR7rs> {
        DOUT7_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR7rs;
impl crate::RegisterSpec for DOUTR7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr7::R`](R) reader structure"]
impl crate::Readable for DOUTR7rs {}
#[doc = "`write(|w| ..)` method takes [`doutr7::W`](W) writer structure"]
impl crate::Writable for DOUTR7rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR7 to value 0"]
impl crate::Resettable for DOUTR7rs {
    const RESET_VALUE: u32 = 0;
}
