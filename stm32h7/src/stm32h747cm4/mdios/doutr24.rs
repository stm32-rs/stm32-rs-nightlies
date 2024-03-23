#[doc = "Register `DOUTR24` reader"]
pub type R = crate::R<DOUTR24rs>;
#[doc = "Register `DOUTR24` writer"]
pub type W = crate::W<DOUTR24rs>;
#[doc = "Field `DOUT24` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT24_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT24` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT24_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout24(&self) -> DOUT24_R {
        DOUT24_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout24(&mut self) -> DOUT24_W<DOUTR24rs> {
        DOUT24_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr24::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR24rs;
impl crate::RegisterSpec for DOUTR24rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr24::R`](R) reader structure"]
impl crate::Readable for DOUTR24rs {}
#[doc = "`write(|w| ..)` method takes [`doutr24::W`](W) writer structure"]
impl crate::Writable for DOUTR24rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR24 to value 0"]
impl crate::Resettable for DOUTR24rs {
    const RESET_VALUE: u32 = 0;
}
