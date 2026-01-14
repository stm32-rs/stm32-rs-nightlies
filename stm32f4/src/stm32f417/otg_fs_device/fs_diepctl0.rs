///Register `FS_DIEPCTL0` reader
pub type R = crate::R<FS_DIEPCTL0rs>;
///Register `FS_DIEPCTL0` writer
pub type W = crate::W<FS_DIEPCTL0rs>;
///Field `MPSIZ` reader - Maximum packet size
pub type MPSIZ_R = crate::FieldReader;
///Field `MPSIZ` writer - Maximum packet size
pub type MPSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `USBAEP` reader - USB active endpoint
pub type USBAEP_R = crate::BitReader;
///Field `NAKSTS` reader - NAK status
pub type NAKSTS_R = crate::BitReader;
///Field `EPTYP` reader - Endpoint type
pub type EPTYP_R = crate::FieldReader;
///Field `STALL` reader - STALL handshake
pub type STALL_R = crate::BitReader;
///Field `STALL` writer - STALL handshake
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFNUM` reader - TxFIFO number
pub type TXFNUM_R = crate::FieldReader;
///Field `TXFNUM` writer - TxFIFO number
pub type TXFNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CNAK` writer - Clear NAK
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNAK` writer - Set NAK
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDIS` reader - Endpoint disable
pub type EPDIS_R = crate::BitReader;
///Field `EPENA` reader - Endpoint enable
pub type EPENA_R = crate::BitReader;
impl R {
    ///Bits 0:1 - Maximum packet size
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 3) as u8)
    }
    ///Bit 15 - USB active endpoint
    #[inline(always)]
    pub fn usbaep(&self) -> USBAEP_R {
        USBAEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - NAK status
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - Endpoint type
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 21 - STALL handshake
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:25 - TxFIFO number
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    ///Bit 30 - Endpoint disable
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Endpoint enable
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_DIEPCTL0")
            .field("mpsiz", &self.mpsiz())
            .field("usbaep", &self.usbaep())
            .field("naksts", &self.naksts())
            .field("eptyp", &self.eptyp())
            .field("stall", &self.stall())
            .field("txfnum", &self.txfnum())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Maximum packet size
    #[inline(always)]
    pub fn mpsiz(&mut self) -> MPSIZ_W<'_, FS_DIEPCTL0rs> {
        MPSIZ_W::new(self, 0)
    }
    ///Bit 21 - STALL handshake
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W<'_, FS_DIEPCTL0rs> {
        STALL_W::new(self, 21)
    }
    ///Bits 22:25 - TxFIFO number
    #[inline(always)]
    pub fn txfnum(&mut self) -> TXFNUM_W<'_, FS_DIEPCTL0rs> {
        TXFNUM_W::new(self, 22)
    }
    ///Bit 26 - Clear NAK
    #[inline(always)]
    pub fn cnak(&mut self) -> CNAK_W<'_, FS_DIEPCTL0rs> {
        CNAK_W::new(self, 26)
    }
    ///Bit 27 - Set NAK
    #[inline(always)]
    pub fn snak(&mut self) -> SNAK_W<'_, FS_DIEPCTL0rs> {
        SNAK_W::new(self, 27)
    }
}
/**OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)

You can [`read`](crate::Reg::read) this register and get [`fs_diepctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_diepctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_FS_DEVICE:FS_DIEPCTL0)*/
pub struct FS_DIEPCTL0rs;
impl crate::RegisterSpec for FS_DIEPCTL0rs {
    type Ux = u32;
}
///`read()` method returns [`fs_diepctl0::R`](R) reader structure
impl crate::Readable for FS_DIEPCTL0rs {}
///`write(|w| ..)` method takes [`fs_diepctl0::W`](W) writer structure
impl crate::Writable for FS_DIEPCTL0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FS_DIEPCTL0 to value 0
impl crate::Resettable for FS_DIEPCTL0rs {}
