#[doc = "Register `DOUTR11` reader"]
pub type R = crate::R<DOUTR11rs>;
#[doc = "Register `DOUTR11` writer"]
pub type W = crate::W<DOUTR11rs>;
#[doc = "Field `DOUT11` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT11_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT11` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT11_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout11(&self) -> DOUT11_R {
        DOUT11_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout11(&mut self) -> DOUT11_W<DOUTR11rs> {
        DOUT11_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR11rs;
impl crate::RegisterSpec for DOUTR11rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr11::R`](R) reader structure"]
impl crate::Readable for DOUTR11rs {}
#[doc = "`write(|w| ..)` method takes [`doutr11::W`](W) writer structure"]
impl crate::Writable for DOUTR11rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR11 to value 0"]
impl crate::Resettable for DOUTR11rs {
    const RESET_VALUE: u32 = 0;
}
