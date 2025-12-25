///Register `MP_AHB3LPENSETR` reader
pub type R = crate::R<MP_AHB3LPENSETRrs>;
///Register `MP_AHB3LPENSETR` writer
pub type W = crate::W<MP_AHB3LPENSETRrs>;
///Field `DCMILPEN` reader - DCMILPEN
pub type DCMILPEN_R = crate::BitReader;
///Field `DCMILPEN` writer - DCMILPEN
pub type DCMILPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYP2LPEN` reader - CRYP2LPEN
pub type CRYP2LPEN_R = crate::BitReader;
///Field `CRYP2LPEN` writer - CRYP2LPEN
pub type CRYP2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASH2LPEN` reader - HASH2LPEN
pub type HASH2LPEN_R = crate::BitReader;
///Field `HASH2LPEN` writer - HASH2LPEN
pub type HASH2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNG2LPEN` reader - RNG2LPEN
pub type RNG2LPEN_R = crate::BitReader;
///Field `RNG2LPEN` writer - RNG2LPEN
pub type RNG2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRC2LPEN` reader - CRC2LPEN
pub type CRC2LPEN_R = crate::BitReader;
///Field `CRC2LPEN` writer - CRC2LPEN
pub type CRC2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEMLPEN` reader - HSEMLPEN
pub type HSEMLPEN_R = crate::BitReader;
///Field `HSEMLPEN` writer - HSEMLPEN
pub type HSEMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IPCCLPEN` reader - IPCCLPEN
pub type IPCCLPEN_R = crate::BitReader;
///Field `IPCCLPEN` writer - IPCCLPEN
pub type IPCCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DCMILPEN
    #[inline(always)]
    pub fn dcmilpen(&self) -> DCMILPEN_R {
        DCMILPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - CRYP2LPEN
    #[inline(always)]
    pub fn cryp2lpen(&self) -> CRYP2LPEN_R {
        CRYP2LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HASH2LPEN
    #[inline(always)]
    pub fn hash2lpen(&self) -> HASH2LPEN_R {
        HASH2LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RNG2LPEN
    #[inline(always)]
    pub fn rng2lpen(&self) -> RNG2LPEN_R {
        RNG2LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CRC2LPEN
    #[inline(always)]
    pub fn crc2lpen(&self) -> CRC2LPEN_R {
        CRC2LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - HSEMLPEN
    #[inline(always)]
    pub fn hsemlpen(&self) -> HSEMLPEN_R {
        HSEMLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - IPCCLPEN
    #[inline(always)]
    pub fn ipcclpen(&self) -> IPCCLPEN_R {
        IPCCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MP_AHB3LPENSETR")
            .field("dcmilpen", &self.dcmilpen())
            .field("cryp2lpen", &self.cryp2lpen())
            .field("hash2lpen", &self.hash2lpen())
            .field("rng2lpen", &self.rng2lpen())
            .field("crc2lpen", &self.crc2lpen())
            .field("hsemlpen", &self.hsemlpen())
            .field("ipcclpen", &self.ipcclpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DCMILPEN
    #[inline(always)]
    pub fn dcmilpen(&mut self) -> DCMILPEN_W<'_, MP_AHB3LPENSETRrs> {
        DCMILPEN_W::new(self, 0)
    }
    ///Bit 4 - CRYP2LPEN
    #[inline(always)]
    pub fn cryp2lpen(&mut self) -> CRYP2LPEN_W<'_, MP_AHB3LPENSETRrs> {
        CRYP2LPEN_W::new(self, 4)
    }
    ///Bit 5 - HASH2LPEN
    #[inline(always)]
    pub fn hash2lpen(&mut self) -> HASH2LPEN_W<'_, MP_AHB3LPENSETRrs> {
        HASH2LPEN_W::new(self, 5)
    }
    ///Bit 6 - RNG2LPEN
    #[inline(always)]
    pub fn rng2lpen(&mut self) -> RNG2LPEN_W<'_, MP_AHB3LPENSETRrs> {
        RNG2LPEN_W::new(self, 6)
    }
    ///Bit 7 - CRC2LPEN
    #[inline(always)]
    pub fn crc2lpen(&mut self) -> CRC2LPEN_W<'_, MP_AHB3LPENSETRrs> {
        CRC2LPEN_W::new(self, 7)
    }
    ///Bit 11 - HSEMLPEN
    #[inline(always)]
    pub fn hsemlpen(&mut self) -> HSEMLPEN_W<'_, MP_AHB3LPENSETRrs> {
        HSEMLPEN_W::new(self, 11)
    }
    ///Bit 12 - IPCCLPEN
    #[inline(always)]
    pub fn ipcclpen(&mut self) -> IPCCLPEN_W<'_, MP_AHB3LPENSETRrs> {
        IPCCLPEN_W::new(self, 12)
    }
}
/**This register is used by the MPU

You can [`read`](crate::Reg::read) this register and get [`mp_ahb3lpensetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb3lpensetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB3LPENSETR)*/
pub struct MP_AHB3LPENSETRrs;
impl crate::RegisterSpec for MP_AHB3LPENSETRrs {
    type Ux = u32;
}
///`read()` method returns [`mp_ahb3lpensetr::R`](R) reader structure
impl crate::Readable for MP_AHB3LPENSETRrs {}
///`write(|w| ..)` method takes [`mp_ahb3lpensetr::W`](W) writer structure
impl crate::Writable for MP_AHB3LPENSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MP_AHB3LPENSETR to value 0x18f1
impl crate::Resettable for MP_AHB3LPENSETRrs {
    const RESET_VALUE: u32 = 0x18f1;
}
