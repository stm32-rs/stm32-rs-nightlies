#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Field `SLVEN` reader - SLVEN"]
pub type SLVEN_R = crate::BitReader;
#[doc = "Field `SLVEN` writer - SLVEN"]
pub type SLVEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_NSS` reader - DIS_NSS"]
pub type DIS_NSS_R = crate::BitReader;
#[doc = "Field `DIS_NSS` writer - DIS_NSS"]
pub type DIS_NSS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDM7` reader - 7-bit Address Detection/4-bit Address Detection"]
pub type ADDM7_R = crate::BitReader;
#[doc = "Field `ADDM7` writer - 7-bit Address Detection/4-bit Address Detection"]
pub type ADDM7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDL` reader - LIN break detection length"]
pub type LBDL_R = crate::BitReader;
#[doc = "Field `LBDL` writer - LIN break detection length"]
pub type LBDL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDIE` reader - LIN break detection interrupt enable"]
pub type LBDIE_R = crate::BitReader;
#[doc = "Field `LBDIE` writer - LIN break detection interrupt enable"]
pub type LBDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBCL` reader - Last bit clock pulse"]
pub type LBCL_R = crate::BitReader;
#[doc = "Field `LBCL` writer - Last bit clock pulse"]
pub type LBCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHA` reader - Clock phase"]
pub type CPHA_R = crate::BitReader;
#[doc = "Field `CPHA` writer - Clock phase"]
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Clock polarity"]
pub type CPOL_R = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock polarity"]
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKEN` reader - Clock enable"]
pub type CLKEN_R = crate::BitReader;
#[doc = "Field `CLKEN` writer - Clock enable"]
pub type CLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - STOP bits"]
pub type STOP_R = crate::FieldReader;
#[doc = "Field `STOP` writer - STOP bits"]
pub type STOP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LINEN` reader - LIN mode enable"]
pub type LINEN_R = crate::BitReader;
#[doc = "Field `LINEN` writer - LIN mode enable"]
pub type LINEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP` reader - Swap TX/RX pins"]
pub type SWAP_R = crate::BitReader;
#[doc = "Field `SWAP` writer - Swap TX/RX pins"]
pub type SWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINV` reader - RX pin active level inversion"]
pub type RXINV_R = crate::BitReader;
#[doc = "Field `RXINV` writer - RX pin active level inversion"]
pub type RXINV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXINV` reader - TX pin active level inversion"]
pub type TXINV_R = crate::BitReader;
#[doc = "Field `TXINV` writer - TX pin active level inversion"]
pub type TXINV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAINV` reader - Binary data inversion"]
pub type TAINV_R = crate::BitReader;
#[doc = "Field `TAINV` writer - Binary data inversion"]
pub type TAINV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSBFIRST` reader - Most significant bit first"]
pub type MSBFIRST_R = crate::BitReader;
#[doc = "Field `MSBFIRST` writer - Most significant bit first"]
pub type MSBFIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABREN` reader - Auto baud rate enable"]
pub type ABREN_R = crate::BitReader;
#[doc = "Field `ABREN` writer - Auto baud rate enable"]
pub type ABREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABRMOD0` reader - ABRMOD0"]
pub type ABRMOD0_R = crate::BitReader;
#[doc = "Field `ABRMOD0` writer - ABRMOD0"]
pub type ABRMOD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABRMOD1` reader - Auto baud rate mode"]
pub type ABRMOD1_R = crate::BitReader;
#[doc = "Field `ABRMOD1` writer - Auto baud rate mode"]
pub type ABRMOD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTOEN` reader - Receiver timeout enable"]
pub type RTOEN_R = crate::BitReader;
#[doc = "Field `RTOEN` writer - Receiver timeout enable"]
pub type RTOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD0_3` reader - Address of the USART node"]
pub type ADD0_3_R = crate::FieldReader;
#[doc = "Field `ADD0_3` writer - Address of the USART node"]
pub type ADD0_3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADD4_7` reader - Address of the USART node"]
pub type ADD4_7_R = crate::FieldReader;
#[doc = "Field `ADD4_7` writer - Address of the USART node"]
pub type ADD4_7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - SLVEN"]
    #[inline(always)]
    pub fn slven(&self) -> SLVEN_R {
        SLVEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - DIS_NSS"]
    #[inline(always)]
    pub fn dis_nss(&self) -> DIS_NSS_R {
        DIS_NSS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    pub fn addm7(&self) -> ADDM7_R {
        ADDM7_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LIN break detection length"]
    #[inline(always)]
    pub fn lbdl(&self) -> LBDL_R {
        LBDL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdie(&self) -> LBDIE_R {
        LBDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    pub fn lbcl(&self) -> LBCL_R {
        LBCL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn linen(&self) -> LINEN_R {
        LINEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline(always)]
    pub fn tainv(&self) -> TAINV_R {
        TAINV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    pub fn msbfirst(&self) -> MSBFIRST_R {
        MSBFIRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Auto baud rate enable"]
    #[inline(always)]
    pub fn abren(&self) -> ABREN_R {
        ABREN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ABRMOD0"]
    #[inline(always)]
    pub fn abrmod0(&self) -> ABRMOD0_R {
        ABRMOD0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Auto baud rate mode"]
    #[inline(always)]
    pub fn abrmod1(&self) -> ABRMOD1_R {
        ABRMOD1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Receiver timeout enable"]
    #[inline(always)]
    pub fn rtoen(&self) -> RTOEN_R {
        RTOEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Address of the USART node"]
    #[inline(always)]
    pub fn add0_3(&self) -> ADD0_3_R {
        ADD0_3_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Address of the USART node"]
    #[inline(always)]
    pub fn add4_7(&self) -> ADD4_7_R {
        ADD4_7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SLVEN"]
    #[inline(always)]
    #[must_use]
    pub fn slven(&mut self) -> SLVEN_W<CR2rs> {
        SLVEN_W::new(self, 0)
    }
    #[doc = "Bit 3 - DIS_NSS"]
    #[inline(always)]
    #[must_use]
    pub fn dis_nss(&mut self) -> DIS_NSS_W<CR2rs> {
        DIS_NSS_W::new(self, 3)
    }
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    #[must_use]
    pub fn addm7(&mut self) -> ADDM7_W<CR2rs> {
        ADDM7_W::new(self, 4)
    }
    #[doc = "Bit 5 - LIN break detection length"]
    #[inline(always)]
    #[must_use]
    pub fn lbdl(&mut self) -> LBDL_W<CR2rs> {
        LBDL_W::new(self, 5)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lbdie(&mut self) -> LBDIE_W<CR2rs> {
        LBDIE_W::new(self, 6)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    #[must_use]
    pub fn lbcl(&mut self) -> LBCL_W<CR2rs> {
        LBCL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<CR2rs> {
        CPHA_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<CR2rs> {
        CPOL_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<CR2rs> {
        CLKEN_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CR2rs> {
        STOP_W::new(self, 12)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn linen(&mut self) -> LINEN_W<CR2rs> {
        LINEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SWAP_W<CR2rs> {
        SWAP_W::new(self, 15)
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RXINV_W<CR2rs> {
        RXINV_W::new(self, 16)
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline(always)]
    #[must_use]
    pub fn txinv(&mut self) -> TXINV_W<CR2rs> {
        TXINV_W::new(self, 17)
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline(always)]
    #[must_use]
    pub fn tainv(&mut self) -> TAINV_W<CR2rs> {
        TAINV_W::new(self, 18)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    #[must_use]
    pub fn msbfirst(&mut self) -> MSBFIRST_W<CR2rs> {
        MSBFIRST_W::new(self, 19)
    }
    #[doc = "Bit 20 - Auto baud rate enable"]
    #[inline(always)]
    #[must_use]
    pub fn abren(&mut self) -> ABREN_W<CR2rs> {
        ABREN_W::new(self, 20)
    }
    #[doc = "Bit 21 - ABRMOD0"]
    #[inline(always)]
    #[must_use]
    pub fn abrmod0(&mut self) -> ABRMOD0_W<CR2rs> {
        ABRMOD0_W::new(self, 21)
    }
    #[doc = "Bit 22 - Auto baud rate mode"]
    #[inline(always)]
    #[must_use]
    pub fn abrmod1(&mut self) -> ABRMOD1_W<CR2rs> {
        ABRMOD1_W::new(self, 22)
    }
    #[doc = "Bit 23 - Receiver timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtoen(&mut self) -> RTOEN_W<CR2rs> {
        RTOEN_W::new(self, 23)
    }
    #[doc = "Bits 24:27 - Address of the USART node"]
    #[inline(always)]
    #[must_use]
    pub fn add0_3(&mut self) -> ADD0_3_W<CR2rs> {
        ADD0_3_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Address of the USART node"]
    #[inline(always)]
    #[must_use]
    pub fn add4_7(&mut self) -> ADD4_7_W<CR2rs> {
        ADD4_7_W::new(self, 28)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
