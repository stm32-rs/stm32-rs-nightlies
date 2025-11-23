///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
///Field `LSION` reader - LSI oscillator enable
pub type LSION_R = crate::BitReader;
///Field `LSION` writer - LSI oscillator enable
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSIRDY` reader - LSI oscillator ready
pub type LSIRDY_R = crate::BitReader;
///Field `MSISRANGE` reader - SI range after Standby mode
pub type MSISRANGE_R = crate::FieldReader;
///Field `MSISRANGE` writer - SI range after Standby mode
pub type MSISRANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RMVF` reader - Remove reset flag
pub type RMVF_R = crate::BitReader;
///Field `RMVF` writer - Remove reset flag
pub type RMVF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIREWALLRSTF` reader - Firewall reset flag
pub type FIREWALLRSTF_R = crate::BitReader;
///Field `OBLRSTF` reader - Option byte loader reset flag
pub type OBLRSTF_R = crate::BitReader;
///Field `PINRSTF` reader - Pin reset flag
pub type PINRSTF_R = crate::BitReader;
///Field `BORRSTF` reader - BOR flag
pub type BORRSTF_R = crate::BitReader;
///Field `SFTRSTF` reader - Software reset flag
pub type SFTRSTF_R = crate::BitReader;
///Field `IWDGRSTF` reader - Independent window watchdog reset flag
pub type IWDGRSTF_R = crate::BitReader;
///Field `WWDGRSTF` reader - Window watchdog reset flag
pub type WWDGRSTF_R = crate::BitReader;
///Field `LPWRSTF` reader - Low-power reset flag
pub type LPWRSTF_R = crate::BitReader;
impl R {
    ///Bit 0 - LSI oscillator enable
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSI oscillator ready
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 8:11 - SI range after Standby mode
    #[inline(always)]
    pub fn msisrange(&self) -> MSISRANGE_R {
        MSISRANGE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 23 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Firewall reset flag
    #[inline(always)]
    pub fn firewallrstf(&self) -> FIREWALLRSTF_R {
        FIREWALLRSTF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Option byte loader reset flag
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Pin reset flag
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - BOR flag
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Software reset flag
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Independent window watchdog reset flag
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Window watchdog reset flag
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low-power reset flag
    #[inline(always)]
    pub fn lpwrstf(&self) -> LPWRSTF_R {
        LPWRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("lpwrstf", &self.lpwrstf())
            .field("wwdgrstf", &self.wwdgrstf())
            .field("iwdgrstf", &self.iwdgrstf())
            .field("sftrstf", &self.sftrstf())
            .field("borrstf", &self.borrstf())
            .field("pinrstf", &self.pinrstf())
            .field("oblrstf", &self.oblrstf())
            .field("firewallrstf", &self.firewallrstf())
            .field("rmvf", &self.rmvf())
            .field("msisrange", &self.msisrange())
            .field("lsirdy", &self.lsirdy())
            .field("lsion", &self.lsion())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSI oscillator enable
    #[inline(always)]
    pub fn lsion(&mut self) -> LSION_W<'_, CSRrs> {
        LSION_W::new(self, 0)
    }
    ///Bits 8:11 - SI range after Standby mode
    #[inline(always)]
    pub fn msisrange(&mut self) -> MSISRANGE_W<'_, CSRrs> {
        MSISRANGE_W::new(self, 8)
    }
    ///Bit 23 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W<'_, CSRrs> {
        RMVF_W::new(self, 23)
    }
}
/**CSR

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x5.html#RCC:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR to value 0x0c00_0600
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0x0c00_0600;
}
