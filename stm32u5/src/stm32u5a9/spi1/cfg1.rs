#[doc = "Register `CFG1` reader"]
pub type R = crate::R<CFG1rs>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<CFG1rs>;
#[doc = "Field `DSIZE` reader - Number of bits in at single SPI data frame"]
pub type DSIZE_R = crate::FieldReader;
#[doc = "Field `DSIZE` writer - Number of bits in at single SPI data frame"]
pub type DSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FTHVL` reader - threshold level"]
pub type FTHVL_R = crate::FieldReader;
#[doc = "Field `FTHVL` writer - threshold level"]
pub type FTHVL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UDRCFG` reader - Behavior of slave transmitter at underrun condition"]
pub type UDRCFG_R = crate::BitReader;
#[doc = "Field `UDRCFG` writer - Behavior of slave transmitter at underrun condition"]
pub type UDRCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMAEN` reader - Rx DMA stream enable"]
pub type RXDMAEN_R = crate::BitReader;
#[doc = "Field `RXDMAEN` writer - Rx DMA stream enable"]
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAEN` reader - Tx DMA stream enable"]
pub type TXDMAEN_R = crate::BitReader;
#[doc = "Field `TXDMAEN` writer - Tx DMA stream enable"]
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCSIZE` reader - Length of CRC frame to be transacted and compared"]
pub type CRCSIZE_R = crate::FieldReader;
#[doc = "Field `CRCSIZE` writer - Length of CRC frame to be transacted and compared"]
pub type CRCSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CRCEN` reader - Hardware CRC computation enable"]
pub type CRCEN_R = crate::BitReader;
#[doc = "Field `CRCEN` writer - Hardware CRC computation enable"]
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBR` reader - Master baud rate"]
pub type MBR_R = crate::FieldReader;
#[doc = "Field `MBR` writer - Master baud rate"]
pub type MBR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BPASS` reader - BPASS"]
pub type BPASS_R = crate::BitReader;
#[doc = "Field `BPASS` writer - BPASS"]
pub type BPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Number of bits in at single SPI data frame"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:8 - threshold level"]
    #[inline(always)]
    pub fn fthvl(&self) -> FTHVL_R {
        FTHVL_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Behavior of slave transmitter at underrun condition"]
    #[inline(always)]
    pub fn udrcfg(&self) -> UDRCFG_R {
        UDRCFG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Rx DMA stream enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx DMA stream enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Length of CRC frame to be transacted and compared"]
    #[inline(always)]
    pub fn crcsize(&self) -> CRCSIZE_R {
        CRCSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - Hardware CRC computation enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Master baud rate"]
    #[inline(always)]
    pub fn mbr(&self) -> MBR_R {
        MBR_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - BPASS"]
    #[inline(always)]
    pub fn bpass(&self) -> BPASS_R {
        BPASS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of bits in at single SPI data frame"]
    #[inline(always)]
    #[must_use]
    pub fn dsize(&mut self) -> DSIZE_W<CFG1rs> {
        DSIZE_W::new(self, 0)
    }
    #[doc = "Bits 5:8 - threshold level"]
    #[inline(always)]
    #[must_use]
    pub fn fthvl(&mut self) -> FTHVL_W<CFG1rs> {
        FTHVL_W::new(self, 5)
    }
    #[doc = "Bit 9 - Behavior of slave transmitter at underrun condition"]
    #[inline(always)]
    #[must_use]
    pub fn udrcfg(&mut self) -> UDRCFG_W<CFG1rs> {
        UDRCFG_W::new(self, 9)
    }
    #[doc = "Bit 14 - Rx DMA stream enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<CFG1rs> {
        RXDMAEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Tx DMA stream enable"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<CFG1rs> {
        TXDMAEN_W::new(self, 15)
    }
    #[doc = "Bits 16:20 - Length of CRC frame to be transacted and compared"]
    #[inline(always)]
    #[must_use]
    pub fn crcsize(&mut self) -> CRCSIZE_W<CFG1rs> {
        CRCSIZE_W::new(self, 16)
    }
    #[doc = "Bit 22 - Hardware CRC computation enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<CFG1rs> {
        CRCEN_W::new(self, 22)
    }
    #[doc = "Bits 28:30 - Master baud rate"]
    #[inline(always)]
    #[must_use]
    pub fn mbr(&mut self) -> MBR_W<CFG1rs> {
        MBR_W::new(self, 28)
    }
    #[doc = "Bit 31 - BPASS"]
    #[inline(always)]
    #[must_use]
    pub fn bpass(&mut self) -> BPASS_W<CFG1rs> {
        BPASS_W::new(self, 31)
    }
}
#[doc = "configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG1rs;
impl crate::RegisterSpec for CFG1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for CFG1rs {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for CFG1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0x0007_0007"]
impl crate::Resettable for CFG1rs {
    const RESET_VALUE: u32 = 0x0007_0007;
}
