#[doc = "Register `DOUTR9` reader"]
pub type R = crate::R<DOUTR9rs>;
#[doc = "Register `DOUTR9` writer"]
pub type W = crate::W<DOUTR9rs>;
#[doc = "Field `DOUT9` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT9_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT9` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT9_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout9(&self) -> DOUT9_R {
        DOUT9_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout9(&mut self) -> DOUT9_W<DOUTR9rs> {
        DOUT9_W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTR9rs;
impl crate::RegisterSpec for DOUTR9rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr9::R`](R) reader structure"]
impl crate::Readable for DOUTR9rs {}
#[doc = "`write(|w| ..)` method takes [`doutr9::W`](W) writer structure"]
impl crate::Writable for DOUTR9rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR9 to value 0"]
impl crate::Resettable for DOUTR9rs {
    const RESET_VALUE: u32 = 0;
}
