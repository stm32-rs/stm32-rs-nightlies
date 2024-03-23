#[doc = "Register `DOUTR%s` reader"]
pub type R = crate::R<DOUTRrs>;
#[doc = "Register `DOUTR%s` writer"]
pub type W = crate::W<DOUTRrs>;
#[doc = "Field `DOUT` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout(&mut self) -> DOUT_W<DOUTRrs> {
        DOUT_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTRrs;
impl crate::RegisterSpec for DOUTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr::R`](R) reader structure"]
impl crate::Readable for DOUTRrs {}
#[doc = "`write(|w| ..)` method takes [`doutr::W`](W) writer structure"]
impl crate::Writable for DOUTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR%s to value 0"]
impl crate::Resettable for DOUTRrs {
    const RESET_VALUE: u32 = 0;
}
