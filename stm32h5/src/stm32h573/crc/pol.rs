#[doc = "Register `POL` reader"]
pub type R = crate::R<POLrs>;
#[doc = "Register `POL` writer"]
pub type W = crate::W<POLrs>;
#[doc = "Field `POL` reader - Programmable polynomial This register is used to write the coefficients of the polynomial to be used for CRC calculation. If the polynomial size is less than 32 bits, the least significant bits have to be used to program the correct value."]
pub type POL_R = crate::FieldReader<u32>;
#[doc = "Field `POL` writer - Programmable polynomial This register is used to write the coefficients of the polynomial to be used for CRC calculation. If the polynomial size is less than 32 bits, the least significant bits have to be used to program the correct value."]
pub type POL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Programmable polynomial This register is used to write the coefficients of the polynomial to be used for CRC calculation. If the polynomial size is less than 32 bits, the least significant bits have to be used to program the correct value."]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Programmable polynomial This register is used to write the coefficients of the polynomial to be used for CRC calculation. If the polynomial size is less than 32 bits, the least significant bits have to be used to program the correct value."]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<POLrs> {
        POL_W::new(self, 0)
    }
}
#[doc = "CRC polynomial\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pol::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pol::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POLrs;
impl crate::RegisterSpec for POLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pol::R`](R) reader structure"]
impl crate::Readable for POLrs {}
#[doc = "`write(|w| ..)` method takes [`pol::W`](W) writer structure"]
impl crate::Writable for POLrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POL to value 0x04c1_1db7"]
impl crate::Resettable for POLrs {
    const RESET_VALUE: u32 = 0x04c1_1db7;
}
