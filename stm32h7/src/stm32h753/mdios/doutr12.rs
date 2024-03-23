#[doc = "Register `DOUTR12` reader"]
pub type R = crate::R<DOUTR12rs>;
#[doc = "Register `DOUTR12` writer"]
pub type W = crate::W<DOUTR12rs>;
#[doc = "Field `DOUT12` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT12_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT12` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT12_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout12(&self) -> DOUT12_R {
        DOUT12_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout12(&mut self) -> DOUT12_W<DOUTR12rs> {
        DOUT12_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR12rs;
impl crate::RegisterSpec for DOUTR12rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr12::R`](R) reader structure"]
impl crate::Readable for DOUTR12rs {}
#[doc = "`write(|w| ..)` method takes [`doutr12::W`](W) writer structure"]
impl crate::Writable for DOUTR12rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR12 to value 0"]
impl crate::Resettable for DOUTR12rs {
    const RESET_VALUE: u32 = 0;
}
