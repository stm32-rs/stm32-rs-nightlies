#[doc = "Register `DOUTR28` reader"]
pub type R = crate::R<DOUTR28rs>;
#[doc = "Register `DOUTR28` writer"]
pub type W = crate::W<DOUTR28rs>;
#[doc = "Field `DOUT28` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT28_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT28` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT28_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout28(&self) -> DOUT28_R {
        DOUT28_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout28(&mut self) -> DOUT28_W<DOUTR28rs> {
        DOUT28_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr28::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr28::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR28rs;
impl crate::RegisterSpec for DOUTR28rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr28::R`](R) reader structure"]
impl crate::Readable for DOUTR28rs {}
#[doc = "`write(|w| ..)` method takes [`doutr28::W`](W) writer structure"]
impl crate::Writable for DOUTR28rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR28 to value 0"]
impl crate::Resettable for DOUTR28rs {
    const RESET_VALUE: u32 = 0;
}
