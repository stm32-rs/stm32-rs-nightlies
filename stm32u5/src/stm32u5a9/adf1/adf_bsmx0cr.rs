#[doc = "Register `ADF_BSMX0CR` reader"]
pub type R = crate::R<ADF_BSMX0CRrs>;
#[doc = "Register `ADF_BSMX0CR` writer"]
pub type W = crate::W<ADF_BSMX0CRrs>;
#[doc = "Field `BSSEL` reader - Bitstream selection"]
pub type BSSEL_R = crate::FieldReader;
#[doc = "Field `BSSEL` writer - Bitstream selection"]
pub type BSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BSMXACTIVE` reader - BSMX active flag"]
pub type BSMXACTIVE_R = crate::BitReader;
#[doc = "Field `BSMXACTIVE` writer - BSMX active flag"]
pub type BSMXACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Bitstream selection"]
    #[inline(always)]
    pub fn bssel(&self) -> BSSEL_R {
        BSSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - BSMX active flag"]
    #[inline(always)]
    pub fn bsmxactive(&self) -> BSMXACTIVE_R {
        BSMXACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Bitstream selection"]
    #[inline(always)]
    #[must_use]
    pub fn bssel(&mut self) -> BSSEL_W<ADF_BSMX0CRrs> {
        BSSEL_W::new(self, 0)
    }
    #[doc = "Bit 31 - BSMX active flag"]
    #[inline(always)]
    #[must_use]
    pub fn bsmxactive(&mut self) -> BSMXACTIVE_W<ADF_BSMX0CRrs> {
        BSMXACTIVE_W::new(self, 31)
    }
}
#[doc = "ADF bitstream matrix control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adf_bsmx0cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adf_bsmx0cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADF_BSMX0CRrs;
impl crate::RegisterSpec for ADF_BSMX0CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adf_bsmx0cr::R`](R) reader structure"]
impl crate::Readable for ADF_BSMX0CRrs {}
#[doc = "`write(|w| ..)` method takes [`adf_bsmx0cr::W`](W) writer structure"]
impl crate::Writable for ADF_BSMX0CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADF_BSMX0CR to value 0"]
impl crate::Resettable for ADF_BSMX0CRrs {
    const RESET_VALUE: u32 = 0;
}
