///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `EN` reader - Enable
pub type EN_R = crate::BitReader;
///Field `EN` writer - Enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATATYPE` reader - Data type
pub type DATATYPE_R = crate::FieldReader;
///Field `DATATYPE` writer - Data type
pub type DATATYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODE` reader - Operating mode
pub type MODE_R = crate::FieldReader;
///Field `MODE` writer - Operating mode
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CHMOD` reader - CHMOD\[1:0\]: Chaining mode
pub type CHMOD_R = crate::FieldReader;
///Field `CHMOD` writer - CHMOD\[1:0\]: Chaining mode
pub type CHMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DMAINEN` reader - DMA input enable
pub type DMAINEN_R = crate::BitReader;
///Field `DMAINEN` writer - DMA input enable
pub type DMAINEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAOUTEN` reader - DMA output enable
pub type DMAOUTEN_R = crate::BitReader;
///Field `DMAOUTEN` writer - DMA output enable
pub type DMAOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GCMPH` reader - GCM or CCM phase selection
pub type GCMPH_R = crate::FieldReader;
///Field `GCMPH` writer - GCM or CCM phase selection
pub type GCMPH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CHMOD_1` reader - CHMOD\[2\]
pub type CHMOD_1_R = crate::BitReader;
///Field `CHMOD_1` writer - CHMOD\[2\]
pub type CHMOD_1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEYSIZE` reader - Key size selection
pub type KEYSIZE_R = crate::BitReader;
///Field `KEYSIZE` writer - Key size selection
pub type KEYSIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEYPROT` reader - Key protection
pub type KEYPROT_R = crate::BitReader;
///Field `KEYPROT` writer - Key protection
pub type KEYPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPBLB` reader - Number of padding bytes in last block
pub type NPBLB_R = crate::FieldReader;
///Field `NPBLB` writer - Number of padding bytes in last block
pub type NPBLB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `KMOD` reader - Key mode selection
pub type KMOD_R = crate::FieldReader;
///Field `KMOD` writer - Key mode selection
pub type KMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `KSHAREID` reader - Key share identification
pub type KSHAREID_R = crate::FieldReader;
///Field `KSHAREID` writer - Key share identification
pub type KSHAREID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `KEYSEL` reader - Key selection
pub type KEYSEL_R = crate::FieldReader;
///Field `KEYSEL` writer - Key selection
pub type KEYSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `IPRST` reader - SAES peripheral software reset
pub type IPRST_R = crate::BitReader;
///Field `IPRST` writer - SAES peripheral software reset
pub type IPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Data type
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - Operating mode
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6 - CHMOD\[1:0\]: Chaining mode
    #[inline(always)]
    pub fn chmod(&self) -> CHMOD_R {
        CHMOD_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 11 - DMA input enable
    #[inline(always)]
    pub fn dmainen(&self) -> DMAINEN_R {
        DMAINEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - DMA output enable
    #[inline(always)]
    pub fn dmaouten(&self) -> DMAOUTEN_R {
        DMAOUTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - GCM or CCM phase selection
    #[inline(always)]
    pub fn gcmph(&self) -> GCMPH_R {
        GCMPH_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 16 - CHMOD\[2\]
    #[inline(always)]
    pub fn chmod_1(&self) -> CHMOD_1_R {
        CHMOD_1_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Key size selection
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Key protection
    #[inline(always)]
    pub fn keyprot(&self) -> KEYPROT_R {
        KEYPROT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:23 - Number of padding bytes in last block
    #[inline(always)]
    pub fn npblb(&self) -> NPBLB_R {
        NPBLB_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:25 - Key mode selection
    #[inline(always)]
    pub fn kmod(&self) -> KMOD_R {
        KMOD_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Key share identification
    #[inline(always)]
    pub fn kshareid(&self) -> KSHAREID_R {
        KSHAREID_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:30 - Key selection
    #[inline(always)]
    pub fn keysel(&self) -> KEYSEL_R {
        KEYSEL_R::new(((self.bits >> 28) & 7) as u8)
    }
    ///Bit 31 - SAES peripheral software reset
    #[inline(always)]
    pub fn iprst(&self) -> IPRST_R {
        IPRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("en", &self.en())
            .field("datatype", &self.datatype())
            .field("mode", &self.mode())
            .field("chmod", &self.chmod())
            .field("dmainen", &self.dmainen())
            .field("dmaouten", &self.dmaouten())
            .field("gcmph", &self.gcmph())
            .field("chmod_1", &self.chmod_1())
            .field("keysize", &self.keysize())
            .field("keyprot", &self.keyprot())
            .field("npblb", &self.npblb())
            .field("kmod", &self.kmod())
            .field("kshareid", &self.kshareid())
            .field("keysel", &self.keysel())
            .field("iprst", &self.iprst())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CRrs> {
        EN_W::new(self, 0)
    }
    ///Bits 1:2 - Data type
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W<'_, CRrs> {
        DATATYPE_W::new(self, 1)
    }
    ///Bits 3:4 - Operating mode
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<'_, CRrs> {
        MODE_W::new(self, 3)
    }
    ///Bits 5:6 - CHMOD\[1:0\]: Chaining mode
    #[inline(always)]
    pub fn chmod(&mut self) -> CHMOD_W<'_, CRrs> {
        CHMOD_W::new(self, 5)
    }
    ///Bit 11 - DMA input enable
    #[inline(always)]
    pub fn dmainen(&mut self) -> DMAINEN_W<'_, CRrs> {
        DMAINEN_W::new(self, 11)
    }
    ///Bit 12 - DMA output enable
    #[inline(always)]
    pub fn dmaouten(&mut self) -> DMAOUTEN_W<'_, CRrs> {
        DMAOUTEN_W::new(self, 12)
    }
    ///Bits 13:14 - GCM or CCM phase selection
    #[inline(always)]
    pub fn gcmph(&mut self) -> GCMPH_W<'_, CRrs> {
        GCMPH_W::new(self, 13)
    }
    ///Bit 16 - CHMOD\[2\]
    #[inline(always)]
    pub fn chmod_1(&mut self) -> CHMOD_1_W<'_, CRrs> {
        CHMOD_1_W::new(self, 16)
    }
    ///Bit 18 - Key size selection
    #[inline(always)]
    pub fn keysize(&mut self) -> KEYSIZE_W<'_, CRrs> {
        KEYSIZE_W::new(self, 18)
    }
    ///Bit 19 - Key protection
    #[inline(always)]
    pub fn keyprot(&mut self) -> KEYPROT_W<'_, CRrs> {
        KEYPROT_W::new(self, 19)
    }
    ///Bits 20:23 - Number of padding bytes in last block
    #[inline(always)]
    pub fn npblb(&mut self) -> NPBLB_W<'_, CRrs> {
        NPBLB_W::new(self, 20)
    }
    ///Bits 24:25 - Key mode selection
    #[inline(always)]
    pub fn kmod(&mut self) -> KMOD_W<'_, CRrs> {
        KMOD_W::new(self, 24)
    }
    ///Bits 26:27 - Key share identification
    #[inline(always)]
    pub fn kshareid(&mut self) -> KSHAREID_W<'_, CRrs> {
        KSHAREID_W::new(self, 26)
    }
    ///Bits 28:30 - Key selection
    #[inline(always)]
    pub fn keysel(&mut self) -> KEYSEL_W<'_, CRrs> {
        KEYSEL_W::new(self, 28)
    }
    ///Bit 31 - SAES peripheral software reset
    #[inline(always)]
    pub fn iprst(&mut self) -> IPRST_W<'_, CRrs> {
        IPRST_W::new(self, 31)
    }
}
/**SAES control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:CR)*/
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
