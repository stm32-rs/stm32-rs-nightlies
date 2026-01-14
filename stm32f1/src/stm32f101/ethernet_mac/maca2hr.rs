///Register `MACA2HR` reader
pub type R = crate::R<MACA2HRrs>;
///Register `MACA2HR` writer
pub type W = crate::W<MACA2HRrs>;
///Field `ETH_MACA2HR` reader - Ethernet MAC address 2 high register
pub type ETH_MACA2HR_R = crate::FieldReader<u16>;
///Field `ETH_MACA2HR` writer - Ethernet MAC address 2 high register
pub type ETH_MACA2HR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `MBC` reader - Mask byte control
pub type MBC_R = crate::FieldReader;
///Field `MBC` writer - Mask byte control
pub type MBC_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SA` reader - Source address
pub type SA_R = crate::BitReader;
///Field `SA` writer - Source address
pub type SA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AE` reader - Address enable
pub type AE_R = crate::BitReader;
///Field `AE` writer - Address enable
pub type AE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - Ethernet MAC address 2 high register
    #[inline(always)]
    pub fn eth_maca2hr(&self) -> ETH_MACA2HR_R {
        ETH_MACA2HR_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 24:29 - Mask byte control
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    ///Bit 30 - Source address
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Address enable
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA2HR")
            .field("eth_maca2hr", &self.eth_maca2hr())
            .field("mbc", &self.mbc())
            .field("sa", &self.sa())
            .field("ae", &self.ae())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Ethernet MAC address 2 high register
    #[inline(always)]
    pub fn eth_maca2hr(&mut self) -> ETH_MACA2HR_W<'_, MACA2HRrs> {
        ETH_MACA2HR_W::new(self, 0)
    }
    ///Bits 24:29 - Mask byte control
    #[inline(always)]
    pub fn mbc(&mut self) -> MBC_W<'_, MACA2HRrs> {
        MBC_W::new(self, 24)
    }
    ///Bit 30 - Source address
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W<'_, MACA2HRrs> {
        SA_W::new(self, 30)
    }
    ///Bit 31 - Address enable
    #[inline(always)]
    pub fn ae(&mut self) -> AE_W<'_, MACA2HRrs> {
        AE_W::new(self, 31)
    }
}
/**Ethernet MAC address 2 high register (ETH_MACA2HR)

You can [`read`](crate::Reg::read) this register and get [`maca2hr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca2hr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#Ethernet_MAC:MACA2HR)*/
pub struct MACA2HRrs;
impl crate::RegisterSpec for MACA2HRrs {
    type Ux = u32;
}
///`read()` method returns [`maca2hr::R`](R) reader structure
impl crate::Readable for MACA2HRrs {}
///`write(|w| ..)` method takes [`maca2hr::W`](W) writer structure
impl crate::Writable for MACA2HRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACA2HR to value 0x50
impl crate::Resettable for MACA2HRrs {
    const RESET_VALUE: u32 = 0x50;
}
