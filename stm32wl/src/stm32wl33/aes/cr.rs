///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `EN` reader - EN: AES IP enable
pub type EN_R = crate::BitReader;
///Field `EN` writer - EN: AES IP enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATATYPE` reader - DATATYPE\[1:0\]: Data type selection
pub type DATATYPE_R = crate::FieldReader;
///Field `DATATYPE` writer - DATATYPE\[1:0\]: Data type selection
pub type DATATYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODE` reader - MODE\[1:0\]: AES operating mode
pub type MODE_R = crate::FieldReader;
///Field `MODE` writer - MODE\[1:0\]: AES operating mode
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CHMOD_1_0` reader - CHMOD\[1:0\]: AES Chaining Mode selection
pub type CHMOD_1_0_R = crate::FieldReader;
///Field `CHMOD_1_0` writer - CHMOD\[1:0\]: AES Chaining Mode selection
pub type CHMOD_1_0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CCFC` reader - CCFC: Computation Complete Flag Clear
pub type CCFC_R = crate::BitReader;
///Field `CCFC` writer - CCFC: Computation Complete Flag Clear
pub type CCFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRC` reader - ERRC: Error clear
pub type ERRC_R = crate::BitReader;
///Field `ERRC` writer - ERRC: Error clear
pub type ERRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCFIE` reader - CCFIE: CCF Flag Interrupt Enable
pub type CCFIE_R = crate::BitReader;
///Field `CCFIE` writer - CCFIE: CCF Flag Interrupt Enable
pub type CCFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRIE` reader - ERRIE: Error Interrupt Enable
pub type ERRIE_R = crate::BitReader;
///Field `ERRIE` writer - ERRIE: Error Interrupt Enable
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAINEN` reader - DMAINEN: DMA Input Enable
pub type DMAINEN_R = crate::BitReader;
///Field `DMAINEN` writer - DMAINEN: DMA Input Enable
pub type DMAINEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAOUTEN` reader - DMAOUTEN: DMA Output Enable
pub type DMAOUTEN_R = crate::BitReader;
///Field `DMAOUTEN` writer - DMAOUTEN: DMA Output Enable
pub type DMAOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GCMPH` reader - GCMPH\[1:0\]: GCM or CCM Phase selection
pub type GCMPH_R = crate::FieldReader;
///Field `GCMPH` writer - GCMPH\[1:0\]: GCM or CCM Phase selection
pub type GCMPH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CHMOD_2` reader - CHMOD\[2\]: Chaining mode selection, bit \[2\]
pub type CHMOD_2_R = crate::BitReader;
///Field `CHMOD_2` writer - CHMOD\[2\]: Chaining mode selection, bit \[2\]
pub type CHMOD_2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEYSIZE` reader - KEYSIZE: Key Size selection.
pub type KEYSIZE_R = crate::BitReader;
///Field `KEYSIZE` writer - KEYSIZE: Key Size selection.
pub type KEYSIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPBLB` reader - NPBLB: Number of Padding Bytes in Last Block of payload.
pub type NPBLB_R = crate::FieldReader;
///Field `NPBLB` writer - NPBLB: Number of Padding Bytes in Last Block of payload.
pub type NPBLB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - EN: AES IP enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - DATATYPE\[1:0\]: Data type selection
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - MODE\[1:0\]: AES operating mode
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6 - CHMOD\[1:0\]: AES Chaining Mode selection
    #[inline(always)]
    pub fn chmod_1_0(&self) -> CHMOD_1_0_R {
        CHMOD_1_0_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - CCFC: Computation Complete Flag Clear
    #[inline(always)]
    pub fn ccfc(&self) -> CCFC_R {
        CCFC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ERRC: Error clear
    #[inline(always)]
    pub fn errc(&self) -> ERRC_R {
        ERRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CCFIE: CCF Flag Interrupt Enable
    #[inline(always)]
    pub fn ccfie(&self) -> CCFIE_R {
        CCFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ERRIE: Error Interrupt Enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DMAINEN: DMA Input Enable
    #[inline(always)]
    pub fn dmainen(&self) -> DMAINEN_R {
        DMAINEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - DMAOUTEN: DMA Output Enable
    #[inline(always)]
    pub fn dmaouten(&self) -> DMAOUTEN_R {
        DMAOUTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - GCMPH\[1:0\]: GCM or CCM Phase selection
    #[inline(always)]
    pub fn gcmph(&self) -> GCMPH_R {
        GCMPH_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 16 - CHMOD\[2\]: Chaining mode selection, bit \[2\]
    #[inline(always)]
    pub fn chmod_2(&self) -> CHMOD_2_R {
        CHMOD_2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - KEYSIZE: Key Size selection.
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 20:23 - NPBLB: Number of Padding Bytes in Last Block of payload.
    #[inline(always)]
    pub fn npblb(&self) -> NPBLB_R {
        NPBLB_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("en", &self.en())
            .field("datatype", &self.datatype())
            .field("mode", &self.mode())
            .field("chmod_1_0", &self.chmod_1_0())
            .field("ccfc", &self.ccfc())
            .field("errc", &self.errc())
            .field("ccfie", &self.ccfie())
            .field("errie", &self.errie())
            .field("dmainen", &self.dmainen())
            .field("dmaouten", &self.dmaouten())
            .field("gcmph", &self.gcmph())
            .field("chmod_2", &self.chmod_2())
            .field("keysize", &self.keysize())
            .field("npblb", &self.npblb())
            .finish()
    }
}
impl W {
    ///Bit 0 - EN: AES IP enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CRrs> {
        EN_W::new(self, 0)
    }
    ///Bits 1:2 - DATATYPE\[1:0\]: Data type selection
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W<'_, CRrs> {
        DATATYPE_W::new(self, 1)
    }
    ///Bits 3:4 - MODE\[1:0\]: AES operating mode
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<'_, CRrs> {
        MODE_W::new(self, 3)
    }
    ///Bits 5:6 - CHMOD\[1:0\]: AES Chaining Mode selection
    #[inline(always)]
    pub fn chmod_1_0(&mut self) -> CHMOD_1_0_W<'_, CRrs> {
        CHMOD_1_0_W::new(self, 5)
    }
    ///Bit 7 - CCFC: Computation Complete Flag Clear
    #[inline(always)]
    pub fn ccfc(&mut self) -> CCFC_W<'_, CRrs> {
        CCFC_W::new(self, 7)
    }
    ///Bit 8 - ERRC: Error clear
    #[inline(always)]
    pub fn errc(&mut self) -> ERRC_W<'_, CRrs> {
        ERRC_W::new(self, 8)
    }
    ///Bit 9 - CCFIE: CCF Flag Interrupt Enable
    #[inline(always)]
    pub fn ccfie(&mut self) -> CCFIE_W<'_, CRrs> {
        CCFIE_W::new(self, 9)
    }
    ///Bit 10 - ERRIE: Error Interrupt Enable
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<'_, CRrs> {
        ERRIE_W::new(self, 10)
    }
    ///Bit 11 - DMAINEN: DMA Input Enable
    #[inline(always)]
    pub fn dmainen(&mut self) -> DMAINEN_W<'_, CRrs> {
        DMAINEN_W::new(self, 11)
    }
    ///Bit 12 - DMAOUTEN: DMA Output Enable
    #[inline(always)]
    pub fn dmaouten(&mut self) -> DMAOUTEN_W<'_, CRrs> {
        DMAOUTEN_W::new(self, 12)
    }
    ///Bits 13:14 - GCMPH\[1:0\]: GCM or CCM Phase selection
    #[inline(always)]
    pub fn gcmph(&mut self) -> GCMPH_W<'_, CRrs> {
        GCMPH_W::new(self, 13)
    }
    ///Bit 16 - CHMOD\[2\]: Chaining mode selection, bit \[2\]
    #[inline(always)]
    pub fn chmod_2(&mut self) -> CHMOD_2_W<'_, CRrs> {
        CHMOD_2_W::new(self, 16)
    }
    ///Bit 18 - KEYSIZE: Key Size selection.
    #[inline(always)]
    pub fn keysize(&mut self) -> KEYSIZE_W<'_, CRrs> {
        KEYSIZE_W::new(self, 18)
    }
    ///Bits 20:23 - NPBLB: Number of Padding Bytes in Last Block of payload.
    #[inline(always)]
    pub fn npblb(&mut self) -> NPBLB_W<'_, CRrs> {
        NPBLB_W::new(self, 20)
    }
}
/**AES_CR register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#AES:CR)*/
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
