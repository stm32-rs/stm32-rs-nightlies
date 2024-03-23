#[doc = "Register `DOUTR26` reader"]
pub type R = crate::R<DOUTR26rs>;
#[doc = "Register `DOUTR26` writer"]
pub type W = crate::W<DOUTR26rs>;
#[doc = "Field `DOUT26` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT26_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT26` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT26_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout26(&self) -> DOUT26_R {
        DOUT26_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout26(&mut self) -> DOUT26_W<DOUTR26rs> {
        DOUT26_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr26::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr26::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR26rs;
impl crate::RegisterSpec for DOUTR26rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr26::R`](R) reader structure"]
impl crate::Readable for DOUTR26rs {}
#[doc = "`write(|w| ..)` method takes [`doutr26::W`](W) writer structure"]
impl crate::Writable for DOUTR26rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR26 to value 0"]
impl crate::Resettable for DOUTR26rs {
    const RESET_VALUE: u32 = 0;
}
