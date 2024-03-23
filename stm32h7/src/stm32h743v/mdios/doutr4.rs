#[doc = "Register `DOUTR4` reader"]
pub type R = crate::R<DOUTR4rs>;
#[doc = "Register `DOUTR4` writer"]
pub type W = crate::W<DOUTR4rs>;
#[doc = "Field `DOUT4` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT4_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT4` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT4_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout4(&self) -> DOUT4_R {
        DOUT4_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout4(&mut self) -> DOUT4_W<DOUTR4rs> {
        DOUT4_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR4rs;
impl crate::RegisterSpec for DOUTR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr4::R`](R) reader structure"]
impl crate::Readable for DOUTR4rs {}
#[doc = "`write(|w| ..)` method takes [`doutr4::W`](W) writer structure"]
impl crate::Writable for DOUTR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR4 to value 0"]
impl crate::Resettable for DOUTR4rs {
    const RESET_VALUE: u32 = 0;
}
