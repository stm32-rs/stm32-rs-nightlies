#[doc = "Register `ADF_DLY0CR` reader"]
pub type R = crate::R<ADF_DLY0CRrs>;
#[doc = "Register `ADF_DLY0CR` writer"]
pub type W = crate::W<ADF_DLY0CRrs>;
#[doc = "Field `SKPDLY` reader - Delay to apply to a bitstream"]
pub type SKPDLY_R = crate::FieldReader;
#[doc = "Field `SKPDLY` writer - Delay to apply to a bitstream"]
pub type SKPDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SKPBF` reader - Skip busy flag"]
pub type SKPBF_R = crate::BitReader;
#[doc = "Field `SKPBF` writer - Skip busy flag"]
pub type SKPBF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Delay to apply to a bitstream"]
    #[inline(always)]
    pub fn skpdly(&self) -> SKPDLY_R {
        SKPDLY_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Skip busy flag"]
    #[inline(always)]
    pub fn skpbf(&self) -> SKPBF_R {
        SKPBF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Delay to apply to a bitstream"]
    #[inline(always)]
    #[must_use]
    pub fn skpdly(&mut self) -> SKPDLY_W<ADF_DLY0CRrs> {
        SKPDLY_W::new(self, 0)
    }
    #[doc = "Bit 31 - Skip busy flag"]
    #[inline(always)]
    #[must_use]
    pub fn skpbf(&mut self) -> SKPBF_W<ADF_DLY0CRrs> {
        SKPBF_W::new(self, 31)
    }
}
#[doc = "ADF delay control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adf_dly0cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adf_dly0cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADF_DLY0CRrs;
impl crate::RegisterSpec for ADF_DLY0CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adf_dly0cr::R`](R) reader structure"]
impl crate::Readable for ADF_DLY0CRrs {}
#[doc = "`write(|w| ..)` method takes [`adf_dly0cr::W`](W) writer structure"]
impl crate::Writable for ADF_DLY0CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADF_DLY0CR to value 0"]
impl crate::Resettable for ADF_DLY0CRrs {
    const RESET_VALUE: u32 = 0;
}
