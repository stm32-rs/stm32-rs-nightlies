///Register `MACA1HR` reader
pub type R = crate::R<MACA1HRrs>;
///Register `MACA1HR` writer
pub type W = crate::W<MACA1HRrs>;
///Field `ADDRHI` reader - ADDRHI
pub type ADDRHI_R = crate::FieldReader<u16>;
///Field `ADDRHI` writer - ADDRHI
pub type ADDRHI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `MBC` reader - MBC
pub type MBC_R = crate::FieldReader;
///Field `MBC` writer - MBC
pub type MBC_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SA` reader - SA
pub type SA_R = crate::BitReader;
///Field `SA` writer - SA
pub type SA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AE` reader - AE
pub type AE_R = crate::BitReader;
///Field `AE` writer - AE
pub type AE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - ADDRHI
    #[inline(always)]
    pub fn addrhi(&self) -> ADDRHI_R {
        ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 24:29 - MBC
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    ///Bit 30 - SA
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - AE
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA1HR")
            .field("addrhi", &self.addrhi())
            .field("mbc", &self.mbc())
            .field("sa", &self.sa())
            .field("ae", &self.ae())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - ADDRHI
    #[inline(always)]
    pub fn addrhi(&mut self) -> ADDRHI_W<MACA1HRrs> {
        ADDRHI_W::new(self, 0)
    }
    ///Bits 24:29 - MBC
    #[inline(always)]
    pub fn mbc(&mut self) -> MBC_W<MACA1HRrs> {
        MBC_W::new(self, 24)
    }
    ///Bit 30 - SA
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W<MACA1HRrs> {
        SA_W::new(self, 30)
    }
    ///Bit 31 - AE
    #[inline(always)]
    pub fn ae(&mut self) -> AE_W<MACA1HRrs> {
        AE_W::new(self, 31)
    }
}
/**The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.

You can [`read`](crate::Reg::read) this register and get [`maca1hr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca1hr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACA1HR)*/
pub struct MACA1HRrs;
impl crate::RegisterSpec for MACA1HRrs {
    type Ux = u32;
}
///`read()` method returns [`maca1hr::R`](R) reader structure
impl crate::Readable for MACA1HRrs {}
///`write(|w| ..)` method takes [`maca1hr::W`](W) writer structure
impl crate::Writable for MACA1HRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MACA1HR to value 0xffff
impl crate::Resettable for MACA1HRrs {
    const RESET_VALUE: u32 = 0xffff;
}