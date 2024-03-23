#[doc = "Register `DOUTR0` reader"]
pub type R = crate::R<DOUTR0rs>;
#[doc = "Register `DOUTR0` writer"]
pub type W = crate::W<DOUTR0rs>;
#[doc = "Field `DOUT0` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT0_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT0` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout0(&self) -> DOUT0_R {
        DOUT0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout0(&mut self) -> DOUT0_W<DOUTR0rs> {
        DOUT0_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR0rs;
impl crate::RegisterSpec for DOUTR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr0::R`](R) reader structure"]
impl crate::Readable for DOUTR0rs {}
#[doc = "`write(|w| ..)` method takes [`doutr0::W`](W) writer structure"]
impl crate::Writable for DOUTR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR0 to value 0"]
impl crate::Resettable for DOUTR0rs {
    const RESET_VALUE: u32 = 0;
}
