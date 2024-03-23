#[doc = "Register `DOUTR13` reader"]
pub type R = crate::R<DOUTR13rs>;
#[doc = "Register `DOUTR13` writer"]
pub type W = crate::W<DOUTR13rs>;
#[doc = "Field `DOUT13` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT13_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT13` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT13_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout13(&self) -> DOUT13_R {
        DOUT13_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout13(&mut self) -> DOUT13_W<DOUTR13rs> {
        DOUT13_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR13rs;
impl crate::RegisterSpec for DOUTR13rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr13::R`](R) reader structure"]
impl crate::Readable for DOUTR13rs {}
#[doc = "`write(|w| ..)` method takes [`doutr13::W`](W) writer structure"]
impl crate::Writable for DOUTR13rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR13 to value 0"]
impl crate::Resettable for DOUTR13rs {
    const RESET_VALUE: u32 = 0;
}
