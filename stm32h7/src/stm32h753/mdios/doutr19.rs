#[doc = "Register `DOUTR19` reader"]
pub type R = crate::R<DOUTR19rs>;
#[doc = "Register `DOUTR19` writer"]
pub type W = crate::W<DOUTR19rs>;
#[doc = "Field `DOUT19` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT19_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT19` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT19_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout19(&self) -> DOUT19_R {
        DOUT19_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout19(&mut self) -> DOUT19_W<DOUTR19rs> {
        DOUT19_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR19rs;
impl crate::RegisterSpec for DOUTR19rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr19::R`](R) reader structure"]
impl crate::Readable for DOUTR19rs {}
#[doc = "`write(|w| ..)` method takes [`doutr19::W`](W) writer structure"]
impl crate::Writable for DOUTR19rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR19 to value 0"]
impl crate::Resettable for DOUTR19rs {
    const RESET_VALUE: u32 = 0;
}
