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
///Field `DMAINEN` reader - Enable DMA management of data input phase
pub type DMAINEN_R = crate::BitReader;
///Field `DMAINEN` writer - Enable DMA management of data input phase
pub type DMAINEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAOUTEN` reader - Enable DMA management of data output phase
pub type DMAOUTEN_R = crate::BitReader;
///Field `DMAOUTEN` writer - Enable DMA management of data output phase
pub type DMAOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GCMPH` reader - GCMPH
pub type GCMPH_R = crate::FieldReader;
///Field `GCMPH` writer - GCMPH
pub type GCMPH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CHMOD_2` reader - CHMOD_2
pub type CHMOD_2_R = crate::BitReader;
///Field `CHMOD_2` writer - CHMOD_2
pub type CHMOD_2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEYSIZE` reader - KEYSIZE
pub type KEYSIZE_R = crate::BitReader;
///Field `KEYSIZE` writer - KEYSIZE
pub type KEYSIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPBLB` reader - NPBLB
pub type NPBLB_R = crate::FieldReader;
///Field `NPBLB` writer - NPBLB
pub type NPBLB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `KMOD` reader - KMOD
pub type KMOD_R = crate::FieldReader;
///Field `KMOD` writer - KMOD
pub type KMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IPRST` reader - IPRST
pub type IPRST_R = crate::BitReader;
///Field `IPRST` writer - IPRST
pub type IPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bits 13:14 - GCMPH
    #[inline(always)]
    pub fn gcmph(&self) -> GCMPH_R {
        GCMPH_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 16 - CHMOD_2
    #[inline(always)]
    pub fn chmod_2(&self) -> CHMOD_2_R {
        CHMOD_2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - KEYSIZE
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 20:23 - NPBLB
    #[inline(always)]
    pub fn npblb(&self) -> NPBLB_R {
        NPBLB_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:25 - KMOD
    #[inline(always)]
    pub fn kmod(&self) -> KMOD_R {
        KMOD_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 31 - IPRST
    #[inline(always)]
    pub fn iprst(&self) -> IPRST_R {
        IPRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("iprst", &self.iprst())
            .field("kmod", &self.kmod())
            .field("npblb", &self.npblb())
            .field("keysize", &self.keysize())
            .field("chmod_2", &self.chmod_2())
            .field("gcmph", &self.gcmph())
            .field("dmaouten", &self.dmaouten())
            .field("dmainen", &self.dmainen())
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
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    ///Bits 1:2 - Data type selection (for data in and data out to/from the cryptographic block)
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W<CRrs> {
        DATATYPE_W::new(self, 1)
    }
    ///Bits 3:4 - AES operating mode
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<CRrs> {
        MODE_W::new(self, 3)
    }
    ///Bits 5:6 - AES chaining mode
    #[inline(always)]
    pub fn chmod(&mut self) -> CHMOD_W<CRrs> {
        CHMOD_W::new(self, 5)
    }
    ///Bit 11 - Enable DMA management of data input phase
    #[inline(always)]
    pub fn dmainen(&mut self) -> DMAINEN_W<CRrs> {
        DMAINEN_W::new(self, 11)
    }
    ///Bit 12 - Enable DMA management of data output phase
    #[inline(always)]
    pub fn dmaouten(&mut self) -> DMAOUTEN_W<CRrs> {
        DMAOUTEN_W::new(self, 12)
    }
    ///Bits 13:14 - GCMPH
    #[inline(always)]
    pub fn gcmph(&mut self) -> GCMPH_W<CRrs> {
        GCMPH_W::new(self, 13)
    }
    ///Bit 16 - CHMOD_2
    #[inline(always)]
    pub fn chmod_2(&mut self) -> CHMOD_2_W<CRrs> {
        CHMOD_2_W::new(self, 16)
    }
    ///Bit 18 - KEYSIZE
    #[inline(always)]
    pub fn keysize(&mut self) -> KEYSIZE_W<CRrs> {
        KEYSIZE_W::new(self, 18)
    }
    ///Bits 20:23 - NPBLB
    #[inline(always)]
    pub fn npblb(&mut self) -> NPBLB_W<CRrs> {
        NPBLB_W::new(self, 20)
    }
    ///Bits 24:25 - KMOD
    #[inline(always)]
    pub fn kmod(&mut self) -> KMOD_W<CRrs> {
        KMOD_W::new(self, 24)
    }
    ///Bit 31 - IPRST
    #[inline(always)]
    pub fn iprst(&mut self) -> IPRST_W<CRrs> {
        IPRST_W::new(self, 31)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#AES:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
