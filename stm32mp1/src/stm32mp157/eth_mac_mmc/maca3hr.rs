///Register `MACA3HR` reader
pub type R = crate::R<MACA3HRrs>;
///Register `MACA3HR` writer
pub type W = crate::W<MACA3HRrs>;
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
        f.debug_struct("MACA3HR")
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
    pub fn addrhi(&mut self) -> ADDRHI_W<'_, MACA3HRrs> {
        ADDRHI_W::new(self, 0)
    }
    ///Bits 24:29 - MBC
    #[inline(always)]
    pub fn mbc(&mut self) -> MBC_W<'_, MACA3HRrs> {
        MBC_W::new(self, 24)
    }
    ///Bit 30 - SA
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W<'_, MACA3HRrs> {
        SA_W::new(self, 30)
    }
    ///Bit 31 - AE
    #[inline(always)]
    pub fn ae(&mut self) -> AE_W<'_, MACA3HRrs> {
        AE_W::new(self, 31)
    }
}
/**The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.

You can [`read`](crate::Reg::read) this register and get [`maca3hr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca3hr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:MACA3HR)*/
pub struct MACA3HRrs;
impl crate::RegisterSpec for MACA3HRrs {
    type Ux = u32;
}
///`read()` method returns [`maca3hr::R`](R) reader structure
impl crate::Readable for MACA3HRrs {}
///`write(|w| ..)` method takes [`maca3hr::W`](W) writer structure
impl crate::Writable for MACA3HRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACA3HR to value 0xffff
impl crate::Resettable for MACA3HRrs {
    const RESET_VALUE: u32 = 0xffff;
}
