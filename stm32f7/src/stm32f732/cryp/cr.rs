///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `EN` reader - AES enable
pub type EN_R = crate::BitReader;
///Field `EN` writer - AES enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATATYPE` reader - Data type selection (for data in and data out to/from the cryptographic block)
pub type DATATYPE_R = crate::FieldReader;
///Field `DATATYPE` writer - Data type selection (for data in and data out to/from the cryptographic block)
pub type DATATYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODE` reader - AES operating mode
pub type MODE_R = crate::FieldReader;
///Field `MODE` writer - AES operating mode
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CHMOD` reader - AES chaining mode
pub type CHMOD_R = crate::FieldReader;
///Field `CHMOD` writer - AES chaining mode
pub type CHMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CCFC` reader - Computation complete flag clear
pub type CCFC_R = crate::BitReader;
///Field `CCFC` writer - Computation complete flag clear
pub type CCFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRC` reader - Error clear
pub type ERRC_R = crate::BitReader;
///Field `ERRC` writer - Error clear
pub type ERRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCFIE` reader - CCF flag interrupt enable
pub type CCFIE_R = crate::BitReader;
///Field `CCFIE` writer - CCF flag interrupt enable
pub type CCFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRIE` reader - Error interrupt enable
pub type ERRIE_R = crate::BitReader;
///Field `ERRIE` writer - Error interrupt enable
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAINEN` reader - Enable DMA management of data input phase
pub type DMAINEN_R = crate::BitReader;
///Field `DMAINEN` writer - Enable DMA management of data input phase
pub type DMAINEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAOUTEN` reader - Enable DMA management of data output phase
pub type DMAOUTEN_R = crate::BitReader;
///Field `DMAOUTEN` writer - Enable DMA management of data output phase
pub type DMAOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GCMPH` reader - Used only for GCM, GMAC and CMAC algorithms and has no effect when other
pub type GCMPH_R = crate::FieldReader;
///Field `GCMPH` writer - Used only for GCM, GMAC and CMAC algorithms and has no effect when other
pub type GCMPH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `KEYSIZE` reader - Key size selection
pub type KEYSIZE_R = crate::BitReader;
///Field `KEYSIZE` writer - Key size selection
pub type KEYSIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - AES enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Data type selection (for data in and data out to/from the cryptographic block)
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - AES operating mode
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6 - AES chaining mode
    #[inline(always)]
    pub fn chmod(&self) -> CHMOD_R {
        CHMOD_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - Computation complete flag clear
    #[inline(always)]
    pub fn ccfc(&self) -> CCFC_R {
        CCFC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Error clear
    #[inline(always)]
    pub fn errc(&self) -> ERRC_R {
        ERRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CCF flag interrupt enable
    #[inline(always)]
    pub fn ccfie(&self) -> CCFIE_R {
        CCFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Enable DMA management of data input phase
    #[inline(always)]
    pub fn dmainen(&self) -> DMAINEN_R {
        DMAINEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Enable DMA management of data output phase
    #[inline(always)]
    pub fn dmaouten(&self) -> DMAOUTEN_R {
        DMAOUTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - Used only for GCM, GMAC and CMAC algorithms and has no effect when other
    #[inline(always)]
    pub fn gcmph(&self) -> GCMPH_R {
        GCMPH_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 18 - Key size selection
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("keysize", &self.keysize())
            .field("gcmph", &self.gcmph())
            .field("dmaouten", &self.dmaouten())
            .field("dmainen", &self.dmainen())
            .field("errie", &self.errie())
            .field("ccfie", &self.ccfie())
            .field("errc", &self.errc())
            .field("ccfc", &self.ccfc())
            .field("chmod", &self.chmod())
            .field("mode", &self.mode())
            .field("datatype", &self.datatype())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - AES enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CRrs> {
        EN_W::new(self, 0)
    }
    ///Bits 1:2 - Data type selection (for data in and data out to/from the cryptographic block)
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W<'_, CRrs> {
        DATATYPE_W::new(self, 1)
    }
    ///Bits 3:4 - AES operating mode
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<'_, CRrs> {
        MODE_W::new(self, 3)
    }
    ///Bits 5:6 - AES chaining mode
    #[inline(always)]
    pub fn chmod(&mut self) -> CHMOD_W<'_, CRrs> {
        CHMOD_W::new(self, 5)
    }
    ///Bit 7 - Computation complete flag clear
    #[inline(always)]
    pub fn ccfc(&mut self) -> CCFC_W<'_, CRrs> {
        CCFC_W::new(self, 7)
    }
    ///Bit 8 - Error clear
    #[inline(always)]
    pub fn errc(&mut self) -> ERRC_W<'_, CRrs> {
        ERRC_W::new(self, 8)
    }
    ///Bit 9 - CCF flag interrupt enable
    #[inline(always)]
    pub fn ccfie(&mut self) -> CCFIE_W<'_, CRrs> {
        CCFIE_W::new(self, 9)
    }
    ///Bit 10 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<'_, CRrs> {
        ERRIE_W::new(self, 10)
    }
    ///Bit 11 - Enable DMA management of data input phase
    #[inline(always)]
    pub fn dmainen(&mut self) -> DMAINEN_W<'_, CRrs> {
        DMAINEN_W::new(self, 11)
    }
    ///Bit 12 - Enable DMA management of data output phase
    #[inline(always)]
    pub fn dmaouten(&mut self) -> DMAOUTEN_W<'_, CRrs> {
        DMAOUTEN_W::new(self, 12)
    }
    ///Bits 13:14 - Used only for GCM, GMAC and CMAC algorithms and has no effect when other
    #[inline(always)]
    pub fn gcmph(&mut self) -> GCMPH_W<'_, CRrs> {
        GCMPH_W::new(self, 13)
    }
    ///Bit 18 - Key size selection
    #[inline(always)]
    pub fn keysize(&mut self) -> KEYSIZE_W<'_, CRrs> {
        KEYSIZE_W::new(self, 18)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F732.html#CRYP:CR)*/
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
