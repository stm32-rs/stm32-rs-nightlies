#[doc = "Register `DDRPHYC_ZQ0CR1` reader"]
pub type R = crate::R<DDRPHYC_ZQ0CR1rs>;
#[doc = "Register `DDRPHYC_ZQ0CR1` writer"]
pub type W = crate::W<DDRPHYC_ZQ0CR1rs>;
#[doc = "Field `ZPROG` reader - ZPROG"]
pub type ZPROG_R = crate::FieldReader;
#[doc = "Field `ZPROG` writer - ZPROG"]
pub type ZPROG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ZPROG"]
    #[inline(always)]
    pub fn zprog(&self) -> ZPROG_R {
        ZPROG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - ZPROG"]
    #[inline(always)]
    #[must_use]
    pub fn zprog(&mut self) -> ZPROG_W<DDRPHYC_ZQ0CR1rs> {
        ZPROG_W::new(self, 0)
    }
}
#[doc = "DDRPHYC ZQ0CR1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_zq0cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_zq0cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_ZQ0CR1rs;
impl crate::RegisterSpec for DDRPHYC_ZQ0CR1rs {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ddrphyc_zq0cr1::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_ZQ0CR1rs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_zq0cr1::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_ZQ0CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_ZQ0CR1 to value 0x7b"]
impl crate::Resettable for DDRPHYC_ZQ0CR1rs {
    const RESET_VALUE: u8 = 0x7b;
}
