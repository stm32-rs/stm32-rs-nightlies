#[doc = "Register `DOUTR22` reader"]
pub type R = crate::R<DOUTR22rs>;
#[doc = "Register `DOUTR22` writer"]
pub type W = crate::W<DOUTR22rs>;
#[doc = "Field `DOUT22` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT22_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT22` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT22_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout22(&self) -> DOUT22_R {
        DOUT22_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout22(&mut self) -> DOUT22_W<DOUTR22rs> {
        DOUT22_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr22::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr22::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR22rs;
impl crate::RegisterSpec for DOUTR22rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr22::R`](R) reader structure"]
impl crate::Readable for DOUTR22rs {}
#[doc = "`write(|w| ..)` method takes [`doutr22::W`](W) writer structure"]
impl crate::Writable for DOUTR22rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR22 to value 0"]
impl crate::Resettable for DOUTR22rs {
    const RESET_VALUE: u32 = 0;
}
