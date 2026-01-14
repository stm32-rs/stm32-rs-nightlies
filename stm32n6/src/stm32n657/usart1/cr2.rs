///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `SLVEN` reader - Synchronous Slave mode enable
pub type SLVEN_R = crate::BitReader;
///Field `SLVEN` writer - Synchronous Slave mode enable
pub type SLVEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIS_NSS` reader - When the DIS_NSS bit is set, the NSS pin input is ignored.
pub type DIS_NSS_R = crate::BitReader;
///Field `DIS_NSS` writer - When the DIS_NSS bit is set, the NSS pin input is ignored.
pub type DIS_NSS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADDM7` reader - 7-bit Address Detection/4-bit Address Detection
pub type ADDM7_R = crate::BitReader;
///Field `ADDM7` writer - 7-bit Address Detection/4-bit Address Detection
pub type ADDM7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LBDL` reader - LIN break detection length
pub type LBDL_R = crate::BitReader;
///Field `LBDL` writer - LIN break detection length
pub type LBDL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LBDIE` reader - LIN break detection interrupt enable
pub type LBDIE_R = crate::BitReader;
///Field `LBDIE` writer - LIN break detection interrupt enable
pub type LBDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LBCL` reader - Last bit clock pulse
pub type LBCL_R = crate::BitReader;
///Field `LBCL` writer - Last bit clock pulse
pub type LBCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPHA` reader - Clock phase
pub type CPHA_R = crate::BitReader;
///Field `CPHA` writer - Clock phase
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPOL` reader - Clock polarity
pub type CPOL_R = crate::BitReader;
///Field `CPOL` writer - Clock polarity
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLKEN` reader - Clock enable
pub type CLKEN_R = crate::BitReader;
///Field `CLKEN` writer - Clock enable
pub type CLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOP` reader - stop bits
pub type STOP_R = crate::FieldReader;
///Field `STOP` writer - stop bits
pub type STOP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LINEN` reader - LIN mode enable
pub type LINEN_R = crate::BitReader;
///Field `LINEN` writer - LIN mode enable
pub type LINEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWAP` reader - Swap TX/RX pins
pub type SWAP_R = crate::BitReader;
///Field `SWAP` writer - Swap TX/RX pins
pub type SWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXINV` reader - RX pin active level inversion
pub type RXINV_R = crate::BitReader;
///Field `RXINV` writer - RX pin active level inversion
pub type RXINV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXINV` reader - TX pin active level inversion
pub type TXINV_R = crate::BitReader;
///Field `TXINV` writer - TX pin active level inversion
pub type TXINV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATAINV` reader - Binary data inversion
pub type DATAINV_R = crate::BitReader;
///Field `DATAINV` writer - Binary data inversion
pub type DATAINV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSBFIRST` reader - Most significant bit first
pub type MSBFIRST_R = crate::BitReader;
///Field `MSBFIRST` writer - Most significant bit first
pub type MSBFIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ABREN` reader - Auto baud rate enable
pub type ABREN_R = crate::BitReader;
///Field `ABREN` writer - Auto baud rate enable
pub type ABREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ABRMOD` reader - Auto baud rate mode
pub type ABRMOD_R = crate::FieldReader;
///Field `ABRMOD` writer - Auto baud rate mode
pub type ABRMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RTOEN` reader - Receiver timeout enable
pub type RTOEN_R = crate::BitReader;
///Field `RTOEN` writer - Receiver timeout enable
pub type RTOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADD` reader - Address of the USART node
pub type ADD_R = crate::FieldReader;
///Field `ADD` writer - Address of the USART node
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Synchronous Slave mode enable
    #[inline(always)]
    pub fn slven(&self) -> SLVEN_R {
        SLVEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - When the DIS_NSS bit is set, the NSS pin input is ignored.
    #[inline(always)]
    pub fn dis_nss(&self) -> DIS_NSS_R {
        DIS_NSS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - 7-bit Address Detection/4-bit Address Detection
    #[inline(always)]
    pub fn addm7(&self) -> ADDM7_R {
        ADDM7_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - LIN break detection length
    #[inline(always)]
    pub fn lbdl(&self) -> LBDL_R {
        LBDL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LIN break detection interrupt enable
    #[inline(always)]
    pub fn lbdie(&self) -> LBDIE_R {
        LBDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Last bit clock pulse
    #[inline(always)]
    pub fn lbcl(&self) -> LBCL_R {
        LBCL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Clock phase
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Clock polarity
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Clock enable
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - stop bits
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - LIN mode enable
    #[inline(always)]
    pub fn linen(&self) -> LINEN_R {
        LINEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Swap TX/RX pins
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - RX pin active level inversion
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TX pin active level inversion
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Binary data inversion
    #[inline(always)]
    pub fn datainv(&self) -> DATAINV_R {
        DATAINV_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Most significant bit first
    #[inline(always)]
    pub fn msbfirst(&self) -> MSBFIRST_R {
        MSBFIRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Auto baud rate enable
    #[inline(always)]
    pub fn abren(&self) -> ABREN_R {
        ABREN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - Auto baud rate mode
    #[inline(always)]
    pub fn abrmod(&self) -> ABRMOD_R {
        ABRMOD_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 23 - Receiver timeout enable
    #[inline(always)]
    pub fn rtoen(&self) -> RTOEN_R {
        RTOEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:31 - Address of the USART node
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("slven", &self.slven())
            .field("dis_nss", &self.dis_nss())
            .field("addm7", &self.addm7())
            .field("lbdl", &self.lbdl())
            .field("lbdie", &self.lbdie())
            .field("lbcl", &self.lbcl())
            .field("cpha", &self.cpha())
            .field("cpol", &self.cpol())
            .field("clken", &self.clken())
            .field("stop", &self.stop())
            .field("linen", &self.linen())
            .field("swap", &self.swap())
            .field("rxinv", &self.rxinv())
            .field("txinv", &self.txinv())
            .field("datainv", &self.datainv())
            .field("msbfirst", &self.msbfirst())
            .field("abren", &self.abren())
            .field("abrmod", &self.abrmod())
            .field("rtoen", &self.rtoen())
            .field("add", &self.add())
            .finish()
    }
}
impl W {
    ///Bit 0 - Synchronous Slave mode enable
    #[inline(always)]
    pub fn slven(&mut self) -> SLVEN_W<'_, CR2rs> {
        SLVEN_W::new(self, 0)
    }
    ///Bit 3 - When the DIS_NSS bit is set, the NSS pin input is ignored.
    #[inline(always)]
    pub fn dis_nss(&mut self) -> DIS_NSS_W<'_, CR2rs> {
        DIS_NSS_W::new(self, 3)
    }
    ///Bit 4 - 7-bit Address Detection/4-bit Address Detection
    #[inline(always)]
    pub fn addm7(&mut self) -> ADDM7_W<'_, CR2rs> {
        ADDM7_W::new(self, 4)
    }
    ///Bit 5 - LIN break detection length
    #[inline(always)]
    pub fn lbdl(&mut self) -> LBDL_W<'_, CR2rs> {
        LBDL_W::new(self, 5)
    }
    ///Bit 6 - LIN break detection interrupt enable
    #[inline(always)]
    pub fn lbdie(&mut self) -> LBDIE_W<'_, CR2rs> {
        LBDIE_W::new(self, 6)
    }
    ///Bit 8 - Last bit clock pulse
    #[inline(always)]
    pub fn lbcl(&mut self) -> LBCL_W<'_, CR2rs> {
        LBCL_W::new(self, 8)
    }
    ///Bit 9 - Clock phase
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W<'_, CR2rs> {
        CPHA_W::new(self, 9)
    }
    ///Bit 10 - Clock polarity
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W<'_, CR2rs> {
        CPOL_W::new(self, 10)
    }
    ///Bit 11 - Clock enable
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W<'_, CR2rs> {
        CLKEN_W::new(self, 11)
    }
    ///Bits 12:13 - stop bits
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<'_, CR2rs> {
        STOP_W::new(self, 12)
    }
    ///Bit 14 - LIN mode enable
    #[inline(always)]
    pub fn linen(&mut self) -> LINEN_W<'_, CR2rs> {
        LINEN_W::new(self, 14)
    }
    ///Bit 15 - Swap TX/RX pins
    #[inline(always)]
    pub fn swap(&mut self) -> SWAP_W<'_, CR2rs> {
        SWAP_W::new(self, 15)
    }
    ///Bit 16 - RX pin active level inversion
    #[inline(always)]
    pub fn rxinv(&mut self) -> RXINV_W<'_, CR2rs> {
        RXINV_W::new(self, 16)
    }
    ///Bit 17 - TX pin active level inversion
    #[inline(always)]
    pub fn txinv(&mut self) -> TXINV_W<'_, CR2rs> {
        TXINV_W::new(self, 17)
    }
    ///Bit 18 - Binary data inversion
    #[inline(always)]
    pub fn datainv(&mut self) -> DATAINV_W<'_, CR2rs> {
        DATAINV_W::new(self, 18)
    }
    ///Bit 19 - Most significant bit first
    #[inline(always)]
    pub fn msbfirst(&mut self) -> MSBFIRST_W<'_, CR2rs> {
        MSBFIRST_W::new(self, 19)
    }
    ///Bit 20 - Auto baud rate enable
    #[inline(always)]
    pub fn abren(&mut self) -> ABREN_W<'_, CR2rs> {
        ABREN_W::new(self, 20)
    }
    ///Bits 21:22 - Auto baud rate mode
    #[inline(always)]
    pub fn abrmod(&mut self) -> ABRMOD_W<'_, CR2rs> {
        ABRMOD_W::new(self, 21)
    }
    ///Bit 23 - Receiver timeout enable
    #[inline(always)]
    pub fn rtoen(&mut self) -> RTOEN_W<'_, CR2rs> {
        RTOEN_W::new(self, 23)
    }
    ///Bits 24:31 - Address of the USART node
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W<'_, CR2rs> {
        ADD_W::new(self, 24)
    }
}
/**USART control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#USART1:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
