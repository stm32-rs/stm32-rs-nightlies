#[doc = "Register `DOUTR3` reader"]
pub type R = crate::R<DOUTR3rs>;
#[doc = "Register `DOUTR3` writer"]
pub type W = crate::W<DOUTR3rs>;
#[doc = "Field `DOUT3` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT3_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT3` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT3_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout3(&self) -> DOUT3_R {
        DOUT3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout3(&mut self) -> DOUT3_W<DOUTR3rs> {
        DOUT3_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR3rs;
impl crate::RegisterSpec for DOUTR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr3::R`](R) reader structure"]
impl crate::Readable for DOUTR3rs {}
#[doc = "`write(|w| ..)` method takes [`doutr3::W`](W) writer structure"]
impl crate::Writable for DOUTR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR3 to value 0"]
impl crate::Resettable for DOUTR3rs {
    const RESET_VALUE: u32 = 0;
}
