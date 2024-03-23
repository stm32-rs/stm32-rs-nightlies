#[doc = "Register `SECBB2R1` reader"]
pub type R = crate::R<SECBB2R1rs>;
#[doc = "Register `SECBB2R1` writer"]
pub type W = crate::W<SECBB2R1rs>;
#[doc = "Field `SECBB2` reader - Secure/non-secure flash Bank 2 sector attribute"]
pub type SECBB2_R = crate::FieldReader<u32>;
#[doc = "Field `SECBB2` writer - Secure/non-secure flash Bank 2 sector attribute"]
pub type SECBB2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure/non-secure flash Bank 2 sector attribute"]
    #[inline(always)]
    pub fn secbb2(&self) -> SECBB2_R {
        SECBB2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure/non-secure flash Bank 2 sector attribute"]
    #[inline(always)]
    #[must_use]
    pub fn secbb2(&mut self) -> SECBB2_W<SECBB2R1rs> {
        SECBB2_W::new(self, 0)
    }
}
#[doc = "FLASH secure block-based register for Bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secbb2r1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secbb2r1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECBB2R1rs;
impl crate::RegisterSpec for SECBB2R1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secbb2r1::R`](R) reader structure"]
impl crate::Readable for SECBB2R1rs {}
#[doc = "`write(|w| ..)` method takes [`secbb2r1::W`](W) writer structure"]
impl crate::Writable for SECBB2R1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECBB2R1 to value 0"]
impl crate::Resettable for SECBB2R1rs {
    const RESET_VALUE: u32 = 0;
}
