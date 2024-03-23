#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `ALGODIR` reader - Algorithm direction"]
pub type ALGODIR_R = crate::BitReader;
#[doc = "Field `ALGODIR` writer - Algorithm direction"]
pub type ALGODIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALGOMODE` reader - Algorithm mode"]
pub type ALGOMODE_R = crate::FieldReader;
#[doc = "Field `ALGOMODE` writer - Algorithm mode"]
pub type ALGOMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DATATYPE` reader - Data type selection"]
pub type DATATYPE_R = crate::FieldReader;
#[doc = "Field `DATATYPE` writer - Data type selection"]
pub type DATATYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYSIZE` reader - Key size selection (AES mode only)"]
pub type KEYSIZE_R = crate::FieldReader;
#[doc = "Field `KEYSIZE` writer - Key size selection (AES mode only)"]
pub type KEYSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FFLUSH` writer - FIFO flush"]
pub type FFLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPEN` reader - Cryptographic processor enable"]
pub type CRYPEN_R = crate::BitReader;
#[doc = "Field `CRYPEN` writer - Cryptographic processor enable"]
pub type CRYPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Algorithm direction"]
    #[inline(always)]
    pub fn algodir(&self) -> ALGODIR_R {
        ALGODIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Algorithm mode"]
    #[inline(always)]
    pub fn algomode(&self) -> ALGOMODE_R {
        ALGOMODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Data type selection"]
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Key size selection (AES mode only)"]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - Cryptographic processor enable"]
    #[inline(always)]
    pub fn crypen(&self) -> CRYPEN_R {
        CRYPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Algorithm direction"]
    #[inline(always)]
    #[must_use]
    pub fn algodir(&mut self) -> ALGODIR_W<CRrs> {
        ALGODIR_W::new(self, 2)
    }
    #[doc = "Bits 3:5 - Algorithm mode"]
    #[inline(always)]
    #[must_use]
    pub fn algomode(&mut self) -> ALGOMODE_W<CRrs> {
        ALGOMODE_W::new(self, 3)
    }
    #[doc = "Bits 6:7 - Data type selection"]
    #[inline(always)]
    #[must_use]
    pub fn datatype(&mut self) -> DATATYPE_W<CRrs> {
        DATATYPE_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Key size selection (AES mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn keysize(&mut self) -> KEYSIZE_W<CRrs> {
        KEYSIZE_W::new(self, 8)
    }
    #[doc = "Bit 14 - FIFO flush"]
    #[inline(always)]
    #[must_use]
    pub fn fflush(&mut self) -> FFLUSH_W<CRrs> {
        FFLUSH_W::new(self, 14)
    }
    #[doc = "Bit 15 - Cryptographic processor enable"]
    #[inline(always)]
    #[must_use]
    pub fn crypen(&mut self) -> CRYPEN_W<CRrs> {
        CRYPEN_W::new(self, 15)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
