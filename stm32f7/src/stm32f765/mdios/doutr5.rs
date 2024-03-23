#[doc = "Register `DOUTR5` reader"]
pub type R = crate::R<DOUTR5rs>;
#[doc = "Register `DOUTR5` writer"]
pub type W = crate::W<DOUTR5rs>;
#[doc = "Field `DOUT5` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT5_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT5` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT5_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout5(&self) -> DOUT5_R {
        DOUT5_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout5(&mut self) -> DOUT5_W<DOUTR5rs> {
        DOUT5_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR5rs;
impl crate::RegisterSpec for DOUTR5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr5::R`](R) reader structure"]
impl crate::Readable for DOUTR5rs {}
#[doc = "`write(|w| ..)` method takes [`doutr5::W`](W) writer structure"]
impl crate::Writable for DOUTR5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR5 to value 0"]
impl crate::Resettable for DOUTR5rs {
    const RESET_VALUE: u32 = 0;
}
