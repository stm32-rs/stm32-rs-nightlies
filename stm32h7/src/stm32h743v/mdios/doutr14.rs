#[doc = "Register `DOUTR14` reader"]
pub type R = crate::R<DOUTR14rs>;
#[doc = "Register `DOUTR14` writer"]
pub type W = crate::W<DOUTR14rs>;
#[doc = "Field `DOUT14` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT14_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT14` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT14_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout14(&self) -> DOUT14_R {
        DOUT14_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout14(&mut self) -> DOUT14_W<DOUTR14rs> {
        DOUT14_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR14rs;
impl crate::RegisterSpec for DOUTR14rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr14::R`](R) reader structure"]
impl crate::Readable for DOUTR14rs {}
#[doc = "`write(|w| ..)` method takes [`doutr14::W`](W) writer structure"]
impl crate::Writable for DOUTR14rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR14 to value 0"]
impl crate::Resettable for DOUTR14rs {
    const RESET_VALUE: u32 = 0;
}
