#[doc = "Register `BRR` reader"]
pub type R = crate::R<BRRrs>;
#[doc = "Register `BRR` writer"]
pub type W = crate::W<BRRrs>;
#[doc = "Field `DIV_Fraction` reader - DIV_Fraction"]
pub type DIV_FRACTION_R = crate::FieldReader;
#[doc = "Field `DIV_Fraction` writer - DIV_Fraction"]
pub type DIV_FRACTION_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `DIV_Mantissa` reader - DIV_Mantissa"]
pub type DIV_MANTISSA_R = crate::FieldReader<u16>;
#[doc = "Field `DIV_Mantissa` writer - DIV_Mantissa"]
pub type DIV_MANTISSA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:3 - DIV_Fraction"]
    #[inline(always)]
    pub fn div_fraction(&self) -> DIV_FRACTION_R {
        DIV_FRACTION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - DIV_Mantissa"]
    #[inline(always)]
    pub fn div_mantissa(&self) -> DIV_MANTISSA_R {
        DIV_MANTISSA_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - DIV_Fraction"]
    #[inline(always)]
    #[must_use]
    pub fn div_fraction(&mut self) -> DIV_FRACTION_W<BRRrs> {
        DIV_FRACTION_W::new(self, 0)
    }
    #[doc = "Bits 4:15 - DIV_Mantissa"]
    #[inline(always)]
    #[must_use]
    pub fn div_mantissa(&mut self) -> DIV_MANTISSA_W<BRRrs> {
        DIV_MANTISSA_W::new(self, 4)
    }
}
#[doc = "UART4 BRR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRRrs;
impl crate::RegisterSpec for BRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brr::R`](R) reader structure"]
impl crate::Readable for BRRrs {}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BRRrs {
    const RESET_VALUE: u32 = 0;
}
