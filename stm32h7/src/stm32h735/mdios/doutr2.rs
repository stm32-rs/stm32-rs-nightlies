#[doc = "Register `DOUTR2` reader"]
pub type R = crate::R<DOUTR2rs>;
#[doc = "Register `DOUTR2` writer"]
pub type W = crate::W<DOUTR2rs>;
#[doc = "Field `DOUT2` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT2_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT2` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout2(&self) -> DOUT2_R {
        DOUT2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout2(&mut self) -> DOUT2_W<DOUTR2rs> {
        DOUT2_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR2rs;
impl crate::RegisterSpec for DOUTR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr2::R`](R) reader structure"]
impl crate::Readable for DOUTR2rs {}
#[doc = "`write(|w| ..)` method takes [`doutr2::W`](W) writer structure"]
impl crate::Writable for DOUTR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR2 to value 0"]
impl crate::Resettable for DOUTR2rs {
    const RESET_VALUE: u32 = 0;
}
