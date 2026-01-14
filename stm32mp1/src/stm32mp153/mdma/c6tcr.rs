///Register `C6TCR` reader
pub type R = crate::R<C6TCRrs>;
///Register `C6TCR` writer
pub type W = crate::W<C6TCRrs>;
///Field `SINC` reader - SINC
pub type SINC_R = crate::FieldReader;
///Field `SINC` writer - SINC
pub type SINC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DINC` reader - DINC
pub type DINC_R = crate::FieldReader;
///Field `DINC` writer - DINC
pub type DINC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SSIZE` reader - SSIZE
pub type SSIZE_R = crate::FieldReader;
///Field `SSIZE` writer - SSIZE
pub type SSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DSIZE` reader - DSIZE
pub type DSIZE_R = crate::FieldReader;
///Field `DSIZE` writer - DSIZE
pub type DSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SINCOS` reader - SINCOS
pub type SINCOS_R = crate::FieldReader;
///Field `SINCOS` writer - SINCOS
pub type SINCOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DINCOS` reader - DINCOS
pub type DINCOS_R = crate::FieldReader;
///Field `DINCOS` writer - DINCOS
pub type DINCOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SBURST` reader - SBURST
pub type SBURST_R = crate::FieldReader;
///Field `SBURST` writer - SBURST
pub type SBURST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DBURST` reader - DBURST
pub type DBURST_R = crate::FieldReader;
///Field `DBURST` writer - DBURST
pub type DBURST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TLEN` reader - TLEN
pub type TLEN_R = crate::FieldReader;
///Field `TLEN` writer - TLEN
pub type TLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `PKE` reader - PKE
pub type PKE_R = crate::BitReader;
///Field `PKE` writer - PKE
pub type PKE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PAM` reader - PAM
pub type PAM_R = crate::FieldReader;
///Field `PAM` writer - PAM
pub type PAM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TRGM` reader - TRGM
pub type TRGM_R = crate::FieldReader;
///Field `TRGM` writer - TRGM
pub type TRGM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SWRM` reader - SWRM
pub type SWRM_R = crate::BitReader;
///Field `SWRM` writer - SWRM
pub type SWRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BWM` reader - BWM
pub type BWM_R = crate::BitReader;
///Field `BWM` writer - BWM
pub type BWM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - SINC
    #[inline(always)]
    pub fn sinc(&self) -> SINC_R {
        SINC_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - DINC
    #[inline(always)]
    pub fn dinc(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - SSIZE
    #[inline(always)]
    pub fn ssize(&self) -> SSIZE_R {
        SSIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - DSIZE
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - SINCOS
    #[inline(always)]
    pub fn sincos(&self) -> SINCOS_R {
        SINCOS_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - DINCOS
    #[inline(always)]
    pub fn dincos(&self) -> DINCOS_R {
        DINCOS_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:14 - SBURST
    #[inline(always)]
    pub fn sburst(&self) -> SBURST_R {
        SBURST_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - DBURST
    #[inline(always)]
    pub fn dburst(&self) -> DBURST_R {
        DBURST_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:24 - TLEN
    #[inline(always)]
    pub fn tlen(&self) -> TLEN_R {
        TLEN_R::new(((self.bits >> 18) & 0x7f) as u8)
    }
    ///Bit 25 - PKE
    #[inline(always)]
    pub fn pke(&self) -> PKE_R {
        PKE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 26:27 - PAM
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - TRGM
    #[inline(always)]
    pub fn trgm(&self) -> TRGM_R {
        TRGM_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 30 - SWRM
    #[inline(always)]
    pub fn swrm(&self) -> SWRM_R {
        SWRM_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - BWM
    #[inline(always)]
    pub fn bwm(&self) -> BWM_R {
        BWM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C6TCR")
            .field("sinc", &self.sinc())
            .field("dinc", &self.dinc())
            .field("ssize", &self.ssize())
            .field("dsize", &self.dsize())
            .field("sincos", &self.sincos())
            .field("dincos", &self.dincos())
            .field("sburst", &self.sburst())
            .field("dburst", &self.dburst())
            .field("tlen", &self.tlen())
            .field("pke", &self.pke())
            .field("pam", &self.pam())
            .field("trgm", &self.trgm())
            .field("swrm", &self.swrm())
            .field("bwm", &self.bwm())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - SINC
    #[inline(always)]
    pub fn sinc(&mut self) -> SINC_W<'_, C6TCRrs> {
        SINC_W::new(self, 0)
    }
    ///Bits 2:3 - DINC
    #[inline(always)]
    pub fn dinc(&mut self) -> DINC_W<'_, C6TCRrs> {
        DINC_W::new(self, 2)
    }
    ///Bits 4:5 - SSIZE
    #[inline(always)]
    pub fn ssize(&mut self) -> SSIZE_W<'_, C6TCRrs> {
        SSIZE_W::new(self, 4)
    }
    ///Bits 6:7 - DSIZE
    #[inline(always)]
    pub fn dsize(&mut self) -> DSIZE_W<'_, C6TCRrs> {
        DSIZE_W::new(self, 6)
    }
    ///Bits 8:9 - SINCOS
    #[inline(always)]
    pub fn sincos(&mut self) -> SINCOS_W<'_, C6TCRrs> {
        SINCOS_W::new(self, 8)
    }
    ///Bits 10:11 - DINCOS
    #[inline(always)]
    pub fn dincos(&mut self) -> DINCOS_W<'_, C6TCRrs> {
        DINCOS_W::new(self, 10)
    }
    ///Bits 12:14 - SBURST
    #[inline(always)]
    pub fn sburst(&mut self) -> SBURST_W<'_, C6TCRrs> {
        SBURST_W::new(self, 12)
    }
    ///Bits 15:17 - DBURST
    #[inline(always)]
    pub fn dburst(&mut self) -> DBURST_W<'_, C6TCRrs> {
        DBURST_W::new(self, 15)
    }
    ///Bits 18:24 - TLEN
    #[inline(always)]
    pub fn tlen(&mut self) -> TLEN_W<'_, C6TCRrs> {
        TLEN_W::new(self, 18)
    }
    ///Bit 25 - PKE
    #[inline(always)]
    pub fn pke(&mut self) -> PKE_W<'_, C6TCRrs> {
        PKE_W::new(self, 25)
    }
    ///Bits 26:27 - PAM
    #[inline(always)]
    pub fn pam(&mut self) -> PAM_W<'_, C6TCRrs> {
        PAM_W::new(self, 26)
    }
    ///Bits 28:29 - TRGM
    #[inline(always)]
    pub fn trgm(&mut self) -> TRGM_W<'_, C6TCRrs> {
        TRGM_W::new(self, 28)
    }
    ///Bit 30 - SWRM
    #[inline(always)]
    pub fn swrm(&mut self) -> SWRM_W<'_, C6TCRrs> {
        SWRM_W::new(self, 30)
    }
    ///Bit 31 - BWM
    #[inline(always)]
    pub fn bwm(&mut self) -> BWM_W<'_, C6TCRrs> {
        BWM_W::new(self, 31)
    }
}
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c6tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:C6TCR)*/
pub struct C6TCRrs;
impl crate::RegisterSpec for C6TCRrs {
    type Ux = u32;
}
///`read()` method returns [`c6tcr::R`](R) reader structure
impl crate::Readable for C6TCRrs {}
///`write(|w| ..)` method takes [`c6tcr::W`](W) writer structure
impl crate::Writable for C6TCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C6TCR to value 0
impl crate::Resettable for C6TCRrs {}
