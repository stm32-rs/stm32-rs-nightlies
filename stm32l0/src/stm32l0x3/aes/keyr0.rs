#[doc = "Register `KEYR0` reader"]
pub type R = crate::R<KEYR0rs>;
#[doc = "Register `KEYR0` writer"]
pub type W = crate::W<KEYR0rs>;
#[doc = "Field `KEY0` reader - Data Output Register (LSB key \\[31:0\\])"]
pub type KEY0_R = crate::FieldReader<u32>;
#[doc = "Field `KEY0` writer - Data Output Register (LSB key \\[31:0\\])"]
pub type KEY0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Output Register (LSB key \\[31:0\\])"]
    #[inline(always)]
    pub fn key0(&self) -> KEY0_R {
        KEY0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Output Register (LSB key \\[31:0\\])"]
    #[inline(always)]
    #[must_use]
    pub fn key0(&mut self) -> KEY0_W<KEYR0rs> {
        KEY0_W::new(self, 0)
    }
}
#[doc = "key register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYR0rs;
impl crate::RegisterSpec for KEYR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyr0::R`](R) reader structure"]
impl crate::Readable for KEYR0rs {}
#[doc = "`write(|w| ..)` method takes [`keyr0::W`](W) writer structure"]
impl crate::Writable for KEYR0rs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYR0 to value 0"]
impl crate::Resettable for KEYR0rs {
    const RESET_VALUE: u32 = 0;
}
