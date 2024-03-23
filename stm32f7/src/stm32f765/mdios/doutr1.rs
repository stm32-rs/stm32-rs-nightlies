#[doc = "Register `DOUTR1` reader"]
pub type R = crate::R<DOUTR1rs>;
#[doc = "Register `DOUTR1` writer"]
pub type W = crate::W<DOUTR1rs>;
#[doc = "Field `DOUT1` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT1_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT1` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout1(&self) -> DOUT1_R {
        DOUT1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout1(&mut self) -> DOUT1_W<DOUTR1rs> {
        DOUT1_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR1rs;
impl crate::RegisterSpec for DOUTR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr1::R`](R) reader structure"]
impl crate::Readable for DOUTR1rs {}
#[doc = "`write(|w| ..)` method takes [`doutr1::W`](W) writer structure"]
impl crate::Writable for DOUTR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR1 to value 0"]
impl crate::Resettable for DOUTR1rs {
    const RESET_VALUE: u32 = 0;
}
