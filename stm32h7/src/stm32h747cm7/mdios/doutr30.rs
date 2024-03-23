#[doc = "Register `DOUTR30` reader"]
pub type R = crate::R<DOUTR30rs>;
#[doc = "Register `DOUTR30` writer"]
pub type W = crate::W<DOUTR30rs>;
#[doc = "Field `DOUT30` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT30_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT30` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT30_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout30(&self) -> DOUT30_R {
        DOUT30_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout30(&mut self) -> DOUT30_W<DOUTR30rs> {
        DOUT30_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr30::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr30::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR30rs;
impl crate::RegisterSpec for DOUTR30rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr30::R`](R) reader structure"]
impl crate::Readable for DOUTR30rs {}
#[doc = "`write(|w| ..)` method takes [`doutr30::W`](W) writer structure"]
impl crate::Writable for DOUTR30rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR30 to value 0"]
impl crate::Resettable for DOUTR30rs {
    const RESET_VALUE: u32 = 0;
}
