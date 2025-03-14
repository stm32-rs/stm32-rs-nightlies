///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `EN1` reader - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1.
pub type EN1_R = crate::BitReader;
///Field `EN1` writer - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1.
pub type EN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEN1` reader - DAC channel1 trigger enable
pub type TEN1_R = crate::BitReader;
///Field `TEN1` writer - DAC channel1 trigger enable
pub type TEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSEL10` reader - TSEL10
pub type TSEL10_R = crate::BitReader;
///Field `TSEL10` writer - TSEL10
pub type TSEL10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSEL11` reader - TSEL11
pub type TSEL11_R = crate::BitReader;
///Field `TSEL11` writer - TSEL11
pub type TSEL11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSEL12` reader - TSEL12
pub type TSEL12_R = crate::BitReader;
///Field `TSEL12` writer - TSEL12
pub type TSEL12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSEL13` reader - TSEL13
pub type TSEL13_R = crate::BitReader;
///Field `TSEL13` writer - TSEL13
pub type TSEL13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAVE1` reader - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled).
pub type WAVE1_R = crate::FieldReader;
///Field `WAVE1` writer - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled).
pub type WAVE1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MAMP1` reader - DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\[11:0\] of LFSR/ triangle amplitude equal to 4095
pub type MAMP1_R = crate::FieldReader;
///Field `MAMP1` writer - DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\[11:0\] of LFSR/ triangle amplitude equal to 4095
pub type MAMP1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DMAEN1` reader - DAC channel1 DMA enable This bit is set and cleared by software.
pub type DMAEN1_R = crate::BitReader;
///Field `DMAEN1` writer - DAC channel1 DMA enable This bit is set and cleared by software.
pub type DMAEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAUDRIE1` reader - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software.
pub type DMAUDRIE1_R = crate::BitReader;
///Field `DMAUDRIE1` writer - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software.
pub type DMAUDRIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEN1` reader - DAC Channel 1 calibration enable This bit is set and cleared by software to enable/disable DAC channel 1 calibration, it can be written only if bit EN1=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored.
pub type CEN1_R = crate::BitReader;
///Field `CEN1` writer - DAC Channel 1 calibration enable This bit is set and cleared by software to enable/disable DAC channel 1 calibration, it can be written only if bit EN1=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored.
pub type CEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HFSEL` reader - HFSEL
pub type HFSEL_R = crate::BitReader;
///Field `HFSEL` writer - HFSEL
pub type HFSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN2` reader - DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2.
pub type EN2_R = crate::BitReader;
///Field `EN2` writer - DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2.
pub type EN2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEN2` reader - DAC channel2 trigger enable
pub type TEN2_R = crate::BitReader;
///Field `TEN2` writer - DAC channel2 trigger enable
pub type TEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSEL20` reader - TSEL20
pub type TSEL20_R = crate::BitReader;
///Field `TSEL20` writer - TSEL20
pub type TSEL20_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSEL21` reader - TSEL21
pub type TSEL21_R = crate::BitReader;
///Field `TSEL21` writer - TSEL21
pub type TSEL21_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSEL22` reader - TSEL22
pub type TSEL22_R = crate::BitReader;
///Field `TSEL22` writer - TSEL22
pub type TSEL22_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSEL23` reader - TSEL23
pub type TSEL23_R = crate::BitReader;
///Field `TSEL23` writer - TSEL23
pub type TSEL23_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAVE2` reader - DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)
pub type WAVE2_R = crate::FieldReader;
///Field `WAVE2` writer - DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)
pub type WAVE2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MAMP2` reader - DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\[11:0\] of LFSR/ triangle amplitude equal to 4095
pub type MAMP2_R = crate::FieldReader;
///Field `MAMP2` writer - DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\[11:0\] of LFSR/ triangle amplitude equal to 4095
pub type MAMP2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DMAEN2` reader - DAC channel2 DMA enable This bit is set and cleared by software.
pub type DMAEN2_R = crate::BitReader;
///Field `DMAEN2` writer - DAC channel2 DMA enable This bit is set and cleared by software.
pub type DMAEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAUDRIE2` reader - DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software.
pub type DMAUDRIE2_R = crate::BitReader;
///Field `DMAUDRIE2` writer - DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software.
pub type DMAUDRIE2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEN2` reader - DAC Channel 2 calibration enable This bit is set and cleared by software to enable/disable DAC channel 2 calibration, it can be written only if bit EN2=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored.
pub type CEN2_R = crate::BitReader;
///Field `CEN2` writer - DAC Channel 2 calibration enable This bit is set and cleared by software to enable/disable DAC channel 2 calibration, it can be written only if bit EN2=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored.
pub type CEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1.
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DAC channel1 trigger enable
    #[inline(always)]
    pub fn ten1(&self) -> TEN1_R {
        TEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TSEL10
    #[inline(always)]
    pub fn tsel10(&self) -> TSEL10_R {
        TSEL10_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TSEL11
    #[inline(always)]
    pub fn tsel11(&self) -> TSEL11_R {
        TSEL11_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TSEL12
    #[inline(always)]
    pub fn tsel12(&self) -> TSEL12_R {
        TSEL12_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TSEL13
    #[inline(always)]
    pub fn tsel13(&self) -> TSEL13_R {
        TSEL13_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled).
    #[inline(always)]
    pub fn wave1(&self) -> WAVE1_R {
        WAVE1_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:11 - DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\[11:0\] of LFSR/ triangle amplitude equal to 4095
    #[inline(always)]
    pub fn mamp1(&self) -> MAMP1_R {
        MAMP1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - DAC channel1 DMA enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn dmaen1(&self) -> DMAEN1_R {
        DMAEN1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn dmaudrie1(&self) -> DMAUDRIE1_R {
        DMAUDRIE1_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - DAC Channel 1 calibration enable This bit is set and cleared by software to enable/disable DAC channel 1 calibration, it can be written only if bit EN1=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored.
    #[inline(always)]
    pub fn cen1(&self) -> CEN1_R {
        CEN1_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - HFSEL
    #[inline(always)]
    pub fn hfsel(&self) -> HFSEL_R {
        HFSEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2.
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DAC channel2 trigger enable
    #[inline(always)]
    pub fn ten2(&self) -> TEN2_R {
        TEN2_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TSEL20
    #[inline(always)]
    pub fn tsel20(&self) -> TSEL20_R {
        TSEL20_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - TSEL21
    #[inline(always)]
    pub fn tsel21(&self) -> TSEL21_R {
        TSEL21_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - TSEL22
    #[inline(always)]
    pub fn tsel22(&self) -> TSEL22_R {
        TSEL22_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - TSEL23
    #[inline(always)]
    pub fn tsel23(&self) -> TSEL23_R {
        TSEL23_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:23 - DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)
    #[inline(always)]
    pub fn wave2(&self) -> WAVE2_R {
        WAVE2_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:27 - DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\[11:0\] of LFSR/ triangle amplitude equal to 4095
    #[inline(always)]
    pub fn mamp2(&self) -> MAMP2_R {
        MAMP2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bit 28 - DAC channel2 DMA enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn dmaen2(&self) -> DMAEN2_R {
        DMAEN2_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn dmaudrie2(&self) -> DMAUDRIE2_R {
        DMAUDRIE2_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - DAC Channel 2 calibration enable This bit is set and cleared by software to enable/disable DAC channel 2 calibration, it can be written only if bit EN2=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored.
    #[inline(always)]
    pub fn cen2(&self) -> CEN2_R {
        CEN2_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("en1", &self.en1())
            .field("ten1", &self.ten1())
            .field("tsel10", &self.tsel10())
            .field("tsel11", &self.tsel11())
            .field("tsel12", &self.tsel12())
            .field("tsel13", &self.tsel13())
            .field("wave1", &self.wave1())
            .field("mamp1", &self.mamp1())
            .field("dmaen1", &self.dmaen1())
            .field("dmaudrie1", &self.dmaudrie1())
            .field("cen1", &self.cen1())
            .field("hfsel", &self.hfsel())
            .field("en2", &self.en2())
            .field("ten2", &self.ten2())
            .field("tsel20", &self.tsel20())
            .field("tsel21", &self.tsel21())
            .field("tsel22", &self.tsel22())
            .field("tsel23", &self.tsel23())
            .field("wave2", &self.wave2())
            .field("mamp2", &self.mamp2())
            .field("dmaen2", &self.dmaen2())
            .field("dmaudrie2", &self.dmaudrie2())
            .field("cen2", &self.cen2())
            .finish()
    }
}
impl W {
    ///Bit 0 - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1.
    #[inline(always)]
    pub fn en1(&mut self) -> EN1_W<CRrs> {
        EN1_W::new(self, 0)
    }
    ///Bit 1 - DAC channel1 trigger enable
    #[inline(always)]
    pub fn ten1(&mut self) -> TEN1_W<CRrs> {
        TEN1_W::new(self, 1)
    }
    ///Bit 2 - TSEL10
    #[inline(always)]
    pub fn tsel10(&mut self) -> TSEL10_W<CRrs> {
        TSEL10_W::new(self, 2)
    }
    ///Bit 3 - TSEL11
    #[inline(always)]
    pub fn tsel11(&mut self) -> TSEL11_W<CRrs> {
        TSEL11_W::new(self, 3)
    }
    ///Bit 4 - TSEL12
    #[inline(always)]
    pub fn tsel12(&mut self) -> TSEL12_W<CRrs> {
        TSEL12_W::new(self, 4)
    }
    ///Bit 5 - TSEL13
    #[inline(always)]
    pub fn tsel13(&mut self) -> TSEL13_W<CRrs> {
        TSEL13_W::new(self, 5)
    }
    ///Bits 6:7 - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled).
    #[inline(always)]
    pub fn wave1(&mut self) -> WAVE1_W<CRrs> {
        WAVE1_W::new(self, 6)
    }
    ///Bits 8:11 - DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\[11:0\] of LFSR/ triangle amplitude equal to 4095
    #[inline(always)]
    pub fn mamp1(&mut self) -> MAMP1_W<CRrs> {
        MAMP1_W::new(self, 8)
    }
    ///Bit 12 - DAC channel1 DMA enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn dmaen1(&mut self) -> DMAEN1_W<CRrs> {
        DMAEN1_W::new(self, 12)
    }
    ///Bit 13 - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn dmaudrie1(&mut self) -> DMAUDRIE1_W<CRrs> {
        DMAUDRIE1_W::new(self, 13)
    }
    ///Bit 14 - DAC Channel 1 calibration enable This bit is set and cleared by software to enable/disable DAC channel 1 calibration, it can be written only if bit EN1=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored.
    #[inline(always)]
    pub fn cen1(&mut self) -> CEN1_W<CRrs> {
        CEN1_W::new(self, 14)
    }
    ///Bit 15 - HFSEL
    #[inline(always)]
    pub fn hfsel(&mut self) -> HFSEL_W<CRrs> {
        HFSEL_W::new(self, 15)
    }
    ///Bit 16 - DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2.
    #[inline(always)]
    pub fn en2(&mut self) -> EN2_W<CRrs> {
        EN2_W::new(self, 16)
    }
    ///Bit 17 - DAC channel2 trigger enable
    #[inline(always)]
    pub fn ten2(&mut self) -> TEN2_W<CRrs> {
        TEN2_W::new(self, 17)
    }
    ///Bit 18 - TSEL20
    #[inline(always)]
    pub fn tsel20(&mut self) -> TSEL20_W<CRrs> {
        TSEL20_W::new(self, 18)
    }
    ///Bit 19 - TSEL21
    #[inline(always)]
    pub fn tsel21(&mut self) -> TSEL21_W<CRrs> {
        TSEL21_W::new(self, 19)
    }
    ///Bit 20 - TSEL22
    #[inline(always)]
    pub fn tsel22(&mut self) -> TSEL22_W<CRrs> {
        TSEL22_W::new(self, 20)
    }
    ///Bit 21 - TSEL23
    #[inline(always)]
    pub fn tsel23(&mut self) -> TSEL23_W<CRrs> {
        TSEL23_W::new(self, 21)
    }
    ///Bits 22:23 - DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)
    #[inline(always)]
    pub fn wave2(&mut self) -> WAVE2_W<CRrs> {
        WAVE2_W::new(self, 22)
    }
    ///Bits 24:27 - DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\[11:0\] of LFSR/ triangle amplitude equal to 4095
    #[inline(always)]
    pub fn mamp2(&mut self) -> MAMP2_W<CRrs> {
        MAMP2_W::new(self, 24)
    }
    ///Bit 28 - DAC channel2 DMA enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn dmaen2(&mut self) -> DMAEN2_W<CRrs> {
        DMAEN2_W::new(self, 28)
    }
    ///Bit 29 - DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn dmaudrie2(&mut self) -> DMAUDRIE2_W<CRrs> {
        DMAUDRIE2_W::new(self, 29)
    }
    ///Bit 30 - DAC Channel 2 calibration enable This bit is set and cleared by software to enable/disable DAC channel 2 calibration, it can be written only if bit EN2=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored.
    #[inline(always)]
    pub fn cen2(&mut self) -> CEN2_W<CRrs> {
        CEN2_W::new(self, 30)
    }
}
/**DAC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#DAC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
