#[doc = "Register `MDMA_C10TCR` reader"]
pub type R = crate::R<MDMA_C10TCRrs>;
#[doc = "Register `MDMA_C10TCR` writer"]
pub type W = crate::W<MDMA_C10TCRrs>;
#[doc = "Field `SINC` reader - SINC"]
pub type SINC_R = crate::FieldReader;
#[doc = "Field `SINC` writer - SINC"]
pub type SINC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DINC` reader - DINC"]
pub type DINC_R = crate::FieldReader;
#[doc = "Field `DINC` writer - DINC"]
pub type DINC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SSIZE` reader - SSIZE"]
pub type SSIZE_R = crate::FieldReader;
#[doc = "Field `SSIZE` writer - SSIZE"]
pub type SSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DSIZE` reader - DSIZE"]
pub type DSIZE_R = crate::FieldReader;
#[doc = "Field `DSIZE` writer - DSIZE"]
pub type DSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SINCOS` reader - SINCOS"]
pub type SINCOS_R = crate::FieldReader;
#[doc = "Field `SINCOS` writer - SINCOS"]
pub type SINCOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DINCOS` reader - DINCOS"]
pub type DINCOS_R = crate::FieldReader;
#[doc = "Field `DINCOS` writer - DINCOS"]
pub type DINCOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SBURST` reader - SBURST"]
pub type SBURST_R = crate::FieldReader;
#[doc = "Field `SBURST` writer - SBURST"]
pub type SBURST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DBURST` reader - DBURST"]
pub type DBURST_R = crate::FieldReader;
#[doc = "Field `DBURST` writer - DBURST"]
pub type DBURST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TLEN` reader - TLEN"]
pub type TLEN_R = crate::FieldReader;
#[doc = "Field `TLEN` writer - TLEN"]
pub type TLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PKE` reader - PKE"]
pub type PKE_R = crate::BitReader;
#[doc = "Field `PKE` writer - PKE"]
pub type PKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAM` reader - PAM"]
pub type PAM_R = crate::FieldReader;
#[doc = "Field `PAM` writer - PAM"]
pub type PAM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRGM` reader - TRGM"]
pub type TRGM_R = crate::FieldReader;
#[doc = "Field `TRGM` writer - TRGM"]
pub type TRGM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWRM` reader - SWRM"]
pub type SWRM_R = crate::BitReader;
#[doc = "Field `SWRM` writer - SWRM"]
pub type SWRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BWM` reader - BWM"]
pub type BWM_R = crate::BitReader;
#[doc = "Field `BWM` writer - BWM"]
pub type BWM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - SINC"]
    #[inline(always)]
    pub fn sinc(&self) -> SINC_R {
        SINC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - DINC"]
    #[inline(always)]
    pub fn dinc(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - SSIZE"]
    #[inline(always)]
    pub fn ssize(&self) -> SSIZE_R {
        SSIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - DSIZE"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SINCOS"]
    #[inline(always)]
    pub fn sincos(&self) -> SINCOS_R {
        SINCOS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - DINCOS"]
    #[inline(always)]
    pub fn dincos(&self) -> DINCOS_R {
        DINCOS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - SBURST"]
    #[inline(always)]
    pub fn sburst(&self) -> SBURST_R {
        SBURST_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - DBURST"]
    #[inline(always)]
    pub fn dburst(&self) -> DBURST_R {
        DBURST_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:24 - TLEN"]
    #[inline(always)]
    pub fn tlen(&self) -> TLEN_R {
        TLEN_R::new(((self.bits >> 18) & 0x7f) as u8)
    }
    #[doc = "Bit 25 - PKE"]
    #[inline(always)]
    pub fn pke(&self) -> PKE_R {
        PKE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - PAM"]
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - TRGM"]
    #[inline(always)]
    pub fn trgm(&self) -> TRGM_R {
        TRGM_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - SWRM"]
    #[inline(always)]
    pub fn swrm(&self) -> SWRM_R {
        SWRM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - BWM"]
    #[inline(always)]
    pub fn bwm(&self) -> BWM_R {
        BWM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SINC"]
    #[inline(always)]
    #[must_use]
    pub fn sinc(&mut self) -> SINC_W<MDMA_C10TCRrs> {
        SINC_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - DINC"]
    #[inline(always)]
    #[must_use]
    pub fn dinc(&mut self) -> DINC_W<MDMA_C10TCRrs> {
        DINC_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - SSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn ssize(&mut self) -> SSIZE_W<MDMA_C10TCRrs> {
        SSIZE_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - DSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn dsize(&mut self) -> DSIZE_W<MDMA_C10TCRrs> {
        DSIZE_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - SINCOS"]
    #[inline(always)]
    #[must_use]
    pub fn sincos(&mut self) -> SINCOS_W<MDMA_C10TCRrs> {
        SINCOS_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - DINCOS"]
    #[inline(always)]
    #[must_use]
    pub fn dincos(&mut self) -> DINCOS_W<MDMA_C10TCRrs> {
        DINCOS_W::new(self, 10)
    }
    #[doc = "Bits 12:14 - SBURST"]
    #[inline(always)]
    #[must_use]
    pub fn sburst(&mut self) -> SBURST_W<MDMA_C10TCRrs> {
        SBURST_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - DBURST"]
    #[inline(always)]
    #[must_use]
    pub fn dburst(&mut self) -> DBURST_W<MDMA_C10TCRrs> {
        DBURST_W::new(self, 15)
    }
    #[doc = "Bits 18:24 - TLEN"]
    #[inline(always)]
    #[must_use]
    pub fn tlen(&mut self) -> TLEN_W<MDMA_C10TCRrs> {
        TLEN_W::new(self, 18)
    }
    #[doc = "Bit 25 - PKE"]
    #[inline(always)]
    #[must_use]
    pub fn pke(&mut self) -> PKE_W<MDMA_C10TCRrs> {
        PKE_W::new(self, 25)
    }
    #[doc = "Bits 26:27 - PAM"]
    #[inline(always)]
    #[must_use]
    pub fn pam(&mut self) -> PAM_W<MDMA_C10TCRrs> {
        PAM_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - TRGM"]
    #[inline(always)]
    #[must_use]
    pub fn trgm(&mut self) -> TRGM_W<MDMA_C10TCRrs> {
        TRGM_W::new(self, 28)
    }
    #[doc = "Bit 30 - SWRM"]
    #[inline(always)]
    #[must_use]
    pub fn swrm(&mut self) -> SWRM_W<MDMA_C10TCRrs> {
        SWRM_W::new(self, 30)
    }
    #[doc = "Bit 31 - BWM"]
    #[inline(always)]
    #[must_use]
    pub fn bwm(&mut self) -> BWM_W<MDMA_C10TCRrs> {
        BWM_W::new(self, 31)
    }
}
#[doc = "This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x00).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c10tcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c10tcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C10TCRrs;
impl crate::RegisterSpec for MDMA_C10TCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c10tcr::R`](R) reader structure"]
impl crate::Readable for MDMA_C10TCRrs {}
#[doc = "`write(|w| ..)` method takes [`mdma_c10tcr::W`](W) writer structure"]
impl crate::Writable for MDMA_C10TCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMA_C10TCR to value 0"]
impl crate::Resettable for MDMA_C10TCRrs {
    const RESET_VALUE: u32 = 0;
}
