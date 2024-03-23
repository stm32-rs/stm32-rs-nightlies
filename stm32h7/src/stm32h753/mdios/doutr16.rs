#[doc = "Register `DOUTR16` reader"]
pub type R = crate::R<DOUTR16rs>;
#[doc = "Register `DOUTR16` writer"]
pub type W = crate::W<DOUTR16rs>;
#[doc = "Field `DOUT16` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT16_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT16` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT16_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout16(&self) -> DOUT16_R {
        DOUT16_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout16(&mut self) -> DOUT16_W<DOUTR16rs> {
        DOUT16_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR16rs;
impl crate::RegisterSpec for DOUTR16rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr16::R`](R) reader structure"]
impl crate::Readable for DOUTR16rs {}
#[doc = "`write(|w| ..)` method takes [`doutr16::W`](W) writer structure"]
impl crate::Writable for DOUTR16rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR16 to value 0"]
impl crate::Resettable for DOUTR16rs {
    const RESET_VALUE: u32 = 0;
}
