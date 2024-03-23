#[doc = "Register `DCR3` reader"]
pub type R = crate::R<DCR3rs>;
#[doc = "Register `DCR3` writer"]
pub type W = crate::W<DCR3rs>;
#[doc = "Field `CSBOUND` reader - NCS boundary This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended. The NCS is released on each boundary of 2CSBOUND bytes. others: NCS boundary set to 2CSBOUND bytes"]
pub type CSBOUND_R = crate::FieldReader;
#[doc = "Field `CSBOUND` writer - NCS boundary This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended. The NCS is released on each boundary of 2CSBOUND bytes. others: NCS boundary set to 2CSBOUND bytes"]
pub type CSBOUND_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 16:20 - NCS boundary This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended. The NCS is released on each boundary of 2CSBOUND bytes. others: NCS boundary set to 2CSBOUND bytes"]
    #[inline(always)]
    pub fn csbound(&self) -> CSBOUND_R {
        CSBOUND_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:20 - NCS boundary This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended. The NCS is released on each boundary of 2CSBOUND bytes. others: NCS boundary set to 2CSBOUND bytes"]
    #[inline(always)]
    #[must_use]
    pub fn csbound(&mut self) -> CSBOUND_W<DCR3rs> {
        CSBOUND_W::new(self, 16)
    }
}
#[doc = "OCTOSPI device configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCR3rs;
impl crate::RegisterSpec for DCR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcr3::R`](R) reader structure"]
impl crate::Readable for DCR3rs {}
#[doc = "`write(|w| ..)` method takes [`dcr3::W`](W) writer structure"]
impl crate::Writable for DCR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCR3 to value 0"]
impl crate::Resettable for DCR3rs {
    const RESET_VALUE: u32 = 0;
}
