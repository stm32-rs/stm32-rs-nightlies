///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `ALGODIR` reader - Algorithm direction
pub type ALGODIR_R = crate::BitReader;
///Field `ALGODIR` writer - Algorithm direction
pub type ALGODIR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALGOMODE` reader - Algorithm mode
pub type ALGOMODE_R = crate::FieldReader;
///Field `ALGOMODE` writer - Algorithm mode
pub type ALGOMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DATATYPE` reader - Data type selection
pub type DATATYPE_R = crate::FieldReader;
///Field `DATATYPE` writer - Data type selection
pub type DATATYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `KEYSIZE` reader - Key size selection (AES mode only)
pub type KEYSIZE_R = crate::FieldReader;
///Field `KEYSIZE` writer - Key size selection (AES mode only)
pub type KEYSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FFLUSH` writer - FIFO flush
pub type FFLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYPEN` reader - Cryptographic processor enable
pub type CRYPEN_R = crate::BitReader;
///Field `CRYPEN` writer - Cryptographic processor enable
pub type CRYPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - Algorithm direction
    #[inline(always)]
    pub fn algodir(&self) -> ALGODIR_R {
        ALGODIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:5 - Algorithm mode
    #[inline(always)]
    pub fn algomode(&self) -> ALGOMODE_R {
        ALGOMODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:7 - Data type selection
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Key size selection (AES mode only)
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 15 - Cryptographic processor enable
    #[inline(always)]
    pub fn crypen(&self) -> CRYPEN_R {
        CRYPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("crypen", &self.crypen())
            .field("keysize", &self.keysize())
            .field("datatype", &self.datatype())
            .field("algomode", &self.algomode())
            .field("algodir", &self.algodir())
            .finish()
    }
}
impl W {
    ///Bit 2 - Algorithm direction
    #[inline(always)]
    pub fn algodir(&mut self) -> ALGODIR_W<'_, CRrs> {
        ALGODIR_W::new(self, 2)
    }
    ///Bits 3:5 - Algorithm mode
    #[inline(always)]
    pub fn algomode(&mut self) -> ALGOMODE_W<'_, CRrs> {
        ALGOMODE_W::new(self, 3)
    }
    ///Bits 6:7 - Data type selection
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W<'_, CRrs> {
        DATATYPE_W::new(self, 6)
    }
    ///Bits 8:9 - Key size selection (AES mode only)
    #[inline(always)]
    pub fn keysize(&mut self) -> KEYSIZE_W<'_, CRrs> {
        KEYSIZE_W::new(self, 8)
    }
    ///Bit 14 - FIFO flush
    #[inline(always)]
    pub fn fflush(&mut self) -> FFLUSH_W<'_, CRrs> {
        FFLUSH_W::new(self, 14)
    }
    ///Bit 15 - Cryptographic processor enable
    #[inline(always)]
    pub fn crypen(&mut self) -> CRYPEN_W<'_, CRrs> {
        CRYPEN_W::new(self, 15)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F215.html#CRYP:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
