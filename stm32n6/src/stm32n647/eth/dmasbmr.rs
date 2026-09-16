///Register `DMASBMR` reader
pub type R = crate::R<DMASBMRrs>;
///Register `DMASBMR` writer
pub type W = crate::W<DMASBMRrs>;
///Field `FB` reader - Fixed Burst Length
pub type FB_R = crate::BitReader;
///Field `FB` writer - Fixed Burst Length
pub type FB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BLEN4` reader - AXI Burst Length 4
pub type BLEN4_R = crate::BitReader;
///Field `BLEN4` writer - AXI Burst Length 4
pub type BLEN4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BLEN8` reader - AXI Burst Length 8
pub type BLEN8_R = crate::BitReader;
///Field `BLEN8` writer - AXI Burst Length 8
pub type BLEN8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BLEN16` reader - AXI Burst Length 16
pub type BLEN16_R = crate::BitReader;
///Field `BLEN16` writer - AXI Burst Length 16
pub type BLEN16_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BLEN32` reader - AXI Burst Length 32
pub type BLEN32_R = crate::BitReader;
///Field `BLEN32` writer - AXI Burst Length 32
pub type BLEN32_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BLEN64` reader - AXI Burst Length 64
pub type BLEN64_R = crate::BitReader;
///Field `BLEN64` writer - AXI Burst Length 64
pub type BLEN64_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BLEN128` reader - AXI Burst Length 128
pub type BLEN128_R = crate::BitReader;
///Field `BLEN128` writer - AXI Burst Length 128
pub type BLEN128_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BLEN256` reader - AXI Burst Length 256
pub type BLEN256_R = crate::BitReader;
///Field `BLEN256` writer - AXI Burst Length 256
pub type BLEN256_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AALE` reader - Automatic AXI LPI enable
pub type AALE_R = crate::BitReader;
///Field `AALE` writer - Automatic AXI LPI enable
pub type AALE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AAL` reader - Address-Aligned Beats
pub type AAL_R = crate::BitReader;
///Field `AAL` writer - Address-Aligned Beats
pub type AAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ONEKBBE` reader - 1 Kbyte Boundary Crossing Enable for the AXI Master
pub type ONEKBBE_R = crate::BitReader;
///Field `ONEKBBE` writer - 1 Kbyte Boundary Crossing Enable for the AXI Master
pub type ONEKBBE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RD_OSR_LMT` reader - AXI Maximum Read Outstanding Request Limit
pub type RD_OSR_LMT_R = crate::FieldReader;
///Field `RD_OSR_LMT` writer - AXI Maximum Read Outstanding Request Limit
pub type RD_OSR_LMT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WR_OSR_LMT` reader - AXI Maximum Write Outstanding Request Limit
pub type WR_OSR_LMT_R = crate::FieldReader;
///Field `WR_OSR_LMT` writer - AXI Maximum Write Outstanding Request Limit
pub type WR_OSR_LMT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPI_XIT_PKT` reader - Unlock on Magic Packet or Remote wake-up packet
pub type LPI_XIT_PKT_R = crate::BitReader;
///Field `LPI_XIT_PKT` writer - Unlock on Magic Packet or Remote wake-up packet
pub type LPI_XIT_PKT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_LPI` reader - Enable Low Power Interface (LPI)
pub type EN_LPI_R = crate::BitReader;
///Field `EN_LPI` writer - Enable Low Power Interface (LPI)
pub type EN_LPI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Fixed Burst Length
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AXI Burst Length 4
    #[inline(always)]
    pub fn blen4(&self) -> BLEN4_R {
        BLEN4_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AXI Burst Length 8
    #[inline(always)]
    pub fn blen8(&self) -> BLEN8_R {
        BLEN8_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AXI Burst Length 16
    #[inline(always)]
    pub fn blen16(&self) -> BLEN16_R {
        BLEN16_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AXI Burst Length 32
    #[inline(always)]
    pub fn blen32(&self) -> BLEN32_R {
        BLEN32_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AXI Burst Length 64
    #[inline(always)]
    pub fn blen64(&self) -> BLEN64_R {
        BLEN64_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AXI Burst Length 128
    #[inline(always)]
    pub fn blen128(&self) -> BLEN128_R {
        BLEN128_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AXI Burst Length 256
    #[inline(always)]
    pub fn blen256(&self) -> BLEN256_R {
        BLEN256_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 10 - Automatic AXI LPI enable
    #[inline(always)]
    pub fn aale(&self) -> AALE_R {
        AALE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - Address-Aligned Beats
    #[inline(always)]
    pub fn aal(&self) -> AAL_R {
        AAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - 1 Kbyte Boundary Crossing Enable for the AXI Master
    #[inline(always)]
    pub fn onekbbe(&self) -> ONEKBBE_R {
        ONEKBBE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 16:17 - AXI Maximum Read Outstanding Request Limit
    #[inline(always)]
    pub fn rd_osr_lmt(&self) -> RD_OSR_LMT_R {
        RD_OSR_LMT_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 24:25 - AXI Maximum Write Outstanding Request Limit
    #[inline(always)]
    pub fn wr_osr_lmt(&self) -> WR_OSR_LMT_R {
        WR_OSR_LMT_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 30 - Unlock on Magic Packet or Remote wake-up packet
    #[inline(always)]
    pub fn lpi_xit_pkt(&self) -> LPI_XIT_PKT_R {
        LPI_XIT_PKT_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Enable Low Power Interface (LPI)
    #[inline(always)]
    pub fn en_lpi(&self) -> EN_LPI_R {
        EN_LPI_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMASBMR")
            .field("fb", &self.fb())
            .field("blen4", &self.blen4())
            .field("blen8", &self.blen8())
            .field("blen16", &self.blen16())
            .field("blen32", &self.blen32())
            .field("blen64", &self.blen64())
            .field("blen128", &self.blen128())
            .field("blen256", &self.blen256())
            .field("aale", &self.aale())
            .field("aal", &self.aal())
            .field("onekbbe", &self.onekbbe())
            .field("rd_osr_lmt", &self.rd_osr_lmt())
            .field("wr_osr_lmt", &self.wr_osr_lmt())
            .field("lpi_xit_pkt", &self.lpi_xit_pkt())
            .field("en_lpi", &self.en_lpi())
            .finish()
    }
}
impl W {
    ///Bit 0 - Fixed Burst Length
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W<'_, DMASBMRrs> {
        FB_W::new(self, 0)
    }
    ///Bit 1 - AXI Burst Length 4
    #[inline(always)]
    pub fn blen4(&mut self) -> BLEN4_W<'_, DMASBMRrs> {
        BLEN4_W::new(self, 1)
    }
    ///Bit 2 - AXI Burst Length 8
    #[inline(always)]
    pub fn blen8(&mut self) -> BLEN8_W<'_, DMASBMRrs> {
        BLEN8_W::new(self, 2)
    }
    ///Bit 3 - AXI Burst Length 16
    #[inline(always)]
    pub fn blen16(&mut self) -> BLEN16_W<'_, DMASBMRrs> {
        BLEN16_W::new(self, 3)
    }
    ///Bit 4 - AXI Burst Length 32
    #[inline(always)]
    pub fn blen32(&mut self) -> BLEN32_W<'_, DMASBMRrs> {
        BLEN32_W::new(self, 4)
    }
    ///Bit 5 - AXI Burst Length 64
    #[inline(always)]
    pub fn blen64(&mut self) -> BLEN64_W<'_, DMASBMRrs> {
        BLEN64_W::new(self, 5)
    }
    ///Bit 6 - AXI Burst Length 128
    #[inline(always)]
    pub fn blen128(&mut self) -> BLEN128_W<'_, DMASBMRrs> {
        BLEN128_W::new(self, 6)
    }
    ///Bit 7 - AXI Burst Length 256
    #[inline(always)]
    pub fn blen256(&mut self) -> BLEN256_W<'_, DMASBMRrs> {
        BLEN256_W::new(self, 7)
    }
    ///Bit 10 - Automatic AXI LPI enable
    #[inline(always)]
    pub fn aale(&mut self) -> AALE_W<'_, DMASBMRrs> {
        AALE_W::new(self, 10)
    }
    ///Bit 12 - Address-Aligned Beats
    #[inline(always)]
    pub fn aal(&mut self) -> AAL_W<'_, DMASBMRrs> {
        AAL_W::new(self, 12)
    }
    ///Bit 13 - 1 Kbyte Boundary Crossing Enable for the AXI Master
    #[inline(always)]
    pub fn onekbbe(&mut self) -> ONEKBBE_W<'_, DMASBMRrs> {
        ONEKBBE_W::new(self, 13)
    }
    ///Bits 16:17 - AXI Maximum Read Outstanding Request Limit
    #[inline(always)]
    pub fn rd_osr_lmt(&mut self) -> RD_OSR_LMT_W<'_, DMASBMRrs> {
        RD_OSR_LMT_W::new(self, 16)
    }
    ///Bits 24:25 - AXI Maximum Write Outstanding Request Limit
    #[inline(always)]
    pub fn wr_osr_lmt(&mut self) -> WR_OSR_LMT_W<'_, DMASBMRrs> {
        WR_OSR_LMT_W::new(self, 24)
    }
    ///Bit 30 - Unlock on Magic Packet or Remote wake-up packet
    #[inline(always)]
    pub fn lpi_xit_pkt(&mut self) -> LPI_XIT_PKT_W<'_, DMASBMRrs> {
        LPI_XIT_PKT_W::new(self, 30)
    }
    ///Bit 31 - Enable Low Power Interface (LPI)
    #[inline(always)]
    pub fn en_lpi(&mut self) -> EN_LPI_W<'_, DMASBMRrs> {
        EN_LPI_W::new(self, 31)
    }
}
/**System bus mode register

You can [`read`](crate::Reg::read) this register and get [`dmasbmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasbmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ETH:DMASBMR)*/
pub struct DMASBMRrs;
impl crate::RegisterSpec for DMASBMRrs {
    type Ux = u32;
}
///`read()` method returns [`dmasbmr::R`](R) reader structure
impl crate::Readable for DMASBMRrs {}
///`write(|w| ..)` method takes [`dmasbmr::W`](W) writer structure
impl crate::Writable for DMASBMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMASBMR to value 0x0101_0000
impl crate::Resettable for DMASBMRrs {
    const RESET_VALUE: u32 = 0x0101_0000;
}
