#[doc = "Register `DOUTR23` reader"]
pub type R = crate::R<DOUTR23rs>;
#[doc = "Register `DOUTR23` writer"]
pub type W = crate::W<DOUTR23rs>;
#[doc = "Field `DOUT23` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT23_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT23` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT23_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout23(&self) -> DOUT23_R {
        DOUT23_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout23(&mut self) -> DOUT23_W<DOUTR23rs> {
        DOUT23_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR23rs;
impl crate::RegisterSpec for DOUTR23rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr23::R`](R) reader structure"]
impl crate::Readable for DOUTR23rs {}
#[doc = "`write(|w| ..)` method takes [`doutr23::W`](W) writer structure"]
impl crate::Writable for DOUTR23rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR23 to value 0"]
impl crate::Resettable for DOUTR23rs {
    const RESET_VALUE: u32 = 0;
}
