///Register `AHB3RSTSETR` reader
pub type R = crate::R<AHB3RSTSETRrs>;
///Register `AHB3RSTSETR` writer
pub type W = crate::W<AHB3RSTSETRrs>;
///Field `DCMIRST` reader - DCMIRST
pub type DCMIRST_R = crate::BitReader;
///Field `DCMIRST` writer - DCMIRST
pub type DCMIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYP2RST` reader - CRYP2RST
pub type CRYP2RST_R = crate::BitReader;
///Field `CRYP2RST` writer - CRYP2RST
pub type CRYP2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASH2RST` reader - HASH2RST
pub type HASH2RST_R = crate::BitReader;
///Field `HASH2RST` writer - HASH2RST
pub type HASH2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNG2RST` reader - RNG2RST
pub type RNG2RST_R = crate::BitReader;
///Field `RNG2RST` writer - RNG2RST
pub type RNG2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRC2RST` reader - CRC2RST
pub type CRC2RST_R = crate::BitReader;
///Field `CRC2RST` writer - CRC2RST
pub type CRC2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEMRST` reader - HSEMRST
pub type HSEMRST_R = crate::BitReader;
///Field `HSEMRST` writer - HSEMRST
pub type HSEMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IPCCRST` reader - IPCCRST
pub type IPCCRST_R = crate::BitReader;
///Field `IPCCRST` writer - IPCCRST
pub type IPCCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DCMIRST
    #[inline(always)]
    pub fn dcmirst(&self) -> DCMIRST_R {
        DCMIRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - CRYP2RST
    #[inline(always)]
    pub fn cryp2rst(&self) -> CRYP2RST_R {
        CRYP2RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HASH2RST
    #[inline(always)]
    pub fn hash2rst(&self) -> HASH2RST_R {
        HASH2RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RNG2RST
    #[inline(always)]
    pub fn rng2rst(&self) -> RNG2RST_R {
        RNG2RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CRC2RST
    #[inline(always)]
    pub fn crc2rst(&self) -> CRC2RST_R {
        CRC2RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - HSEMRST
    #[inline(always)]
    pub fn hsemrst(&self) -> HSEMRST_R {
        HSEMRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - IPCCRST
    #[inline(always)]
    pub fn ipccrst(&self) -> IPCCRST_R {
        IPCCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3RSTSETR")
            .field("dcmirst", &self.dcmirst())
            .field("cryp2rst", &self.cryp2rst())
            .field("hash2rst", &self.hash2rst())
            .field("rng2rst", &self.rng2rst())
            .field("crc2rst", &self.crc2rst())
            .field("hsemrst", &self.hsemrst())
            .field("ipccrst", &self.ipccrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - DCMIRST
    #[inline(always)]
    pub fn dcmirst(&mut self) -> DCMIRST_W<'_, AHB3RSTSETRrs> {
        DCMIRST_W::new(self, 0)
    }
    ///Bit 4 - CRYP2RST
    #[inline(always)]
    pub fn cryp2rst(&mut self) -> CRYP2RST_W<'_, AHB3RSTSETRrs> {
        CRYP2RST_W::new(self, 4)
    }
    ///Bit 5 - HASH2RST
    #[inline(always)]
    pub fn hash2rst(&mut self) -> HASH2RST_W<'_, AHB3RSTSETRrs> {
        HASH2RST_W::new(self, 5)
    }
    ///Bit 6 - RNG2RST
    #[inline(always)]
    pub fn rng2rst(&mut self) -> RNG2RST_W<'_, AHB3RSTSETRrs> {
        RNG2RST_W::new(self, 6)
    }
    ///Bit 7 - CRC2RST
    #[inline(always)]
    pub fn crc2rst(&mut self) -> CRC2RST_W<'_, AHB3RSTSETRrs> {
        CRC2RST_W::new(self, 7)
    }
    ///Bit 11 - HSEMRST
    #[inline(always)]
    pub fn hsemrst(&mut self) -> HSEMRST_W<'_, AHB3RSTSETRrs> {
        HSEMRST_W::new(self, 11)
    }
    ///Bit 12 - IPCCRST
    #[inline(always)]
    pub fn ipccrst(&mut self) -> IPCCRST_W<'_, AHB3RSTSETRrs> {
        IPCCRST_W::new(self, 12)
    }
}
/**This register is used to activate the reset of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`ahb3rstsetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstsetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:AHB3RSTSETR)*/
pub struct AHB3RSTSETRrs;
impl crate::RegisterSpec for AHB3RSTSETRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3rstsetr::R`](R) reader structure
impl crate::Readable for AHB3RSTSETRrs {}
///`write(|w| ..)` method takes [`ahb3rstsetr::W`](W) writer structure
impl crate::Writable for AHB3RSTSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3RSTSETR to value 0
impl crate::Resettable for AHB3RSTSETRrs {}
