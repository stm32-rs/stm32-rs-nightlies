#[doc = "Register `DOUTR21` reader"]
pub type R = crate::R<DOUTR21rs>;
#[doc = "Register `DOUTR21` writer"]
pub type W = crate::W<DOUTR21rs>;
#[doc = "Field `DOUT21` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT21_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT21` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT21_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout21(&self) -> DOUT21_R {
        DOUT21_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout21(&mut self) -> DOUT21_W<DOUTR21rs> {
        DOUT21_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR21rs;
impl crate::RegisterSpec for DOUTR21rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr21::R`](R) reader structure"]
impl crate::Readable for DOUTR21rs {}
#[doc = "`write(|w| ..)` method takes [`doutr21::W`](W) writer structure"]
impl crate::Writable for DOUTR21rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR21 to value 0"]
impl crate::Resettable for DOUTR21rs {
    const RESET_VALUE: u32 = 0;
}
