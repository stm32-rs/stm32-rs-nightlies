///Register `C2IMR2` reader
pub type R = crate::R<C2IMR2rs>;
///Register `C2IMR2` writer
pub type W = crate::W<C2IMR2rs>;
/**Peripheral DMA1 CH1 interrupt mask to CPU2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1_CH1_IM {
    ///0: Peripheral interrupt forwarded to CPU2
    Unmasked = 0,
    ///1: Peripheral interrupt to CPU2 masked
    Masked = 1,
}
impl From<DMA1_CH1_IM> for bool {
    #[inline(always)]
    fn from(variant: DMA1_CH1_IM) -> Self {
        variant as u8 != 0
    }
}
///Field `DMA1_CH1_IM` reader - Peripheral DMA1 CH1 interrupt mask to CPU2
pub type DMA1_CH1_IM_R = crate::BitReader<DMA1_CH1_IM>;
impl DMA1_CH1_IM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMA1_CH1_IM {
        match self.bits {
            false => DMA1_CH1_IM::Unmasked,
            true => DMA1_CH1_IM::Masked,
        }
    }
    ///Peripheral interrupt forwarded to CPU2
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == DMA1_CH1_IM::Unmasked
    }
    ///Peripheral interrupt to CPU2 masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == DMA1_CH1_IM::Masked
    }
}
///Field `DMA1_CH1_IM` writer - Peripheral DMA1 CH1 interrupt mask to CPU2
pub type DMA1_CH1_IM_W<'a, REG> = crate::BitWriter<'a, REG, DMA1_CH1_IM>;
impl<'a, REG> DMA1_CH1_IM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral interrupt forwarded to CPU2
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1_CH1_IM::Unmasked)
    }
    ///Peripheral interrupt to CPU2 masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1_CH1_IM::Masked)
    }
}
///Field `DMA1_CH2_IM` reader - Peripheral DMA1 CH2 interrupt mask to CPU2
pub use DMA1_CH1_IM_R as DMA1_CH2_IM_R;
///Field `DMA1_CH3_IM` reader - Peripheral DMA1 CH3 interrupt mask to CPU2
pub use DMA1_CH1_IM_R as DMA1_CH3_IM_R;
///Field `DMA1_CH4_IM` reader - Peripheral DMA1 CH4 interrupt mask to CPU2
pub use DMA1_CH1_IM_R as DMA1_CH4_IM_R;
///Field `DMA1_CH5_IM` reader - Peripheral DMA1 CH5 interrupt mask to CPU2
pub use DMA1_CH1_IM_R as DMA1_CH5_IM_R;
///Field `DMA1_CH6_IM` reader - Peripheral DMA1 CH6 interrupt mask to CPU2
pub use DMA1_CH1_IM_R as DMA1_CH6_IM_R;
///Field `DMA1_CH7_IM` reader - Peripheral DMA1 CH7 interrupt mask to CPU2
pub use DMA1_CH1_IM_R as DMA1_CH7_IM_R;
///Field `DMA2_CH1_IM` reader - Peripheral DMA2 CH1 interrupt mask to CPU1
pub use DMA1_CH1_IM_R as DMA2_CH1_IM_R;
///Field `DMA2_CH2_IM` reader - Peripheral DMA2 CH2 interrupt mask to CPU1
pub use DMA1_CH1_IM_R as DMA2_CH2_IM_R;
///Field `DMA2_CH3_IM` reader - Peripheral DMA2 CH3 interrupt mask to CPU1
pub use DMA1_CH1_IM_R as DMA2_CH3_IM_R;
///Field `DMA2_CH4_IM` reader - Peripheral DMA2 CH4 interrupt mask to CPU1
pub use DMA1_CH1_IM_R as DMA2_CH4_IM_R;
///Field `DMA2_CH5_IM` reader - Peripheral DMA2 CH5 interrupt mask to CPU1
pub use DMA1_CH1_IM_R as DMA2_CH5_IM_R;
///Field `DMA2_CH6_IM` reader - Peripheral DMA2 CH6 interrupt mask to CPU1
pub use DMA1_CH1_IM_R as DMA2_CH6_IM_R;
///Field `DMA2_CH7_IM` reader - Peripheral DMA2 CH7 interrupt mask to CPU1
pub use DMA1_CH1_IM_R as DMA2_CH7_IM_R;
///Field `DMAM_UX1_IM` reader - Peripheral DMAM UX1 interrupt mask to CPU1
pub use DMA1_CH1_IM_R as DMAM_UX1_IM_R;
///Field `PVM1IM` reader - Peripheral PVM1IM interrupt mask to CPU1
pub use DMA1_CH1_IM_R as PVM1IM_R;
///Field `PVM3IM` reader - Peripheral PVM3IM interrupt mask to CPU1
pub use DMA1_CH1_IM_R as PVM3IM_R;
///Field `PVDIM` reader - Peripheral PVDIM interrupt mask to CPU1
pub use DMA1_CH1_IM_R as PVDIM_R;
///Field `TSCIM` reader - Peripheral TSCIM interrupt mask to CPU1
pub use DMA1_CH1_IM_R as TSCIM_R;
///Field `LCDIM` reader - Peripheral LCDIM interrupt mask to CPU1
pub use DMA1_CH1_IM_R as LCDIM_R;
///Field `DMA1_CH2_IM` writer - Peripheral DMA1 CH2 interrupt mask to CPU2
pub use DMA1_CH1_IM_W as DMA1_CH2_IM_W;
///Field `DMA1_CH3_IM` writer - Peripheral DMA1 CH3 interrupt mask to CPU2
pub use DMA1_CH1_IM_W as DMA1_CH3_IM_W;
///Field `DMA1_CH4_IM` writer - Peripheral DMA1 CH4 interrupt mask to CPU2
pub use DMA1_CH1_IM_W as DMA1_CH4_IM_W;
///Field `DMA1_CH5_IM` writer - Peripheral DMA1 CH5 interrupt mask to CPU2
pub use DMA1_CH1_IM_W as DMA1_CH5_IM_W;
///Field `DMA1_CH6_IM` writer - Peripheral DMA1 CH6 interrupt mask to CPU2
pub use DMA1_CH1_IM_W as DMA1_CH6_IM_W;
///Field `DMA1_CH7_IM` writer - Peripheral DMA1 CH7 interrupt mask to CPU2
pub use DMA1_CH1_IM_W as DMA1_CH7_IM_W;
///Field `DMA2_CH1_IM` writer - Peripheral DMA2 CH1 interrupt mask to CPU1
pub use DMA1_CH1_IM_W as DMA2_CH1_IM_W;
///Field `DMA2_CH2_IM` writer - Peripheral DMA2 CH2 interrupt mask to CPU1
pub use DMA1_CH1_IM_W as DMA2_CH2_IM_W;
///Field `DMA2_CH3_IM` writer - Peripheral DMA2 CH3 interrupt mask to CPU1
pub use DMA1_CH1_IM_W as DMA2_CH3_IM_W;
///Field `DMA2_CH4_IM` writer - Peripheral DMA2 CH4 interrupt mask to CPU1
pub use DMA1_CH1_IM_W as DMA2_CH4_IM_W;
///Field `DMA2_CH5_IM` writer - Peripheral DMA2 CH5 interrupt mask to CPU1
pub use DMA1_CH1_IM_W as DMA2_CH5_IM_W;
///Field `DMA2_CH6_IM` writer - Peripheral DMA2 CH6 interrupt mask to CPU1
pub use DMA1_CH1_IM_W as DMA2_CH6_IM_W;
///Field `DMA2_CH7_IM` writer - Peripheral DMA2 CH7 interrupt mask to CPU1
pub use DMA1_CH1_IM_W as DMA2_CH7_IM_W;
///Field `DMAM_UX1_IM` writer - Peripheral DMAM UX1 interrupt mask to CPU1
pub use DMA1_CH1_IM_W as DMAM_UX1_IM_W;
///Field `PVM1IM` writer - Peripheral PVM1IM interrupt mask to CPU1
pub use DMA1_CH1_IM_W as PVM1IM_W;
///Field `PVM3IM` writer - Peripheral PVM3IM interrupt mask to CPU1
pub use DMA1_CH1_IM_W as PVM3IM_W;
///Field `PVDIM` writer - Peripheral PVDIM interrupt mask to CPU1
pub use DMA1_CH1_IM_W as PVDIM_W;
///Field `TSCIM` writer - Peripheral TSCIM interrupt mask to CPU1
pub use DMA1_CH1_IM_W as TSCIM_W;
///Field `LCDIM` writer - Peripheral LCDIM interrupt mask to CPU1
pub use DMA1_CH1_IM_W as LCDIM_W;
impl R {
    ///Bit 0 - Peripheral DMA1 CH1 interrupt mask to CPU2
    #[inline(always)]
    pub fn dma1_ch1_im(&self) -> DMA1_CH1_IM_R {
        DMA1_CH1_IM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Peripheral DMA1 CH2 interrupt mask to CPU2
    #[inline(always)]
    pub fn dma1_ch2_im(&self) -> DMA1_CH2_IM_R {
        DMA1_CH2_IM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Peripheral DMA1 CH3 interrupt mask to CPU2
    #[inline(always)]
    pub fn dma1_ch3_im(&self) -> DMA1_CH3_IM_R {
        DMA1_CH3_IM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Peripheral DMA1 CH4 interrupt mask to CPU2
    #[inline(always)]
    pub fn dma1_ch4_im(&self) -> DMA1_CH4_IM_R {
        DMA1_CH4_IM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Peripheral DMA1 CH5 interrupt mask to CPU2
    #[inline(always)]
    pub fn dma1_ch5_im(&self) -> DMA1_CH5_IM_R {
        DMA1_CH5_IM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Peripheral DMA1 CH6 interrupt mask to CPU2
    #[inline(always)]
    pub fn dma1_ch6_im(&self) -> DMA1_CH6_IM_R {
        DMA1_CH6_IM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Peripheral DMA1 CH7 interrupt mask to CPU2
    #[inline(always)]
    pub fn dma1_ch7_im(&self) -> DMA1_CH7_IM_R {
        DMA1_CH7_IM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Peripheral DMA2 CH1 interrupt mask to CPU1
    #[inline(always)]
    pub fn dma2_ch1_im(&self) -> DMA2_CH1_IM_R {
        DMA2_CH1_IM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Peripheral DMA2 CH2 interrupt mask to CPU1
    #[inline(always)]
    pub fn dma2_ch2_im(&self) -> DMA2_CH2_IM_R {
        DMA2_CH2_IM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Peripheral DMA2 CH3 interrupt mask to CPU1
    #[inline(always)]
    pub fn dma2_ch3_im(&self) -> DMA2_CH3_IM_R {
        DMA2_CH3_IM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Peripheral DMA2 CH4 interrupt mask to CPU1
    #[inline(always)]
    pub fn dma2_ch4_im(&self) -> DMA2_CH4_IM_R {
        DMA2_CH4_IM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Peripheral DMA2 CH5 interrupt mask to CPU1
    #[inline(always)]
    pub fn dma2_ch5_im(&self) -> DMA2_CH5_IM_R {
        DMA2_CH5_IM_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Peripheral DMA2 CH6 interrupt mask to CPU1
    #[inline(always)]
    pub fn dma2_ch6_im(&self) -> DMA2_CH6_IM_R {
        DMA2_CH6_IM_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Peripheral DMA2 CH7 interrupt mask to CPU1
    #[inline(always)]
    pub fn dma2_ch7_im(&self) -> DMA2_CH7_IM_R {
        DMA2_CH7_IM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Peripheral DMAM UX1 interrupt mask to CPU1
    #[inline(always)]
    pub fn dmam_ux1_im(&self) -> DMAM_UX1_IM_R {
        DMAM_UX1_IM_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Peripheral PVM1IM interrupt mask to CPU1
    #[inline(always)]
    pub fn pvm1im(&self) -> PVM1IM_R {
        PVM1IM_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Peripheral PVM3IM interrupt mask to CPU1
    #[inline(always)]
    pub fn pvm3im(&self) -> PVM3IM_R {
        PVM3IM_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - Peripheral PVDIM interrupt mask to CPU1
    #[inline(always)]
    pub fn pvdim(&self) -> PVDIM_R {
        PVDIM_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Peripheral TSCIM interrupt mask to CPU1
    #[inline(always)]
    pub fn tscim(&self) -> TSCIM_R {
        TSCIM_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Peripheral LCDIM interrupt mask to CPU1
    #[inline(always)]
    pub fn lcdim(&self) -> LCDIM_R {
        LCDIM_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2IMR2")
            .field("dma1_ch1_im", &self.dma1_ch1_im())
            .field("dma1_ch2_im", &self.dma1_ch2_im())
            .field("dma1_ch3_im", &self.dma1_ch3_im())
            .field("dma1_ch4_im", &self.dma1_ch4_im())
            .field("dma1_ch5_im", &self.dma1_ch5_im())
            .field("dma1_ch6_im", &self.dma1_ch6_im())
            .field("dma1_ch7_im", &self.dma1_ch7_im())
            .field("dma2_ch1_im", &self.dma2_ch1_im())
            .field("dma2_ch2_im", &self.dma2_ch2_im())
            .field("dma2_ch3_im", &self.dma2_ch3_im())
            .field("dma2_ch4_im", &self.dma2_ch4_im())
            .field("dma2_ch5_im", &self.dma2_ch5_im())
            .field("dma2_ch6_im", &self.dma2_ch6_im())
            .field("dma2_ch7_im", &self.dma2_ch7_im())
            .field("dmam_ux1_im", &self.dmam_ux1_im())
            .field("pvm1im", &self.pvm1im())
            .field("pvm3im", &self.pvm3im())
            .field("pvdim", &self.pvdim())
            .field("tscim", &self.tscim())
            .field("lcdim", &self.lcdim())
            .finish()
    }
}
impl W {
    ///Bit 0 - Peripheral DMA1 CH1 interrupt mask to CPU2
    #[inline(always)]
    pub fn dma1_ch1_im(&mut self) -> DMA1_CH1_IM_W<'_, C2IMR2rs> {
        DMA1_CH1_IM_W::new(self, 0)
    }
    ///Bit 1 - Peripheral DMA1 CH2 interrupt mask to CPU2
    #[inline(always)]
    pub fn dma1_ch2_im(&mut self) -> DMA1_CH2_IM_W<'_, C2IMR2rs> {
        DMA1_CH2_IM_W::new(self, 1)
    }
    ///Bit 2 - Peripheral DMA1 CH3 interrupt mask to CPU2
    #[inline(always)]
    pub fn dma1_ch3_im(&mut self) -> DMA1_CH3_IM_W<'_, C2IMR2rs> {
        DMA1_CH3_IM_W::new(self, 2)
    }
    ///Bit 3 - Peripheral DMA1 CH4 interrupt mask to CPU2
    #[inline(always)]
    pub fn dma1_ch4_im(&mut self) -> DMA1_CH4_IM_W<'_, C2IMR2rs> {
        DMA1_CH4_IM_W::new(self, 3)
    }
    ///Bit 4 - Peripheral DMA1 CH5 interrupt mask to CPU2
    #[inline(always)]
    pub fn dma1_ch5_im(&mut self) -> DMA1_CH5_IM_W<'_, C2IMR2rs> {
        DMA1_CH5_IM_W::new(self, 4)
    }
    ///Bit 5 - Peripheral DMA1 CH6 interrupt mask to CPU2
    #[inline(always)]
    pub fn dma1_ch6_im(&mut self) -> DMA1_CH6_IM_W<'_, C2IMR2rs> {
        DMA1_CH6_IM_W::new(self, 5)
    }
    ///Bit 6 - Peripheral DMA1 CH7 interrupt mask to CPU2
    #[inline(always)]
    pub fn dma1_ch7_im(&mut self) -> DMA1_CH7_IM_W<'_, C2IMR2rs> {
        DMA1_CH7_IM_W::new(self, 6)
    }
    ///Bit 8 - Peripheral DMA2 CH1 interrupt mask to CPU1
    #[inline(always)]
    pub fn dma2_ch1_im(&mut self) -> DMA2_CH1_IM_W<'_, C2IMR2rs> {
        DMA2_CH1_IM_W::new(self, 8)
    }
    ///Bit 9 - Peripheral DMA2 CH2 interrupt mask to CPU1
    #[inline(always)]
    pub fn dma2_ch2_im(&mut self) -> DMA2_CH2_IM_W<'_, C2IMR2rs> {
        DMA2_CH2_IM_W::new(self, 9)
    }
    ///Bit 10 - Peripheral DMA2 CH3 interrupt mask to CPU1
    #[inline(always)]
    pub fn dma2_ch3_im(&mut self) -> DMA2_CH3_IM_W<'_, C2IMR2rs> {
        DMA2_CH3_IM_W::new(self, 10)
    }
    ///Bit 11 - Peripheral DMA2 CH4 interrupt mask to CPU1
    #[inline(always)]
    pub fn dma2_ch4_im(&mut self) -> DMA2_CH4_IM_W<'_, C2IMR2rs> {
        DMA2_CH4_IM_W::new(self, 11)
    }
    ///Bit 12 - Peripheral DMA2 CH5 interrupt mask to CPU1
    #[inline(always)]
    pub fn dma2_ch5_im(&mut self) -> DMA2_CH5_IM_W<'_, C2IMR2rs> {
        DMA2_CH5_IM_W::new(self, 12)
    }
    ///Bit 13 - Peripheral DMA2 CH6 interrupt mask to CPU1
    #[inline(always)]
    pub fn dma2_ch6_im(&mut self) -> DMA2_CH6_IM_W<'_, C2IMR2rs> {
        DMA2_CH6_IM_W::new(self, 13)
    }
    ///Bit 14 - Peripheral DMA2 CH7 interrupt mask to CPU1
    #[inline(always)]
    pub fn dma2_ch7_im(&mut self) -> DMA2_CH7_IM_W<'_, C2IMR2rs> {
        DMA2_CH7_IM_W::new(self, 14)
    }
    ///Bit 15 - Peripheral DMAM UX1 interrupt mask to CPU1
    #[inline(always)]
    pub fn dmam_ux1_im(&mut self) -> DMAM_UX1_IM_W<'_, C2IMR2rs> {
        DMAM_UX1_IM_W::new(self, 15)
    }
    ///Bit 16 - Peripheral PVM1IM interrupt mask to CPU1
    #[inline(always)]
    pub fn pvm1im(&mut self) -> PVM1IM_W<'_, C2IMR2rs> {
        PVM1IM_W::new(self, 16)
    }
    ///Bit 18 - Peripheral PVM3IM interrupt mask to CPU1
    #[inline(always)]
    pub fn pvm3im(&mut self) -> PVM3IM_W<'_, C2IMR2rs> {
        PVM3IM_W::new(self, 18)
    }
    ///Bit 20 - Peripheral PVDIM interrupt mask to CPU1
    #[inline(always)]
    pub fn pvdim(&mut self) -> PVDIM_W<'_, C2IMR2rs> {
        PVDIM_W::new(self, 20)
    }
    ///Bit 21 - Peripheral TSCIM interrupt mask to CPU1
    #[inline(always)]
    pub fn tscim(&mut self) -> TSCIM_W<'_, C2IMR2rs> {
        TSCIM_W::new(self, 21)
    }
    ///Bit 22 - Peripheral LCDIM interrupt mask to CPU1
    #[inline(always)]
    pub fn lcdim(&mut self) -> LCDIM_W<'_, C2IMR2rs> {
        LCDIM_W::new(self, 22)
    }
}
/**CPU2 interrupt mask register 1

You can [`read`](crate::Reg::read) this register and get [`c2imr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2imr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:C2IMR2)*/
pub struct C2IMR2rs;
impl crate::RegisterSpec for C2IMR2rs {
    type Ux = u32;
}
///`read()` method returns [`c2imr2::R`](R) reader structure
impl crate::Readable for C2IMR2rs {}
///`write(|w| ..)` method takes [`c2imr2::W`](W) writer structure
impl crate::Writable for C2IMR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2IMR2 to value 0
impl crate::Resettable for C2IMR2rs {}
