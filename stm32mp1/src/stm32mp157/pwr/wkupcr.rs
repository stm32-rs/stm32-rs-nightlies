///Register `WKUPCR` reader
pub type R = crate::R<WKUPCRrs>;
///Register `WKUPCR` writer
pub type W = crate::W<WKUPCRrs>;
///Field `WKUPC1` reader - WKUPC1
pub type WKUPC1_R = crate::BitReader;
///Field `WKUPC1` writer - WKUPC1
pub type WKUPC1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPC2` reader - WKUPC2
pub type WKUPC2_R = crate::BitReader;
///Field `WKUPC2` writer - WKUPC2
pub type WKUPC2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPC3` reader - WKUPC3
pub type WKUPC3_R = crate::BitReader;
///Field `WKUPC3` writer - WKUPC3
pub type WKUPC3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPC4` reader - WKUPC4
pub type WKUPC4_R = crate::BitReader;
///Field `WKUPC4` writer - WKUPC4
pub type WKUPC4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPC5` reader - WKUPC5
pub type WKUPC5_R = crate::BitReader;
///Field `WKUPC5` writer - WKUPC5
pub type WKUPC5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPC6` reader - WKUPC6
pub type WKUPC6_R = crate::BitReader;
///Field `WKUPC6` writer - WKUPC6
pub type WKUPC6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPP1` reader - WKUPP1
pub type WKUPP1_R = crate::BitReader;
///Field `WKUPP1` writer - WKUPP1
pub type WKUPP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPP2` reader - WKUPP2
pub type WKUPP2_R = crate::BitReader;
///Field `WKUPP2` writer - WKUPP2
pub type WKUPP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPP3` reader - WKUPP3
pub type WKUPP3_R = crate::BitReader;
///Field `WKUPP3` writer - WKUPP3
pub type WKUPP3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPP4` reader - WKUPP4
pub type WKUPP4_R = crate::BitReader;
///Field `WKUPP4` writer - WKUPP4
pub type WKUPP4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPP5` reader - WKUPP5
pub type WKUPP5_R = crate::BitReader;
///Field `WKUPP5` writer - WKUPP5
pub type WKUPP5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPP6` reader - WKUPP6
pub type WKUPP6_R = crate::BitReader;
///Field `WKUPP6` writer - WKUPP6
pub type WKUPP6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPPUPD1` reader - WKUPPUPD1
pub type WKUPPUPD1_R = crate::FieldReader;
///Field `WKUPPUPD1` writer - WKUPPUPD1
pub type WKUPPUPD1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WKUPPUPD2` reader - WKUPPUPD2
pub type WKUPPUPD2_R = crate::FieldReader;
///Field `WKUPPUPD2` writer - WKUPPUPD2
pub type WKUPPUPD2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WKUPPUPD3` reader - WKUPPUPD3
pub type WKUPPUPD3_R = crate::FieldReader;
///Field `WKUPPUPD3` writer - WKUPPUPD3
pub type WKUPPUPD3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WKUPPUPD4` reader - WKUPPUPD4
pub type WKUPPUPD4_R = crate::FieldReader;
///Field `WKUPPUPD4` writer - WKUPPUPD4
pub type WKUPPUPD4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WKUPPUPD5` reader - WKUPPUPD5
pub type WKUPPUPD5_R = crate::FieldReader;
///Field `WKUPPUPD5` writer - WKUPPUPD5
pub type WKUPPUPD5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WKUPPUPD6` reader - WKUPPUPD6
pub type WKUPPUPD6_R = crate::FieldReader;
///Field `WKUPPUPD6` writer - WKUPPUPD6
pub type WKUPPUPD6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - WKUPC1
    #[inline(always)]
    pub fn wkupc1(&self) -> WKUPC1_R {
        WKUPC1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WKUPC2
    #[inline(always)]
    pub fn wkupc2(&self) -> WKUPC2_R {
        WKUPC2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WKUPC3
    #[inline(always)]
    pub fn wkupc3(&self) -> WKUPC3_R {
        WKUPC3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - WKUPC4
    #[inline(always)]
    pub fn wkupc4(&self) -> WKUPC4_R {
        WKUPC4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - WKUPC5
    #[inline(always)]
    pub fn wkupc5(&self) -> WKUPC5_R {
        WKUPC5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - WKUPC6
    #[inline(always)]
    pub fn wkupc6(&self) -> WKUPC6_R {
        WKUPC6_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - WKUPP1
    #[inline(always)]
    pub fn wkupp1(&self) -> WKUPP1_R {
        WKUPP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - WKUPP2
    #[inline(always)]
    pub fn wkupp2(&self) -> WKUPP2_R {
        WKUPP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - WKUPP3
    #[inline(always)]
    pub fn wkupp3(&self) -> WKUPP3_R {
        WKUPP3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - WKUPP4
    #[inline(always)]
    pub fn wkupp4(&self) -> WKUPP4_R {
        WKUPP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - WKUPP5
    #[inline(always)]
    pub fn wkupp5(&self) -> WKUPP5_R {
        WKUPP5_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - WKUPP6
    #[inline(always)]
    pub fn wkupp6(&self) -> WKUPP6_R {
        WKUPP6_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 16:17 - WKUPPUPD1
    #[inline(always)]
    pub fn wkuppupd1(&self) -> WKUPPUPD1_R {
        WKUPPUPD1_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - WKUPPUPD2
    #[inline(always)]
    pub fn wkuppupd2(&self) -> WKUPPUPD2_R {
        WKUPPUPD2_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - WKUPPUPD3
    #[inline(always)]
    pub fn wkuppupd3(&self) -> WKUPPUPD3_R {
        WKUPPUPD3_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - WKUPPUPD4
    #[inline(always)]
    pub fn wkuppupd4(&self) -> WKUPPUPD4_R {
        WKUPPUPD4_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - WKUPPUPD5
    #[inline(always)]
    pub fn wkuppupd5(&self) -> WKUPPUPD5_R {
        WKUPPUPD5_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - WKUPPUPD6
    #[inline(always)]
    pub fn wkuppupd6(&self) -> WKUPPUPD6_R {
        WKUPPUPD6_R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WKUPCR")
            .field("wkupc1", &self.wkupc1())
            .field("wkupc2", &self.wkupc2())
            .field("wkupc3", &self.wkupc3())
            .field("wkupc4", &self.wkupc4())
            .field("wkupc5", &self.wkupc5())
            .field("wkupc6", &self.wkupc6())
            .field("wkupp1", &self.wkupp1())
            .field("wkupp2", &self.wkupp2())
            .field("wkupp3", &self.wkupp3())
            .field("wkupp4", &self.wkupp4())
            .field("wkupp5", &self.wkupp5())
            .field("wkupp6", &self.wkupp6())
            .field("wkuppupd1", &self.wkuppupd1())
            .field("wkuppupd2", &self.wkuppupd2())
            .field("wkuppupd3", &self.wkuppupd3())
            .field("wkuppupd4", &self.wkuppupd4())
            .field("wkuppupd5", &self.wkuppupd5())
            .field("wkuppupd6", &self.wkuppupd6())
            .finish()
    }
}
impl W {
    ///Bit 0 - WKUPC1
    #[inline(always)]
    pub fn wkupc1(&mut self) -> WKUPC1_W<'_, WKUPCRrs> {
        WKUPC1_W::new(self, 0)
    }
    ///Bit 1 - WKUPC2
    #[inline(always)]
    pub fn wkupc2(&mut self) -> WKUPC2_W<'_, WKUPCRrs> {
        WKUPC2_W::new(self, 1)
    }
    ///Bit 2 - WKUPC3
    #[inline(always)]
    pub fn wkupc3(&mut self) -> WKUPC3_W<'_, WKUPCRrs> {
        WKUPC3_W::new(self, 2)
    }
    ///Bit 3 - WKUPC4
    #[inline(always)]
    pub fn wkupc4(&mut self) -> WKUPC4_W<'_, WKUPCRrs> {
        WKUPC4_W::new(self, 3)
    }
    ///Bit 4 - WKUPC5
    #[inline(always)]
    pub fn wkupc5(&mut self) -> WKUPC5_W<'_, WKUPCRrs> {
        WKUPC5_W::new(self, 4)
    }
    ///Bit 5 - WKUPC6
    #[inline(always)]
    pub fn wkupc6(&mut self) -> WKUPC6_W<'_, WKUPCRrs> {
        WKUPC6_W::new(self, 5)
    }
    ///Bit 8 - WKUPP1
    #[inline(always)]
    pub fn wkupp1(&mut self) -> WKUPP1_W<'_, WKUPCRrs> {
        WKUPP1_W::new(self, 8)
    }
    ///Bit 9 - WKUPP2
    #[inline(always)]
    pub fn wkupp2(&mut self) -> WKUPP2_W<'_, WKUPCRrs> {
        WKUPP2_W::new(self, 9)
    }
    ///Bit 10 - WKUPP3
    #[inline(always)]
    pub fn wkupp3(&mut self) -> WKUPP3_W<'_, WKUPCRrs> {
        WKUPP3_W::new(self, 10)
    }
    ///Bit 11 - WKUPP4
    #[inline(always)]
    pub fn wkupp4(&mut self) -> WKUPP4_W<'_, WKUPCRrs> {
        WKUPP4_W::new(self, 11)
    }
    ///Bit 12 - WKUPP5
    #[inline(always)]
    pub fn wkupp5(&mut self) -> WKUPP5_W<'_, WKUPCRrs> {
        WKUPP5_W::new(self, 12)
    }
    ///Bit 13 - WKUPP6
    #[inline(always)]
    pub fn wkupp6(&mut self) -> WKUPP6_W<'_, WKUPCRrs> {
        WKUPP6_W::new(self, 13)
    }
    ///Bits 16:17 - WKUPPUPD1
    #[inline(always)]
    pub fn wkuppupd1(&mut self) -> WKUPPUPD1_W<'_, WKUPCRrs> {
        WKUPPUPD1_W::new(self, 16)
    }
    ///Bits 18:19 - WKUPPUPD2
    #[inline(always)]
    pub fn wkuppupd2(&mut self) -> WKUPPUPD2_W<'_, WKUPCRrs> {
        WKUPPUPD2_W::new(self, 18)
    }
    ///Bits 20:21 - WKUPPUPD3
    #[inline(always)]
    pub fn wkuppupd3(&mut self) -> WKUPPUPD3_W<'_, WKUPCRrs> {
        WKUPPUPD3_W::new(self, 20)
    }
    ///Bits 22:23 - WKUPPUPD4
    #[inline(always)]
    pub fn wkuppupd4(&mut self) -> WKUPPUPD4_W<'_, WKUPCRrs> {
        WKUPPUPD4_W::new(self, 22)
    }
    ///Bits 24:25 - WKUPPUPD5
    #[inline(always)]
    pub fn wkuppupd5(&mut self) -> WKUPPUPD5_W<'_, WKUPCRrs> {
        WKUPPUPD5_W::new(self, 24)
    }
    ///Bits 26:27 - WKUPPUPD6
    #[inline(always)]
    pub fn wkuppupd6(&mut self) -> WKUPPUPD6_W<'_, WKUPCRrs> {
        WKUPPUPD6_W::new(self, 26)
    }
}
/**Not reset by wakeup from Standby mode, but by any application reset (such as NRST, IWDG). Access 6 wait states when writing this register (when clearing a WKUPF, the AHB write access completes after the WKUPF has cleared). This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access on individual WKUPC\[6:1\], WKUPP\[6:1\] bits and WKUPPUPD\[6:1\] bit pairs are discarded when the corresponding WKUPEN\[6:1\] bit in PWR MPU wakeup enable register (PWR_MPUWKUPENR) is set. No bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.

You can [`read`](crate::Reg::read) this register and get [`wkupcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkupcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#PWR:WKUPCR)*/
pub struct WKUPCRrs;
impl crate::RegisterSpec for WKUPCRrs {
    type Ux = u32;
}
///`read()` method returns [`wkupcr::R`](R) reader structure
impl crate::Readable for WKUPCRrs {}
///`write(|w| ..)` method takes [`wkupcr::W`](W) writer structure
impl crate::Writable for WKUPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WKUPCR to value 0
impl crate::Resettable for WKUPCRrs {}
