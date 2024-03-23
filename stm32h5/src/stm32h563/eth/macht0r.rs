#[doc = "Register `MACHT0R` reader"]
pub type R = crate::R<MACHT0Rrs>;
#[doc = "Register `MACHT0R` writer"]
pub type W = crate::W<MACHT0Rrs>;
#[doc = "Field `HT31T0` reader - MAC Hash Table First 32 Bits This field contains the first 32 Bits \\[31:0\\]
of the Hash table."]
pub type HT31T0_R = crate::FieldReader<u32>;
#[doc = "Field `HT31T0` writer - MAC Hash Table First 32 Bits This field contains the first 32 Bits \\[31:0\\]
of the Hash table."]
pub type HT31T0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC Hash Table First 32 Bits This field contains the first 32 Bits \\[31:0\\]
of the Hash table."]
    #[inline(always)]
    pub fn ht31t0(&self) -> HT31T0_R {
        HT31T0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Hash Table First 32 Bits This field contains the first 32 Bits \\[31:0\\]
of the Hash table."]
    #[inline(always)]
    #[must_use]
    pub fn ht31t0(&mut self) -> HT31T0_W<MACHT0Rrs> {
        HT31T0_W::new(self, 0)
    }
}
#[doc = "Hash Table 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macht0r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macht0r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACHT0Rrs;
impl crate::RegisterSpec for MACHT0Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macht0r::R`](R) reader structure"]
impl crate::Readable for MACHT0Rrs {}
#[doc = "`write(|w| ..)` method takes [`macht0r::W`](W) writer structure"]
impl crate::Writable for MACHT0Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACHT0R to value 0"]
impl crate::Resettable for MACHT0Rrs {
    const RESET_VALUE: u32 = 0;
}
